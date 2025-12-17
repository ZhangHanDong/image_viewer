# Rust + Makepad AI App 训练营课程计划

## 训练营概述

**主题**: 用 Rust 和 Makepad 开发 AI App
**周期**: 4 周（每周末授课，周六 + 周日）
**规模**: 30+ 人大班制
**前置要求**: 有 Rust 基础（了解所有权、借用、基本语法）
**目标**: 学员能够独立使用 Rust + Makepad 构建具有 AI 功能的桌面应用

---

## 课程结构

### 第一周：Makepad 框架入门

#### 周六（3-4小时）
**上午：Makepad 框架概述**
- Makepad 架构与设计理念
- 项目结构与 `live_design!` 宏
- Live DSL 语法入门
- 对应 Workshop: `1 - Building a Minimal App`

**下午：实操 - 第一个 Makepad 应用**
- 环境配置与项目创建
- 理解 Widget 生命周期
- 实操：Hello Makepad

#### 周日（3-4小时）
**上午：UI 组件基础**
- View、Button、Image 基础组件
- 布局系统：Flow（Down/Right/Overlay）、Fill、Fit
- 对应 Workshop: `2 - Building a Slideshow`

**下午：实战练习**
- 完成 Slideshow 的按钮和图片展示
- SVG 图标与 `dep()` 资源加载
- 样式配置与主题

#### 第一周作业（20分）
1. **基础作业（10分）**: 完成一个显示个人信息的卡片组件（头像、姓名、简介）
2. **进阶作业（10分）**: 为卡片添加社交链接按钮（带自定义图标）

---

### 第二周：交互与状态管理

#### 周六（3-4小时）
**上午：事件处理**
- Makepad 事件系统
- `handle_event()` 方法详解
- Button 点击事件处理
- 对应 Workshop: `3 - Making the Slideshow Interactive`

**下午：状态管理**
- `#[rust]` 与 `#[live]` 属性区别
- 组件状态更新与重绘
- Widget 引用与 `id!()` 宏

#### 周日（3-4小时）
**上午：复杂布局 - Grid 系统**
- PortalList 高效列表渲染
- Grid 布局设计
- 对应 Workshop: `4 - Building an Image Grid`

**下午：动态数据绑定**
- 文件系统操作与图片加载
- 异步图片加载 `load_image_file_by_path_async()`
- 对应 Workshop: `5 - Making the Image Grid Dynamic`

#### 第二周作业（25分）
1. **基础作业（10分）**: 实现一个可点击切换的图片轮播
2. **进阶作业（15分）**: 实现一个动态加载本地文件夹图片的网格视图

---

### 第三周：高级 UI & 多页面应用

#### 周六（3-4小时）
**上午：多视图导航**
- PageFlip 组件实现页面切换
- 菜单栏与导航设计
- 对应 Workshop: `6 - Switching Between Views`

**下午：动画系统**
- Animator 状态机
- Hover 效果与过渡动画
- 动画时间与缓动函数
- 对应 Workshop: `7 - Adding Animations`

#### 周日（3-4小时）
**上午：Shader 基础**
- GLSL 着色器入门
- `fn pixel()` 自定义渲染
- 对应 Workshop: `8 - Styling With Shaders`

**下午：综合实战**
- 整合所有学到的技术
- 代码审查与优化建议

#### 第三周作业（25分）
1. **基础作业（10分）**: 为 Grid 项添加 hover 动画效果
2. **进阶作业（15分）**: 实现带动画的双视图切换应用（Grid ↔ 详情）

---

### 第四周：AI 集成 & 毕业项目

#### 周六（3-4小时）
**上午：Moly Kit 入门**
- OpenAI 兼容 API 配置
- Chat Widget 集成
- BotContext 与 UiRunner
- 对应 Workshop: `Appendix/1 - Embedding an LLM Chat`

**下午：Vision 能力集成**
- 自定义 BotClient trait
- 图片上下文注入
- 对应 Workshop: `Appendix/2 - Current Image as Conversation Context`

#### 周日（3-4小时）
**上午：图像生成集成**
- DALL-E / 图像生成 API
- Hook 系统深入
- 对应 Workshop: `Appendix/3 - Generating Images to the Grid`

