use makepad_widgets::*;
use moly_kit::{
    acp::{AcpAgentConfig, AcpBotClient, StdioAcpConnection},
    protocol::*,
    utils::asynchronous::spawn,
    ChatTask, ChatWidgetRefExt,
};
use std::path::PathBuf;
use std::sync::Arc;

use ::log as log_crate;

live_design! {
    use link::widgets::*;
    use moly_kit::widgets::chat::Chat;

    // Agent Panel Header with connect button
    AgentPanelHeader = <View> {
        width: Fill,
        height: Fit,
        flow: Right,
        padding: 10,
        spacing: 10,
        align: { y: 0.5 },
        show_bg: true,
        draw_bg: {
            color: #f0f0f0
        }

        agent_label = <Label> {
            text: "Agent",
            draw_text: {
                text_style: { font_size: 12.0 },
                color: #333
            }
        }

        status_label = <Label> {
            text: "Disconnected",
            draw_text: {
                text_style: { font_size: 10.0 },
                color: #888
            }
        }

        <Filler> {}

        connect_button = <Button> {
            text: "Connect",
            width: Fit,
            height: Fit,
            padding: { left: 8, right: 8, top: 4, bottom: 4 },
        }
    }

    // Single Agent Chat Panel
    AgentChatPanel = <View> {
        width: Fill,
        height: Fill,
        flow: Down,
        show_bg: true,
        draw_bg: {
            color: #fafafa
        }

        header = <AgentPanelHeader> {}

        chat = <Chat> {
            width: Fill,
            height: Fill,
            padding: 10,
            draw_bg: {
                border_radius: 0.0,
                color: #fff
            }
        }

        forward_bar = <View> {
            width: Fill,
            height: Fit,
            padding: 10,
            align: { x: 0.5 },
            visible: false,

            forward_button = <Button> {
                text: "→ Forward",
                width: Fit,
                height: Fit,
                padding: { left: 6, right: 6, top: 3, bottom: 3 },
                draw_bg: {
                    color: #e0e0e0,
                    color_down: #c0c0c0,
                },
                draw_text: {
                    text_style: { font_size: 9.0 },
                    color: #555
                }
            }
        }
    }

    // Working directory input (shared)
    WorkingDirBar = <View> {
        width: Fill,
        height: Fit,
        flow: Right,
        padding: 10,
        spacing: 10,
        align: { y: 0.5 },
        show_bg: true,
        draw_bg: {
            color: #e8e8e8
        }

        <Label> {
            text: "Working Dir:",
            draw_text: { text_style: { font_size: 10.0 } }
        }

        working_dir_input = <TextInput> {
            width: Fill,
            height: Fit,
            empty_text: "/path/to/project"
        }
    }

    // Dual Agent Panel
    DualAgentPanel = <View> {
        width: Fill,
        height: Fill,
        flow: Down,
        show_bg: true,
        draw_bg: {
            color: #ffffff
        }

        working_dir_bar = <WorkingDirBar> {}

        panels = <View> {
            width: Fill,
            height: Fill,
            flow: Right,

            // Claude Code Panel (Left)
            claude_panel = <AgentChatPanel> {
                header = {
                    agent_label = { text: "Claude Code" }
                }
            }

            // Divider
            <View> {
                width: 2,
                height: Fill,
                show_bg: true,
                draw_bg: { color: #ddd }
            }

            // Codex Panel (Right)
            codex_panel = <AgentChatPanel> {
                header = {
                    agent_label = { text: "Codex" }
                }
            }
        }
    }

    App = {{App}} {
        ui: <Root> {
            <Window> {
                body = <DualAgentPanel> {}
            }
        }
    }
}

/// Agent type for configuration.
#[derive(Clone, Debug, PartialEq)]
enum AgentType {
    ClaudeCode,
    Codex,
}

impl AgentType {
    fn config(&self, working_dir: PathBuf) -> AcpAgentConfig {
        match self {
            AgentType::ClaudeCode => AcpAgentConfig::claude_code(working_dir),
            AgentType::Codex => AcpAgentConfig::codex(working_dir),
        }
    }

    fn name(&self) -> &'static str {
        match self {
            AgentType::ClaudeCode => "Claude Code",
            AgentType::Codex => "Codex",
        }
    }
}

/// Per-agent state.
/// The connection and client fields are kept to maintain the connection alive.
#[allow(dead_code)]
struct AgentState {
    connection: Arc<StdioAcpConnection>,
    client: AcpBotClient,
    connected: bool,
}

#[derive(Live)]
struct App {
    #[live]
    ui: WidgetRef,
    #[rust]
    claude_agent: Option<AgentState>,
    #[rust]
    codex_agent: Option<AgentState>,
}

impl App {
    fn get_working_dir(&self) -> PathBuf {
        let text = self
            .ui
            .text_input(id!(working_dir_bar.working_dir_input))
            .text();

        if text.is_empty() {
            std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."))
        } else {
            PathBuf::from(text)
        }
    }

