# Moly Kit ACP Multi-Agent Architecture

## Overview

This document describes the architecture of Moly Kit's ACP (Agent Client Protocol) integration for supporting multiple AI coding agents like Claude Code and Codex.

## Architecture Diagram

```
┌─────────────────────────────────────────────────────────────────────────────────┐
│                              Makepad Application                                 │
│  ┌───────────────────────────────────────────────────────────────────────────┐  │
│  │                           App (app.rs)                                    │  │
│  │  ┌─────────────────────┐              ┌─────────────────────┐             │  │
│  │  │   Claude Panel      │              │    Codex Panel      │             │  │
│  │  │  ┌───────────────┐  │              │  ┌───────────────┐  │             │  │
│  │  │  │  Chat Widget  │  │   Forward    │  │  Chat Widget  │  │             │  │
│  │  │  │  (moly-kit)   │◄─┼──────────────┼─►│  (moly-kit)   │  │             │  │
│  │  │  └───────┬───────┘  │              │  └───────┬───────┘  │             │  │
│  │  │          │          │              │          │          │             │  │
│  │  │  ┌───────▼───────┐  │              │  ┌───────▼───────┐  │             │  │
│  │  │  │ AcpBotClient  │  │              │  │ AcpBotClient  │  │             │  │
│  │  │  │  (Adapter)    │  │              │  │  (Adapter)    │  │             │  │
│  │  │  └───────┬───────┘  │              │  └───────┬───────┘  │             │  │
│  │  └──────────┼──────────┘              └──────────┼──────────┘             │  │
│  └─────────────┼────────────────────────────────────┼────────────────────────┘  │
└────────────────┼────────────────────────────────────┼────────────────────────────┘
                 │                                    │
                 │ BotClient Trait                    │ BotClient Trait
                 │                                    │
┌────────────────┼────────────────────────────────────┼────────────────────────────┐
│                │           Moly Kit                 │                            │
│  ┌─────────────▼─────────────────────────────────────▼─────────────────────┐     │
│  │                         AcpClient (acp_client.rs)                       │     │
│  │  ┌─────────────────────────────────────────────────────────────────┐    │     │
│  │  │                    JSON-RPC 2.0 Handler                         │    │     │
│  │  │  • on_receive_notification (SessionNotification)                │    │     │
│  │  │  • on_receive_request (RequestPermissionRequest)                │    │     │
│  │  │  • send_request (Initialize, NewSession, Prompt)                │    │     │
│  │  └─────────────────────────────────────────────────────────────────┘    │     │
│  │  ┌─────────────────────────────────────────────────────────────────┐    │     │
│  │  │                     ByteStreams (sacp)                          │    │     │
│  │  │  • Async read/write streams                                     │    │     │
│  │  │  • Message serialization/deserialization                        │    │     │
│  │  └─────────────────────────────────────────────────────────────────┘    │     │
│  └─────────────────────────────────────────────────────────────────────────┘     │
│                 │                                    │                           │
└─────────────────┼────────────────────────────────────┼───────────────────────────┘
                  │ stdio (stdin/stdout)               │ stdio (stdin/stdout)
                  │                                    │
┌─────────────────┼────────────────────────────────────┼───────────────────────────┐
│                 │        ACP Adapters (npm)          │                           │
│  ┌──────────────▼──────────────┐      ┌──────────────▼──────────────┐            │
│  │   claude-code-acp           │      │      codex-acp              │            │
│  │  (@zed-industries)          │      │   (@zed-industries)         │            │
│  │                             │      │                             │            │
│  │  ┌───────────────────────┐  │      │  ┌───────────────────────┐  │            │
│  │  │  ACP Protocol Bridge  │  │      │  │  ACP Protocol Bridge  │  │            │
│  │  │  • JSON-RPC ↔ Native  │  │      │  │  • JSON-RPC ↔ Native  │  │            │
│  │  └───────────┬───────────┘  │      │  └───────────┬───────────┘  │            │
│  └──────────────┼──────────────┘      └──────────────┼──────────────┘            │
└─────────────────┼────────────────────────────────────┼───────────────────────────┘
                  │                                    │
                  │ Native Protocol                    │ Native Protocol
                  │                                    │
┌─────────────────┼────────────────────────────────────┼───────────────────────────┐
│                 │         AI Agent CLIs              │                           │
│  ┌──────────────▼──────────────┐      ┌──────────────▼──────────────┐            │
│  │       Claude Code           │      │         Codex               │            │
│  │      (Anthropic)            │      │       (OpenAI)              │            │
│  │                             │      │                             │            │
│  │  • Read/Write files         │      │  • Read/Write files         │            │
│  │  • Execute commands         │      │  • Execute commands         │            │
│  │  • Search codebase          │      │  • Search codebase          │            │
│  │  • Git operations           │      │  • Git operations           │            │
│  └──────────────┬──────────────┘      └──────────────┬──────────────┘            │
└─────────────────┼────────────────────────────────────┼───────────────────────────┘
                  │                                    │
                  │ API Calls                          │ API Calls
                  │                                    │
┌─────────────────▼────────────────────────────────────▼───────────────────────────┐
│                            Cloud AI Services                                     │
│  ┌─────────────────────────────┐      ┌─────────────────────────────┐            │
│  │    Anthropic API            │      │      OpenAI API             │            │
│  │    (Claude Models)          │      │    (GPT-4/o1 Models)        │            │
│  └─────────────────────────────┘      └─────────────────────────────┘            │
└──────────────────────────────────────────────────────────────────────────────────┘
```

## Data Flow