**下午：毕业项目答辩**
- 学员展示毕业项目
- 评审与反馈
- 颁发结业证书

#### 第四周作业 & 毕业项目（30分）
**毕业项目要求**:
选择以下方向之一完成：
1. **AI 图片助手**: 结合 Vision API 的图片分析应用
2. **AI 图片生成器**: 文字描述生成图片的创意工具
3. **自选项目**: 使用 Makepad + AI API 的创意应用（需提前审批）

---

## 积分与考核体系

### 积分构成（总分 100 分）

| 项目 | 分数 | 说明 |
|------|------|------|
| 第一周作业 | 20 分 | 基础 10 + 进阶 10 |
| 第二周作业 | 25 分 | 基础 10 + 进阶 15 |
| 第三周作业 | 25 分 | 基础 10 + 进阶 15 |
| 毕业项目 | 30 分 | 完整度 + 创意 + 代码质量 |

### 毕业标准
- **合格**: 60 分以上
- **良好**: 75 分以上
- **优秀**: 90 分以上

### 额外加分项（最多 15 分）
- 帮助其他学员解决问题：+2 分/次（最多 +6）
- 提交 Workshop 改进 PR：+5 分/个
- 课堂积极互动：+1 分/次（最多 +4）
- **完成 ACP Agent 选修内容**：+5 分（需提交可运行 Demo）

---

## 作业提交与评分规则（大班制）

### 组织架构
- 按 5-6 人分组，每组选 1 名组长
- 组长职责：组织组内代码互评、汇总问题、协助答疑

### 提交方式
- 创建个人 GitHub 仓库
- 每周作业提交 PR 到指定分支
- 截止时间：下周五 23:59

### 评分流程（三级评审）
1. **组内互评**（占 30%）：组员之间交叉评审，每人至少评 2 份
2. **组长评审**（占 30%）：组长对组内作业进行总体评价
3. **助教抽查**（占 40%）：助教随机抽查 + 重点关注

### 评分标准
- **功能完整性**: 40%
- **代码质量**: 30%（可读性、Rust 惯用法）
- **UI/UX**: 20%
- **文档/注释**: 10%

### 迟交规则
- 迟交 1-3 天：扣 20%
- 迟交 3-7 天：扣 50%
- 超过 7 天：不接受提交

---

## 课前准备要求

### 环境配置
```bash
# Rust 工具链
rustup default nightly

# 克隆 Workshop 仓库
git clone <workshop-repo>

# 验证环境
cd "1 - Building a Minimal App/1.1 - Setting Up Our Project Structure"
cargo run
```

### 前置知识
- Rust 基础语法（变量、函数、结构体、枚举）
- 基本的命令行操作
- Git 基础操作

### 推荐预习
- Rust Book 第 1-10 章
- Makepad 官方文档

---

## AI API 费用方案（混合模式）

### 课堂演示
- 使用训练营统一账号
- 讲师演示和现场 Debug 使用

### 学员作业
学员可自由选择任意 OpenAI 兼容的大模型 API，包括但不限于：

| 提供商 | API 端点 | 聊天模型示例 | 图像生成 |
|--------|----------|--------------|----------|
| OpenAI | `https://api.openai.com/v1` | `gpt-4o-mini` | `dall-e-3` |
| 智谱 AI | `https://open.bigmodel.cn/api/paas/v4` | `glm-4-flash` | `cogview-3` |
| 通义千问 | `https://dashscope.aliyuncs.com/compatible-mode/v1` | `qwen-turbo` | `wanx-v1` |
| 月之暗面 | `https://api.moonshot.cn/v1` | `moonshot-v1-8k` | - |
| DeepSeek | `https://api.deepseek.com/v1` | `deepseek-chat` | - |
| 硅基流动 | `https://api.siliconflow.cn/v1` | `Qwen/Qwen2.5-7B-Instruct` | `black-forest-labs/FLUX.1-schnell` |
| 快手 Kling | `https://api.klingai.com/v1` | `kling-chat` | `kling-v1` |

> 注：部分提供商不支持图像生成，第四周作业可使用支持图像生成的 API 或使用统一测试账号