    fn handle_connect_button(&mut self, cx: &mut Cx, is_claude: bool) {
        let agent = if is_claude {
            &self.claude_agent
        } else {
            &self.codex_agent
        };

        let is_connected = agent.as_ref().map(|a| a.connected).unwrap_or(false);
        let working_dir = self.get_working_dir();
        let agent_type = if is_claude {
            AgentType::ClaudeCode
        } else {
            AgentType::Codex
        };

        if is_connected {
            // Disconnect - just mark as disconnected, connection will be dropped
            if is_claude {
                self.claude_agent = None;
            } else {
                self.codex_agent = None;
            }
            self.update_connection_status(cx, is_claude, false);
        } else {
            // Connect
            let config = agent_type.config(working_dir.clone());
            let connection = Arc::new(StdioAcpConnection::new(config));
            let client = AcpBotClient::new(connection.clone(), working_dir);

            let ui = self.ui_runner();
            let connection_clone = connection.clone();

            spawn(async move {
                match connection_clone.start().await {
                    Ok(info) => {
                        log_crate::info!("Agent {} started: {:?}", agent_type.name(), info);
                        ui.defer(move |me, cx, _scope| {
                            // Store the agent state
                            let state = AgentState {
                                connection: connection_clone,
                                client: client.clone(),
                                connected: true,
                            };

                            if is_claude {
                                me.claude_agent = Some(state);
                            } else {
                                me.codex_agent = Some(state);
                            }

                            me.update_connection_status(cx, is_claude, true);
                            me.configure_agent_chat(cx, client, is_claude);
                        });
                    }
                    Err(e) => {
                        log_crate::error!("Failed to start agent: {}", e);
                        ui.defer(move |me, cx, _scope| {
                            me.update_connection_status(cx, is_claude, false);
                        });
                    }
                }
            });
        }
    }

    fn configure_agent_chat(&mut self, cx: &mut Cx, client: AcpBotClient, is_claude: bool) {
        let chat_id: &[LiveId] = if is_claude {
            id!(panels.claude_panel.chat)
        } else {
            id!(panels.codex_panel.chat)
        };

        let mut bot_context = BotContext::from(client.clone());
        let mut chat = self.ui.chat(chat_id);
        chat.write().set_bot_context(cx, Some(bot_context.clone()));

        let ui = self.ui_runner();
        let chat_id_owned: Vec<LiveId> = chat_id.to_vec();

        spawn(async move {
            let _ = bot_context.load().await;

            ui.defer(move |me, cx, _scope| {
                let mut chat = me.ui.chat(&chat_id_owned);

                chat.write()
                    .set_bot_context(cx, Some(bot_context.clone()));

                if let Some(bot) = bot_context.bots().first() {
                    chat.write().set_bot_id(cx, Some(bot.id.clone()));
                    log_crate::info!(
                        "Bot configured: {} with capabilities: {:?}",
                        bot.name,
                        bot.capabilities
                    );
                }

                me.ui.redraw(cx);
            });
        });

        // Configure permission hooks to handle tool call approvals
        self.configure_permission_hooks(cx, client, chat_id);
    }

    fn configure_permission_hooks(
        &mut self,
        _cx: &mut Cx,
        client: AcpBotClient,
        chat_id: &[LiveId],
    ) {
        let chat_id_owned: Vec<LiveId> = chat_id.to_vec();

        self.ui
            .chat(&chat_id_owned)
            .write()
            .set_hook_before(move |task_group, chat, _cx| {
                log_crate::debug!(
                    "hook_before called with {} tasks",
                    task_group.len()
                );

                let client = client.clone();
                let mut tasks_to_remove = Vec::new();
                let mut tasks_to_add = Vec::new();

                for (i, task) in task_group.iter().enumerate() {
                    match task {
                        ChatTask::ApproveToolCalls(index) => {
                            log_crate::info!(
                                "ApproveToolCalls task detected for index {}",
                                index
                            );

                            let tool_calls: Vec<ToolCall> = chat.messages_ref().read().messages
                                .get(*index)
                                .map(|m| m.content.tool_calls.clone())
                                .unwrap_or_default();

                            for tc in &tool_calls {
                                let option_id = tc
                                    .permission_options
                                    .iter()
                                    .find(|o| o.kind.is_allow())
                                    .map(|o| o.option_id.as_str());

                                moly_kit::protocol::BotClient::approve_tool_call(
                                    &client,
                                    &tc.id,
                                    option_id,
                                );
                            }

                            if let Some(message) = chat.messages_ref().read().messages.get(*index).cloned() {
                                let mut updated = message.clone();
                                updated.update_content(|content| {
                                    for tool_call in &mut content.tool_calls {
                                        tool_call.permission_status =
                                            ToolCallPermissionStatus::Approved;
                                    }
                                });
                                tasks_to_add.push(ChatTask::UpdateMessage(*index, updated));
                            }

                            tasks_to_remove.push(i);
                        }
                        ChatTask::DenyToolCalls(index) => {
                            log_crate::info!(
                                "DenyToolCalls task detected for index {}",
                                index
                            );

                            let tool_calls: Vec<ToolCall> = chat.messages_ref().read().messages
                                .get(*index)
                                .map(|m| m.content.tool_calls.clone())
                                .unwrap_or_default();

                            for tc in &tool_calls {
                                let option_id = tc
                                    .permission_options
                                    .iter()
                                    .find(|o| o.kind.is_reject())
                                    .map(|o| o.option_id.as_str());

                                moly_kit::protocol::BotClient::deny_tool_call(
                                    &client,
                                    &tc.id,
                                    option_id,
                                );
                            }

                            if let Some(message) = chat.messages_ref().read().messages.get(*index).cloned() {
                                let mut updated = message.clone();
                                updated.update_content(|content| {
                                    for tool_call in &mut content.tool_calls {
                                        tool_call.permission_status =
                                            ToolCallPermissionStatus::Denied;
                                    }
                                });
                                tasks_to_add.push(ChatTask::UpdateMessage(*index, updated));
                            }

                            tasks_to_remove.push(i);
                        }
                        _ => {}
                    }
                }

                // Remove handled tasks in reverse order
                for i in tasks_to_remove.into_iter().rev() {
                    task_group.remove(i);
                }

                task_group.extend(tasks_to_add);
            });
    }

