AI Chat Summary 

本机优先 · 模块化 · 低侵入分发
从指定 群 与 时间窗口（默认过去 24h）自动汇总聊天内容，采用 Map/Reduce/Refine 的摘要流程，产出 Markdown 与 结构化 JSON，并支持以文件传输助手/公众号等渠道分发。

⸻

亮点特性
	•	隐私优先：全流程以本机执行为默认；外部模型接入前显式告知。
	•	模块化架构：extractor-* / preprocess / ai / storage / orchestrator / monitoring / sender-* 解耦，易于替换与扩展。
	•	低侵入分发：预留 WeChat 分发通道（文件传输助手/公众号），不强依赖第三方机器人。
	•	可观测性：内置最小监控服务（/health、/metrics），便于集成到本地运维脚本。
	•	密钥安全：模型 API Key 设计为存放在 macOS Keychain，不写入代码或明文配置。

⸻

架构与模块

┌─────────────────────────────────────────────────────────────┐
│                           orchestrator                      │
│   (编排 extract → preprocess → ai(Map/Reduce/Refine) → out) │
└───────────────┬───────────────────────────────┬─────────────┘
                │                               │
        extractor-*                        ai / ai-deepseek
       (数据提取层)                           (LLM Provider)
                │                               │
           preprocess                         storage
        (过滤/分块等)                        (MD/JSON 落盘)
                │
            sender-wechat
          (低侵入分发通道)
                │
             monitoring
     (/health /metrics + tracing 入口)

模块职责简述
	•	extractor-*：数据源适配（如 extractor-json、extractor-wechat）。
	•	preprocess：过滤与分块（群、时间窗口、时间间隔分段）。
	•	ai / ai-deepseek：统一 Provider 接口 + DeepSeek 实现（Map/Reduce/Refine）。
	•	storage：落盘产物（Markdown + JSON）。
	•	orchestrator：编排整条流水线。
	•	config：配置加载与 Keychain 对接（API Key 不入仓）。
	•	monitoring：提供 /health 与 /metrics。
	•	sender-wechat：以文件传输助手/公众号等方式低侵入分发。

⸻

目录结构

ai-chat-summary/
├─ Cargo.toml                # workspace（虚拟根，无 [package]）
├─ config.example.toml       # 配置占位（默认时间窗口/接收人/端口/模型名）
├─ apps/
│  └─ cli/                   # 可执行入口（打印 + 启动 /health,/metrics）
├─ monitoring/               # 监控与健康检查（Axum）
├─ extractor-json/           # JSON 数据源适配
├─ extractor-wechat/         # WeChat 本地库适配（WCDB/SQLCipher）
├─ preprocess/               # 过滤、分块
├─ ai/                       # Provider Trait（Map/Reduce/Refine）
├─ ai-deepseek/              # DeepSeek Provider 实现（Keychain 设计）
├─ orchestrator/             # 编排（面向接口编程）
├─ storage/                  # 输出 Markdown / JSON
└─ sender-wechat/            # 分发通道实现


⸻

环境要求
	•	macOS（Apple Silicon, arm64）
	•	Rust 工具链（建议 rustc >= 1.88）
	•	Xcode Command Line Tools：xcode-select --install
	•	（可选）Homebrew 与监听器：
	•	watchexec：brew install watchexec 或 cargo install watchexec-cli --locked
	•	或 cargo-watch：brew install cargo-watch（或 cargo install cargo-watch --no-default-features）

⸻

快速开始
	1.	构建与运行

cargo build --workspace
cargo run -p cli


	2.	健康检查

curl -s localhost:4188/health    # 期望输出：ok
curl -s localhost:4188/metrics   # 期望输出：metrics_placeholder


	3.	（可选）监听开发

# 在项目根目录执行，忽略编译产物与输出目录，避免频繁重启
watchexec --restart --watch . --ignore target --ignore output -- cargo run -p cli
# 或：cargo watch -x "run -p cli"



⸻

配置（占位文件）

仓库根提供 config.example.toml（示例字段如下，实际读取将通过 config 模块与 Keychain 对接）：

window_default_hours = 24
receiver_prefer = "filehelper"       # 文件传输助手
receiver_fallback = "your_official_account"
monitor_port = 4188
model_provider = "deepseek"
model_name = "deepseek-chat"
model_base_url = "https://api.deepseek.com"

安全提示：API Key 将存储在 macOS Keychain，不会写入仓库或明文配置。

⸻

数据源说明（概览）
	•	WeChat 本地库（macOS 客户端）：
常见路径（4.0+）：~/Library/Containers/com.tencent.xinWeChat/Data/Documents/xwechat_files/<wxid_...>/db_storage/message/
典型库：message_0.db、message_1.db、message_fts.db 等（WCDB/SQLCipher）。
提取逻辑封装在 extractor-wechat，不会将底层表结构外泄到上游。
	•	JSON 数据源：
用于本地调试与离线回放，结构化输入消息，便于端到端验证。

⸻

隐私与安全
	•	默认本机执行：所有处理在本地完成，外部模型调用需显式启用。
	•	密钥安全：API Key 仅在 Keychain 中存取，不落盘、不入库、不入日志。
	•	权限最小化：访问 WeChat 本地库需用户在系统设置中授予“完全磁盘访问”，可随时撤回。
	•	日志与产物：请在使用前评估是否包含敏感信息，必要时进行脱敏与访问控制。

⸻

贡献
	•	提交前建议运行：cargo fmt && cargo clippy && cargo build --workspace
	•	Commit 前缀建议：feat: / fix: / chore: / docs:
	•	新增子 crate 时使用：cargo new <name> --lib --vcs none（避免生成内嵌 Git 仓库）

⸻

许可证

待定（建议 MIT 或 Apache-2.0）。在明确许可证前，请勿分发含敏感数据的构建产物。