```
┌──────────────────────────────────────────────────────────────────────────────────┐
│                              Data Flow                                           │
├──────────────────────────────────────────────────────────────────────────────────┤
│                                                                                  │
│  User Input → Chat Widget → AcpBotClient.send() → AcpClient.prompt()             │
│                                    │                    │                        │
│                                    │                    ▼                        │
│                                    │         JSON-RPC Request (stdio)            │
│                                    │                    │                        │
│                                    │                    ▼                        │
│                                    │            ACP Adapter                      │
│                                    │                    │                        │
│                                    │                    ▼                        │
│                                    │           Agent CLI Process                 │
│                                    │                    │                        │
│                                    │                    ▼                        │
│                                    │             Cloud AI API                    │
│                                    │                    │                        │
│                                    ◄────────────────────┘                        │
│                                    │                                             │
│  AcpEvent Stream:                  │                                             │
│  • Text(String)         ◄──────────┤ SessionNotification                         │
│  • Thinking(String)     ◄──────────┤                                             │
│  • ToolUse{...}         ◄──────────┤                                             │
│  • PermissionRequest    ◄──────────┤ RequestPermissionRequest                    │
│  • ToolResult{...}      ◄──────────┤                                             │
│  • Completed            ◄──────────┘                                             │
│                                                                                  │
└──────────────────────────────────────────────────────────────────────────────────┘
```

## Permission Flow

```
┌──────────────────────────────────────────────────────────────────────────────────┐
│                         Permission Flow                                          │
├──────────────────────────────────────────────────────────────────────────────────┤
│                                                                                  │
│  Agent wants to execute tool (e.g., write file)                                  │
│           │                                                                      │
│           ▼                                                                      │
│  ACP Adapter sends RequestPermissionRequest                                      │
│           │                                                                      │
│           ▼                                                                      │
│  AcpClient receives → AcpEvent::PermissionRequest                                │
│           │                                                                      │
│           ▼                                                                      │
│  AcpBotClient → ToolCall { permission_status: Pending }                          │
│           │                                                                      │
│           ▼                                                                      │
│  Chat Widget shows [Approve] [Deny] buttons                                      │
│           │                                                                      │
│           ▼                                                                      │
│  User clicks → ChatTask::ApproveToolCalls / DenyToolCalls                        │
│           │                                                                      │
│           ▼                                                                      │
│  Hook intercepts → AcpClient.respond_permission(true/false)                      │
│           │                                                                      │
│           ▼                                                                      │
│  Agent continues or cancels                                                      │
│                                                                                  │
└──────────────────────────────────────────────────────────────────────────────────┘
```

## Key Components

| Component | Location | Responsibility |
|-----------|----------|----------------|
| **BotClient** | `moly-kit/src/protocol.rs` | Trait that Chat widget uses to communicate with bots |
| **AcpBotClient** | `acp_bot_client.rs` | Adapter implementing BotClient for ACP agents |
| **AcpClient** | `moly-kit/src/acp/acp_client.rs` | Low-level ACP protocol client |
| **ACP Adapters** | npm packages | Bridge ACP ↔ native agent protocols |

### BotClient Trait

```rust
pub trait BotClient {
    fn bots(&self) -> BoxPlatformSendFuture<'static, ClientResult<Vec<Bot>>>;
    fn send(
        &mut self,
        bot_id: &BotId,
        messages: &[Message],
        tools: &[Tool],
    ) -> BoxPlatformSendStream<'static, ClientResult<MessageContent>>;
    fn clone_box(&self) -> Box<dyn BotClient>;
}
```

### AcpBotClient Adapter

- Converts `Message` → ACP prompt format
- Converts `AcpEvent` → `MessageContent`
- Handles attachments and image context
- Manages permission request state

### AcpClient

- Spawns agent process via `tokio::process::Command`
- Manages stdio communication (stdin/stdout pipes)
- Handles JSON-RPC 2.0 message serialization
- Receives `SessionNotification` for streaming updates
- Handles `RequestPermissionRequest` for tool approval

## Layer Architecture

| Layer | Components | Responsibility |
|-------|------------|----------------|
| **UI Layer** | Makepad App + Chat Widget | User interface, message display, permission buttons |
| **Adapter Layer** | AcpBotClient | Implements BotClient trait, converts message formats |
| **Protocol Layer** | AcpClient (moly-kit) | JSON-RPC 2.0 communication, process management |
| **Bridge Layer** | ACP Adapters (npm) | Converts ACP protocol to native agent protocols |
| **Agent Layer** | Claude Code / Codex | AI agents that execute code operations |
| **Cloud Layer** | Anthropic / OpenAI API | Provides AI inference capabilities |

## Adding New Agents

To add support for a new agent (e.g., Aider, Cursor):

1. **Install ACP adapter** (if available):
   ```bash
   npm install -g @zed-industries/new-agent-acp
   ```

2. **Add to AgentType enum** in `acp_bot_client.rs`:
   ```rust
   pub enum AgentType {
       ClaudeCode,
       Codex,
       NewAgent,  // Add new variant
   }

   impl AgentType {
       pub fn command(&self) -> &'static str {
           match self {
               AgentType::ClaudeCode => "claude-code-acp",
               AgentType::Codex => "codex-acp",
               AgentType::NewAgent => "new-agent-acp",
           }
       }
   }
   ```

3. **Update UI** to include the new agent option

## Limitations

- **No attach to existing processes**: ACP spawns new processes; cannot connect to running agent terminals
- **Text-based protocol**: Images are passed as file paths, not binary data
- **Permission handling varies**: Some adapters send permissions as text, not formal requests

## References

- [ACP Specification](https://github.com/anthropics/acp)
- [Zed ACP Adapters](https://github.com/zed-industries)
- [Moly Kit Documentation](https://github.com/moxin-org/moly)
