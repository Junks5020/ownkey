# ownkey 功能总览

一个面向开发者与学习者的本地优先（local‑first）命令行密码管理器，用加密文件保存 Key/Value 类型的机密（如 API Key、Token、环境变量等），并预留同步扩展能力。

---

## 1. 安装与基础使用

### 1.1 环境要求

- Rust 稳定版（建议通过 `rustup` 安装）
- 支持的系统：Unix 系统优先（权限控制与 Keychain 功能在 Unix/macOS 上表现最佳）

### 1.2 构建与运行

```bash
git clone <repo-url> ownkey
cd ownkey

# 构建
cargo build

# 查看 CLI 帮助
cargo run -- --help

# 运行全部测试
cargo test
```

---

## 2. CLI 命令一览

所有命令默认操作的 Vault 文件为：`~/.ownkey/vault.json`，可通过 `-p/--path` 指定自定义路径。

### 2.1 `init` —— 初始化加密 Vault

```bash
ownkey init [PATH] [--password <PASSWORD>] [--keychain-account <ACCOUNT>] [--keychain-service <SERVICE>] [--no-session]
```

- 功能：
  - 创建一个新的加密 Vault 文件。
  - 默认路径：`~/.ownkey/vault.json`。
  - 若目录不存在，会自动创建。
  - Unix 上确保文件权限为 `600`。
- 主要参数：
  - `PATH`：可选，Vault 文件路径；缺省则使用默认路径。
  - `--password`：可选，主密码；未提供时将交互式（隐藏输入）读取。
  - `--keychain-account`：可选，仅 macOS 生效；设置后主密码会存入系统 Keychain。
  - `--keychain-service`：可选，Keychain service 名，默认 `ownkey`。
  - `--no-session`：禁用会话缓存（不在 `~/.ownkey/session` 中缓存派生密钥）。
- 典型用法：
  - `ownkey init`（默认路径，交互式设置密码）
  - `ownkey init ./vault.json --password testpw`

### 2.2 `add` —— 添加条目

```bash
ownkey add [OPTIONS] <KEY> [VALUE]
```

- 功能：
  - 向 Vault 中添加一条键值对。
  - 若 `VALUE` 省略，则交互式（隐藏输入）输入两遍，并确保二次确认一致。
- 主要参数：
  - `-p, --path <PATH>`：可选，自定义 Vault 文件路径。
  - `<KEY>`：必填，键名（如 `github_token`）。
  - `[VALUE]`：可选，值；省略时使用交互式输入。
  - `--notes <NOTES>`：可选，备注字段（当前未持久化，仅为扩展预留）。
  - `--password <PASSWORD>`：可选，主密码；缺省时使用隐藏输入。
  - `--keychain-account` / `--keychain-service` / `--no-session`：同 `init`。
- 行为：
  - 自动确保 Vault 存在且可解密。
  - 成功后在 stdout 输出 `<KEY>`。

### 2.3 `list` —— 列出所有键

```bash
ownkey list [OPTIONS]
```

- 功能：输出 Vault 中所有键名，每行一个。
- 主要参数：
  - `-p, --path <PATH>`：可选，自定义 Vault 路径。
  - `--password` / `--keychain-account` / `--keychain-service` / `--no-session`：同上。
- 行为：
  - Vault 为空时，命令成功退出但不输出任何内容。

### 2.4 `view` —— 查看指定键的值

```bash
ownkey view [OPTIONS] <KEY>
```

- 功能：查看指定键的值。
- 主要参数：
  - `-p, --path <PATH>`：可选。
  - `<KEY>`：必填，要查看的键名。
  - `--password` / `--keychain-account` / `--keychain-service` / `--no-session`：同上。
  - `--json`：以 JSON 格式输出结果，形如 `{"key":"...","value":"..."}`。
- 行为：
  - 键存在：stdout 输出值（或 JSON），命令成功退出。
  - 键不存在：返回错误，stderr 输出 `No entry found for key <KEY>`，进程以非 0 码退出。