    fn update_connection_status(&mut self, cx: &mut Cx, is_claude: bool, connected: bool) {
        let (status_id, button_id, forward_bar_id) = if is_claude {
            (
                id!(panels.claude_panel.header.status_label),
                id!(panels.claude_panel.header.connect_button),
                id!(panels.claude_panel.forward_bar),
            )
        } else {
            (
                id!(panels.codex_panel.header.status_label),
                id!(panels.codex_panel.header.connect_button),
                id!(panels.codex_panel.forward_bar),
            )
        };

        let status_label = self.ui.label(status_id);
        let button = self.ui.button(button_id);

        if connected {
            status_label.set_text(cx, "Connected");
            button.set_text(cx, "Disconnect");
        } else {
            status_label.set_text(cx, "Disconnected");
            button.set_text(cx, "Connect");
        }

        self.ui.view(forward_bar_id).set_visible(cx, connected);
        self.ui.redraw(cx);
    }

    fn forward_message(&mut self, cx: &mut Cx, from_claude: bool) {
        let (source_chat_id, target_chat_id, prefix) = if from_claude {
            (
                id!(panels.claude_panel.chat),
                id!(panels.codex_panel.chat),
                "[Forwarded from Claude Code]",
            )
        } else {
            (
                id!(panels.codex_panel.chat),
                id!(panels.claude_panel.chat),
                "[Forwarded from Codex]",
            )
        };

        let source_chat = self.ui.chat(source_chat_id);
        let messages = source_chat.read().messages_ref();
        let messages_read = messages.read();

        if let Some(last_msg) = messages_read.messages.last() {
            if last_msg.from != EntityId::User {
                let text = last_msg.content.text.clone();
                drop(messages_read);

                let mut target_chat = self.ui.chat(target_chat_id);
                let forward_msg = format!("{}\n\n{}", prefix, text);

                target_chat
                    .read()
                    .messages_ref()
                    .write()
                    .messages
                    .push(Message {
                        from: EntityId::User,
                        content: MessageContent {
                            text: forward_msg,
                            ..Default::default()
                        },
                        metadata: MessageMetadata::default(),
                    });

                target_chat.write().perform(cx, &[ChatTask::Send]);
                self.ui.redraw(cx);
            }
        }
    }
}

impl LiveRegister for App {
    fn live_register(cx: &mut Cx) {
        makepad_widgets::live_design(cx);
        moly_kit::live_design(cx);
    }
}

impl LiveHook for App {
    fn after_new_from_doc(&mut self, cx: &mut Cx) {
        // Set default working directory
        let working_dir = std::env::current_dir()
            .unwrap_or_else(|_| PathBuf::from("."))
            .to_string_lossy()
            .to_string();
        self.ui
            .text_input(id!(working_dir_bar.working_dir_input))
            .set_text(cx, &working_dir);
    }
}

impl AppMain for App {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event) {
        self.ui_runner()
            .handle(cx, event, &mut Scope::empty(), self);
        self.match_event(cx, event);
        self.ui.handle_event(cx, event, &mut Scope::empty());
    }
}

impl MatchEvent for App {
    fn handle_actions(&mut self, cx: &mut Cx, actions: &Actions) {
        // Handle Claude connect button
        if self
            .ui
            .button(id!(panels.claude_panel.header.connect_button))
            .clicked(actions)
        {
            self.handle_connect_button(cx, true);
        }

        // Handle Codex connect button
        if self
            .ui
            .button(id!(panels.codex_panel.header.connect_button))
            .clicked(actions)
        {
            self.handle_connect_button(cx, false);
        }

        // Forward from Claude to Codex
        if self
            .ui
            .button(id!(panels.claude_panel.forward_bar.forward_button))
            .clicked(actions)
        {
            self.forward_message(cx, true);
        }

        // Forward from Codex to Claude
        if self
            .ui
            .button(id!(panels.codex_panel.forward_bar.forward_button))
            .clicked(actions)
        {
            self.forward_message(cx, false);
        }
    }
}

app_main!(App);