### 配置示例
```bash
# .env 文件
API_URL="https://api.openai.com/v1"  # 或其他兼容端点
API_KEY="sk-xxx"
MODEL_ID="gpt-4o-mini"               # 聊天模型
IMAGE_MODEL_ID="dall-e-3"            # 图像生成（第四周）
```

### 费用预估
- 国内 API 通常更便宜，建议预算：¥20-50 / 月
- 部分提供商有免费额度（如智谱、硅基流动）

---

## 选修内容：ACP Agent 集成

**对应 Workshop**: `Appendix/Moly Kit Integration/4 - Using ACP Agents`

### 概述

ACP (Agent Client Protocol) 允许将 Claude Code、Codex 等 AI 编程代理集成到 Makepad 应用中。学员可以学习如何构建一个可以调用真正 AI 编程代理的桌面应用。

### 技术架构

| 层级 | 组件 | 职责 |
|------|------|------|
| UI 层 | Makepad App + Chat Widget | 用户界面、消息显示、权限按钮 |
| 适配层 | AcpBotClient | 实现 BotClient trait，转换消息格式 |
| 协议层 | AcpClient (moly-kit) | JSON-RPC 2.0 通信，进程管理 |
| 桥接层 | ACP Adapters (npm) | 将 ACP 协议转换为各 Agent 的原生协议 |
| Agent 层 | Claude Code / Codex | 实际执行代码操作的 AI Agent |
| 云服务层 | Anthropic / OpenAI API | 提供 AI 推理能力 |

### 学习内容

#### 1. AcpClient 使用
- 使用 Moly Kit 的 `AcpClient` 与 ACP 代理通信
- 创建 `BotClient` 适配器封装 ACP 代理

#### 2. 事件映射
| ACP 事件 | MessageContent 字段 |
|----------|---------------------|
| `AcpEvent::Text` | `text` |
| `AcpEvent::Thinking` | `reasoning` |
| `AcpEvent::ToolUse` | `tool_calls` (Approved) |
| `AcpEvent::PermissionRequest` | `tool_calls` (Pending) |
| `AcpEvent::ToolResult` | `tool_results` |

#### 3. 权限处理流程
1. 收到 `AcpEvent::PermissionRequest`
2. Chat Widget 显示 Approve/Deny 按钮
3. 用户点击批准/拒绝
4. 调用 `AcpClient::respond_permission()` 或 `cancel_permission()`
5. 代理继续执行或取消

#### 4. 图像上下文传递
- 将当前图片路径传递给 Agent
- 代理可以分析或处理图像

### 前置准备

```bash
# 安装 Claude Code ACP 适配器
npm install -g @zed-industries/claude-code-acp

# 认证方式（二选一）
claude login                        # 订阅用户浏览器登录
export ANTHROPIC_API_KEY="your-key" # API Key 方式

# 或安装 Codex ACP 适配器
npm install -g @zed-industries/codex-acp
codex /login                        # 订阅用户登录
export OPENAI_API_KEY="your-key"    # API Key 方式
```

### UI 组件
- **Agent Dropdown**: 选择 Claude Code 或 Codex
- **Working Directory Input**: 设置代理工作目录
- **Connect Button**: 启动/停止代理连接
- **Status Label**: 显示连接状态
- **ACP Chat**: 与代理交互的聊天界面

### 加分规则
- 完成可运行 Demo：+5 分
- 需提交录屏或现场演示
- 加分展示（可选）：
  - 添加自定义工具供代理使用
  - 实现会话持久化管理
  - 配置 MCP Server 扩展代理能力

---

## 答疑与支持

### 答疑时间
- **周三晚 20:00-21:30**：线上答疑（腾讯会议/飞书）
- **周末课后**：现场答疑 30 分钟

### 沟通渠道
- 微信/飞书学习群：日常问题
- GitHub Issues：技术问题存档
- 组长收集：汇总共性问题

---

## 时间线总览

| 周次 | 周六主题 | 周日主题 | 作业 |
|------|----------|----------|------|
| 1 | Makepad 入门 | UI 组件基础 | 个人卡片组件 |
| 2 | 事件与状态 | Grid 与动态数据 | 图片轮播 + 网格 |
| 3 | 多视图导航 + 动画 | Shader 基础 | 动画双视图应用 |
| 4 | LLM + Vision 集成 | 图像生成 + 答辩 | 毕业项目 |