### 2.5 `delete` —— 删除条目

```bash
ownkey delete [OPTIONS] <KEY>
```

- 功能：删除指定键。
- 主要参数：
  - `-p, --path <PATH>`：可选。
  - `<KEY>`：要删除的键。
  - `--password` / `--keychain-account` / `--keychain-service` / `--no-session`：同上。
  - `-y, --yes`：跳过交互确认，直接删除（适合脚本）。
- 行为：
  - 键不存在：输出 `No entry found for key <KEY>`，命令成功退出（不视为错误）。
  - 键存在：
    - 无 `--yes`：提示  
      `This will permanently delete key "<KEY>" from <PATH>. Proceed? [y/N]:`  
      只有输入 `y` 或 `yes`（不区分大小写）才执行删除，否则输出 `Delete cancelled.` 并退出。
    - 有 `--yes`：直接删除。
  - 成功删除后输出：`Deleted key <KEY>`。

### 2.6 `search` —— 搜索

```bash
ownkey search [OPTIONS] <KEYWORD>
```

- 功能：
  - 在键名和明文值中进行子串匹配搜索。
- 主要参数：
  - `-p, --path <PATH>`：可选。
  - `<KEYWORD>`：搜索关键字。
  - `--password` / `--keychain-account` / `--keychain-service` / `--no-session`：同上。
  - `--exact`：只输出匹配到的键名；不带该参数时，会输出带值预览（截取前若干字符）。
- 行为：
  - 查到匹配项时成功退出。
  - 未匹配到时也成功退出，只是输出为空（或提示性文案视具体实现）。

### 2.7 `restore-backup`（隐藏命令）

```bash
ownkey restore-backup
```

- 功能：
  - 将 `~/.ownkey/backups/vault.json.bak` 覆盖恢复到默认 Vault 路径 `~/.ownkey/vault.json`。
- 行为：
  - 交互式确认覆盖。
  - 无备份文件时返回错误并提示。

### 2.8 同步相关命令（当前行为占位）

```bash
ownkey sync
ownkey login [--username <NAME>]
ownkey logout
```

- 功能（当前阶段）：
  - `sync`：调用当前选定的 SyncBackend 的 `pull` / `push`，然后输出 `Sync not implemented yet`。
  - `login`：调用 `login()` 后输出 `Sync login not implemented yet`。
  - `logout`：调用 `logout()` 后输出 `Sync logout not implemented yet`。
- 说明：
  - 实际同步行为在 `sync_provider = "local_only"` 时是 No-op。
  - 在 `sync_provider = "file"` 时会使用 `FileSyncBackend`，已经具备本地文件级 “远端备份” 能力（见下）。

---

## 3. Vault 加密与安全模型

### 3.1 加密格式

Vault 文件为 JSON 结构：

```json
{
  "salt": "<base64>",
  "nonce": "<base64>",
  "ciphertext": "<base64>"
}
```

- `salt`：随机 16 字节，经 Base64 编码。
- `nonce`：随机 12 字节，经 Base64 编码。
- `ciphertext`：使用 AES-256-GCM 加密 Vault 内容后的密文，Base64 编码。

主密码经过 PBKDF2‑HMAC‑SHA256（10 万次迭代）派生出 32 字节密钥，用于 AES‑256‑GCM 加解密。

### 3.2 旧格式兼容

- 若文件无法解析为 `EncryptedVault`，会尝试解析为旧版纯 JSON 结构：
  - 若是简单 `Vault` 结构则直接加载。
  - 若是 `{ "items": [...] }`，则尝试从中迁移出 `name/secret` 字段，构建新的 `Vault`。
- 迁移失败时会报错并附带提示“Vault 文件可能损坏或截断”。

### 3.3 密码获取策略

优先级：

1. CLI 参数 `--password <PASSWORD>`；
2. macOS Keychain：若提供 `--keychain-account`，尝试从 Keychain 读取；
3. 交互式输入：使用 `rpassword::prompt_password` 进行隐藏输入。

