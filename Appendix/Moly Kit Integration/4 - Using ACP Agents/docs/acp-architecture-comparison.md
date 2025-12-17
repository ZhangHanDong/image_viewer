# Zed ACP 架构 vs MolyKit 当前实现对比

## 架构分层对比

### Zed 架构

```
AcpConnection (连接层) → AcpThread (线程/会话管理) → AcpThreadView (UI)
```

- **AcpConnection**: 管理与代理进程的 stdio 通信
- **AcpThread**: 独立的 entity，管理消息、工具调用、权限请求的状态
- **AcpThreadView**: UI 渲染层，订阅 AcpThread 状态变化

### MolyKit 当前架构

```
AcpClient (连接层) → AcpBotClient (BotClient适配器) → Chat Widget (UI)
```

- **AcpClient**: 管理与代理进程的通信
- **AcpBotClient**: 适配 BotClient trait，同时处理状态管理
- **Chat Widget**: 通用聊天 UI

**问题**: 缺少独立的"线程/会话管理"层。状态管理和事件处理混合在 `AcpBotClient` 的 event 处理闭包中。

## 状态管理差异

### Zed 的做法

```rust
// AcpThread 是独立的 entity
pub struct ToolCall {
    pub id: acp::ToolCallId,
    pub label: Entity<Markdown>,
    pub kind: acp::ToolKind,
    pub content: Vec<ToolCallContent>,
    pub status: ToolCallStatus,  // 明确的状态枚举
    pub locations: Vec<acp::ToolCallLocation>,
    pub raw_input: Option<serde_json::Value>,
    pub raw_output: Option<serde_json::Value>,
}

pub enum AgentThreadEntry {
    UserMessage(UserMessage),
    AssistantMessage(AssistantMessage),
    ToolCall(ToolCall),
}
```

- `AcpThread` 维护完整的会话状态
- `ToolCall` 有明确的 `status` 字段
- UI 订阅状态变化自动更新

### MolyKit 当前做法

```rust
// 状态在闭包内管理
while let Some(event) = event_rx.next().await {
    match event {
        AcpEvent::ToolUse { ... } => {
            tool_calls.push(...);  // 局部变量
            tx.unbounded_send(...);  // 直接发送
        }
        AcpEvent::PermissionRequest { ... } => {
            // 需要找到之前的 tool_call 并更新状态
            // tool_calls 是闭包内的局部变量
        }
    }
}
```

- 状态分散在多处（`tool_calls` 局部变量、`AcpBotState.pending_permission`）
- 每次事件都重新构造 `MessageContent` 发送
- 需要手动匹配 `ToolUse` 和 `PermissionRequest` 的 ID

## 当前 Bug 的根源分析

### Permission 按钮不显示/不工作

1. **ID 不匹配问题**: `ToolUse` 事件的 `tool_id` 和 `PermissionRequest` 的 `request_id` 需要手动匹配
2. **状态更新时机**: `PermissionRequest` 到达时，需要找到对应的 `ToolCall` 并更新其 `permission_status`
3. **option_id 问题**: Codex 发送的 `PermissionRequest` 包含特定的 `option_id`，需要回传正确的 ID

### Quick Fix（已实现）

在 `AcpEvent::PermissionRequest` 中捕获 `allow_option_id` 和 `deny_option_id`，在响应权限时使用正确的 ID：

```rust
AcpEvent::PermissionRequest {
    request_id,
    tool_name,
    description,
    allow_option_id,   // 新增
    deny_option_id,    // 新增
}
```

## 改进建议

### 短期（Quick Fix）

- [x] 捕获 `allow_option_id` / `deny_option_id`
- [x] 响应权限时使用正确的 option_id
- [ ] 添加更多日志以便调试

### 中期（架构改进）

1. **抽取 `AcpSession` 层**
   - 独立管理会话状态（messages, tool_calls, pending_permissions）
   - 提供状态变更通知机制

2. **统一 ToolCall 状态管理**
   - 所有 tool call 状态变化都通过一个中心点
   - 明确的状态枚举和转换

3. **使用响应式状态**
   - UI 订阅状态变化
   - 不再每次事件都重新构造完整的 `MessageContent`

### 长期（参考 Zed 架构）

```rust
// 建议的架构
pub struct AcpSession {
    session_id: String,
    entries: Vec<SessionEntry>,
    pending_permissions: HashMap<String, PendingPermission>,
}

pub enum SessionEntry {
    UserMessage(UserMessage),
    AssistantMessage(AssistantMessage),
    ToolCall(ToolCall),
}

pub struct ToolCall {
    pub id: String,
    pub name: String,
    pub status: ToolCallStatus,
    pub input: serde_json::Value,
    pub output: Option<String>,
}

pub enum ToolCallStatus {
    Pending,           // 等待执行
    AwaitingApproval,  // 等待用户批准
    Approved,          // 已批准，执行中
    Denied,            // 已拒绝
    Completed,         // 已完成
    Error(String),     // 执行出错
}
```

## Zed AcpThread 核心类型参考

```rust
// 用户消息
pub struct UserMessage {
    pub id: Option<UserMessageId>,
    pub content: ContentBlock,
    pub chunks: Vec<acp::ContentBlock>,
    pub checkpoint: Option<Checkpoint>,
}

// 助手消息
pub struct AssistantMessage {
    pub chunks: Vec<AssistantMessageChunk>,
}

// 助手消息块类型
pub enum AssistantMessageChunk {
    Message { block: ContentBlock },
    Thought { block: ContentBlock },
}
```

## AgentConnection Trait 参考

```rust
pub trait AgentConnection {
    fn telemetry_id(&self) -> &'static str;
    fn new_thread(...) -> Task<Result<Entity<AcpThread>>>;
    fn auth_methods(&self) -> &[acp::AuthMethod];
    fn authenticate(&self, method: acp::AuthMethodId, cx: &mut App) -> Task<Result<()>>;
    fn prompt(&self, ...) -> Task<Result<acp::PromptResponse>>;
    fn resume(&self, session_id: &acp::SessionId, cx: &App) -> Option<Rc<dyn AgentSessionResume>>;
    fn cancel(&self, session_id: &acp::SessionId, cx: &mut App);
    fn truncate(&self, session_id: &acp::SessionId, cx: &App) -> Option<Rc<dyn AgentSessionTruncate>>;
    fn model_selector(&self, session_id: &acp::SessionId) -> Option<Rc<dyn AgentModelSelector>>;
    // ...
}
```

## 相关文件

- Zed ACP 实现: `zed/crates/acp_thread/`, `zed/crates/agent_servers/`
- MolyKit ACP: `moly-kit/src/acp/`
- Demo 适配器: `src/acp_bot_client.rs`
