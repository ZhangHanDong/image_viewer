# Lesson 4: Using ACP Agents (Claude Code & Codex)

This lesson demonstrates how to integrate ACP (Agent Client Protocol) compatible AI coding agents like Claude Code and Codex into a Makepad application using Moly Kit.

## What You'll Learn

- How to use the `AcpClient` from Moly Kit to communicate with ACP agents
- How to create a `BotClient` adapter for ACP agents
- How to handle permission requests from agents (for tool execution)
- How to pass image context to AI agents
- How to switch between different agent types

## Prerequisites

- ACP adapters for Claude Code or Codex installed
- For Claude Code: Claude subscription (Pro/Max) or `ANTHROPIC_API_KEY` environment variable
- For Codex: ChatGPT subscription or `OPENAI_API_KEY` environment variable

## Installation

### Claude Code (via ACP adapter)

Claude Code doesn't natively support ACP protocol, so we use Zed's ACP adapter:

```bash
# Install the ACP adapter from Zed
npm install -g @zed-industries/claude-code-acp

# The adapter internally uses Claude Code SDK, make sure you're authenticated:
# Option 1: Login via browser (for subscription users)
claude login

# Option 2: Set API key
export ANTHROPIC_API_KEY="your-key"
```

### Codex (via ACP adapter)

```bash
# Install the ACP adapter from Zed
npm install -g @zed-industries/codex-acp

# Authenticate:
# Option 1: Login via browser (for ChatGPT subscription users)
codex /login

# Option 2: Set API key
export OPENAI_API_KEY="your-key"
```

## Running the Example

```bash
cd "Appendix/Moly Kit Integration/4 - Using ACP Agents"
cargo run
```

## Architecture Overview

### AcpBotClient Adapter

The `AcpBotClient` wraps the `AcpClient` to implement the `BotClient` trait, enabling seamless integration with Moly Kit's Chat widget:

```rust
pub struct AcpBotClient {
    client: Arc<Mutex<Option<AcpClient>>>,
    state: Arc<Mutex<AcpBotState>>,
}

impl BotClient for AcpBotClient {
    fn send(...) -> BoxPlatformSendStream<...> {
        // Convert AcpEvents to MessageContent stream
    }
}
```

### Event Mapping

ACP events are converted to Moly Kit's message content format:

| ACP Event | MessageContent Field |
|-----------|---------------------|
| `AcpEvent::Text` | `text` |
| `AcpEvent::Thinking` | `reasoning` |
| `AcpEvent::ToolUse` | `tool_calls` (Approved) |
| `AcpEvent::PermissionRequest` | `tool_calls` (Pending) |
| `AcpEvent::ToolResult` | `tool_results` |

### Permission Handling

When the agent requests permission to execute a tool:

1. `AcpEvent::PermissionRequest` is received
2. A `ToolCall` with `ToolCallPermissionStatus::Pending` is added to the message
3. The Chat widget displays Approve/Deny buttons
4. User clicks Approve/Deny
5. `ChatTask::ApproveToolCalls` or `ChatTask::DenyToolCalls` is dispatched
6. The hook calls `AcpClient::respond_permission()` or `cancel_permission()`
7. The agent continues or cancels

## Key Components

### UI Components

- **Agent Dropdown**: Select between Claude Code and Codex
- **Working Directory Input**: Set the agent's working directory
- **Connect Button**: Start/stop the agent connection
- **Status Label**: Shows connection status
- **ACP Chat**: The chat interface for interacting with the agent

### State Management

```rust
pub struct AcpBotState {
    pub pending_permission: Option<PendingPermission>,
    pub working_dir: PathBuf,
    pub current_image_path: Option<PathBuf>,
    pub is_connected: bool,
    pub agent_type: AgentType,
}
```

## Usage Tips

1. **Image Context**: When you switch images in the slideshow, the current image path is automatically passed to the agent, allowing it to analyze or process the image.

2. **Working Directory**: Set the working directory to your project root so the agent can read and modify files.

3. **Permission Requests**: Always review what the agent is asking to do before approving. The description shows what tool will be executed.

4. **Agent Switching**: You can switch between Claude Code and Codex, but you'll need to reconnect after switching.

## Troubleshooting

### Agent Won't Connect

- Ensure the agent CLI is installed and in your PATH
- Check that the required API key environment variable is set
- Look at the console for error messages

### Permission Buttons Not Showing

- The agent must send a `PermissionRequest` for buttons to appear
- Some agents may auto-approve certain actions

### Chat Not Responding

- Check if the agent is connected (status should show "Connected")
- Look at the console for error messages
- Try disconnecting and reconnecting

## Next Steps

- Explore adding custom tools that the agent can use
- Implement persistent session management
- Add MCP server configuration for extended capabilities



  层次结构

  | 层       | 组件                        | 职责|
  |---------|---------------------------|---------------------------|
  | UI 层    | Makepad App + Chat Widget | 用户界面、消息显示、权限按钮            |
  | 适配层     | AcpBotClient              | 实现 BotClient trait，转换消息格式 |
  | 协议层     | AcpClient (moly-kit)      | JSON-RPC 2.0 通信，进程管理      |
  | 桥接层     | ACP Adapters (npm)        | 将 ACP 协议转换为各 Agent 的原生协议  |
  | Agent 层 | Claude Code / Codex       | 实际执行代码操作的 AI Agent        |
  | 云服务层    | Anthropic / OpenAI API    | 提供 AI 推理能力|