并且：

- 若使用 `--password`，程序会在 stderr 输出警告：
  - 提示命令行参数可能出现在 shell 历史及进程列表中；
  - 建议优先使用交互式输入。

### 3.4 会话缓存

- 为减少频繁输入密码，程序可以将派生密钥缓存到 `~/.ownkey/session` 文件中，并带有 TTL（例如 5 分钟）。
- 缓存与具体 Vault 路径关联；路径不同则不复用。
- 使用 `--no-session` 可禁用缓存，强制每次重新输入密码与派生密钥。

### 3.5 文件权限与备份

- 每次读写 Vault 时：
  - 使用加锁文件读写，防止并发访问破坏数据；
  - 写入采用临时文件 + `fsync` + `rename` 的原子写模式；
  - Unix 上强制将 Vault 文件权限修正为 `600`，若不符合会输出告警并自动修正。
- 备份：
  - 每次成功写入 Vault 时，会在 `~/.ownkey/backups/vault.json.bak` 生成/更新备份文件；
  - `restore-backup` 命令即从该备份覆盖默认 Vault。

---

## 4. 配置与同步后端

### 4.1 配置文件：`~/.ownkey/config.toml`

首次运行时若文件不存在会自动生成模板：

```toml
# ownkey configuration
#
# sync_provider controls how vault sync works.
# Supported values:
#   "local_only" - no remote sync (default)
#   "file"       - sync to a local/remote file path
#   "http"       - sync via HTTP backend

sync_provider = "local_only"
```

- `sync_provider`：
  - `"local_only"`：只使用本地 Vault，不做任何远端同步。
  - `"file"`：启用文件级同步后端（FileSyncBackend）。
  - `"http"`：预留，尚未实现，当前仅打印 warning 并回退到 `local_only`。

程序启动时会尝试加载该配置，失败则打印警告并使用默认配置。

### 4.2 SyncBackend trait

位于 `src/sync/backend.rs`：

```rust
pub trait SyncBackend: Send + Sync {
    fn is_logged_in(&self) -> bool;
    fn login(&self, username: Option<&str>) -> Result<(), SyncError>;
    fn logout(&self) -> Result<(), SyncError>;
    fn pull(&self) -> Result<Option<Vec<u8>>, SyncError>;
    fn push(&self, encrypted_blob: &[u8]) -> Result<(), SyncError>;
}
```

### 4.3 NoopSyncBackend（local_only）

- 实现在 `src/sync/noop.rs`。
- 所有方法直接返回成功（`pull` 返回 `Ok(None)`）。
- 在 `sync_provider = "local_only"` 时使用，保证 sync 命令存在但不做实际工作。

### 4.4 FileSyncBackend（file）

实现于 `src/sync/file.rs`：

- 路径：
  - 本地 Vault：
    - 默认使用 `vault_store::default_vault_path()`，即 `~/.ownkey/vault.json`；
    - 测试中也支持传入自定义路径。
  - 远端 Vault：
    - 默认使用 `~/.ownkey/remote_vault.json`。
- 行为：
  - `login`：
    - 创建远端目录；
    - 若远端文件不存在：
      - 若本地 Vault 存在，则复制本地 Vault 作为初始远端；
      - 否则创建一个空 JSON `{}`。
  - `is_logged_in`：
    - 远端文件存在即视为 “已登录”。
  - `push`：
    - 要求本地 Vault 存在，否则返回 `PushFailed("local vault does not exist ...")`；
    - 将本地 Vault 复制为远端文件。
  - `pull`：
    - 若远端文件不存在：返回 `Ok(None)`；
    - 若存在：复制远端文件覆盖本地 Vault，并返回本地文件内容 `Ok(Some(Vec<u8>))`。
  - `logout`：
    - 当前为 No-op（可在未来扩展为清理远端或状态的操作）。

### 4.5 HTTP Backend（预留）

- 当 `sync_provider = "http"` 时：
  - `select_backend()` 会打印 warning：“http sync backend is not implemented yet, falling back to local_only”；
  - 实际仍使用 `NoopSyncBackend`。

---

## 5. 错误处理与退出码

- 顶层 `main` 函数：
  - 调用 `run()`，若返回 `Err`，则在 stderr 打印 `Error: <消息>`，并以退出码 `1` 退出。
  - 正常完成时以退出码 `0` 退出。
- 生产代码中不使用 `unwrap`/`expect` 进行主流程控制：
  - 错误均通过 `anyhow::Result` 或 `SyncError` 进行传播。
  - 极少数量的 `unwrap` 仅存在于测试或某些极小范围辅助代码中。
- 错误信息力求：
  - 简洁明了，例如：
    - `Vault not found. Run 'ownkey init' to create a new encrypted vault.`
    - `Vault password is incorrect or vault is corrupted.`
    - `Vault file appears damaged or truncated. A backup copy may be available.`

---

## 6. 测试覆盖概览

主要测试文件（位于 `tests/`）：

- `cli_smoke.rs`：
  - 确保 `ownkey --help` 可以成功运行并输出基本帮助信息。
- `vault_flow.rs`：
  - `init_creates_empty_vault_file`：`init` 创建加密 Vault。
  - `add_persists_new_item`：`add` 后 `view` 能读到正确值。
  - `list_shows_items_after_add`：`list` 在添加后正确列出键。
  - `view_displays_existing_item`：`view` 已存在 key 成功。
  - `delete_removes_item`：`delete` 后 `view` 返回失败。
  - `search_finds_matching_item`：`search` 能找出匹配键。
- `reliability.rs`：
  - `default_path_is_created`：默认路径的 Vault 会被正确创建。
  - `permissions_are_600`：初始化后的 Vault 权限为 `600`。
- `sync_test.rs`：
  - `test_sync_commands_exist`：`sync/login/logout` 命令存在且返回成功，并输出占位消息。
  - `test_sync_backend_trait_basic_behavior`：`NoopSyncBackend` 满足基本 trait 行为。
  - `test_sync_flow_invokes_backend`：验证 `SyncBackend` trait 的调用流程。
  - `file_sync_login_creates_remote_and_is_logged_in`：验证 file backend 的 `login` 行为。
  - `file_sync_push_and_pull_round_trip`：验证 push/pull 本地与远端内容往返。
- `cli_edge_cases.rs`：
  - `list_succeeds_on_empty_vault`：空 Vault 上 `list` 仍成功且无输出。
  - `view_nonexistent_key_returns_error`：不存在 key 时 `view` 应失败并输出错误信息。
  - `wrong_password_is_rejected`：使用错误密码 `view` 会失败并提示密码错误或 Vault 损坏。
  - `corrupted_vault_file_causes_failure`：损坏文件时命令应失败并提示格式错误。
- `src/vault.rs` 内部单元测试：
  - 加密/解密的 round‑trip 测试。
  - 错误密码 / 损坏密文 / 损坏文件的异常路径测试。

---

## 7. 已实现与可扩展方向

- 已实现：
  - 本地加密 Vault（AES‑256‑GCM + PBKDF2）。
  - CLI 命令：`init/add/list/view/delete/search/restore-backup/sync/login/logout`。
  - 密码输入链路：CLI 参数 / Keychain / 交互式隐藏输入。
  - 会话缓存与 Keychain 集成（macOS）。
  - 文件锁、权限修复（600）、原子写与自动备份。
  - 配置系统（`~/.ownkey/config.toml`）及 `sync_provider` 切换。
  - `NoopSyncBackend` 与 `FileSyncBackend`。
  - 覆盖核心流程与边界情况的测试集合。
- 可扩展方向（根据 ROADMAP）：
  - 实现 HTTP/云端 SyncBackend。
  - 增强 CLI 同步命令（真正完成 login/logout/sync 的业务逻辑）。
  - 增加更多平台的安全存储集成（Windows/Linux）。
  - Web Dashboard、团队协作、审计日志等更高层功能。

