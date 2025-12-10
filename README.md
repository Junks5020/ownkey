# ownkey

一个面向学习者的命令行密码管理器（Key/Value Vault），默认本地加密存储，适合开发者管理 API Keys、Token、环境变量等机密信息。

当前仓库已经实现了完整可用的本地加密 vault、基础 CLI 命令以及可靠的文件读写与权限控制。后续还可以在此基础上继续扩展同步、团队协作等能力。

## 安装与构建

前置要求：

- Rust 稳定版（建议通过 `rustup` 安装）

克隆并构建：

```bash
git clone <this-repo-url> ownkey
cd ownkey
cargo build
```

查看帮助：

```bash
cargo run -- --help
```

运行测试：

```bash
cargo test
```

## 核心特性

- 本地文件加密存储（AES-256-GCM + PBKDF2-HMAC-SHA256）
- 默认 vault 路径：`~/.ownkey/vault.json`（可用 `-p/--path` 覆盖）
- CLI 命令：`init`、`add`、`list`、`view`、`delete`、`search`、隐藏命令 `restore-backup`
- macOS 钥匙串集成（可选）：用 `--keychain-account` 保存主密码
- 会话缓存（可选）：短时间内重复操作无需重复输入密码，可用 `--no-session` 禁用
- 文件锁、原子写和自动备份，防止并发写入和数据损坏

## 快速上手

### 初始化 vault

```bash
# 使用默认路径 ~/.ownkey/vault.json，交互式输入密码
ownkey init

# 指定路径并通过命令行传入密码（注意安全风险）
ownkey init ./vault.json --password testpw
```

初始化时会：

- 创建空 vault（加密格式）
- 设置或确认主密码
- 在 Unix 上把文件权限设置为 `600`
- 同时创建备份 `~/.ownkey/backups/vault.json.bak`

### 添加条目

```bash
# 指定路径和密码，非交互模式
ownkey add --path ./vault.json --password testpw gj_key 123123

# 不提供 value，交互式（无回显）输入并二次确认
ownkey add --path ./vault.json gj_key
```

说明：

- `<key>` 为存储键名，例如 `github_token`
- `<value>` 可省略，省略时会使用隐藏输入并二次确认
- `--notes` 参数暂未持久化，仅为未来扩展预留

### 列出与查看条目

```bash
# 列出所有 key
ownkey list --path ./vault.json --password testpw

# 查看某个 key 的值
ownkey view --path ./vault.json --password testpw gj_key

# 以 JSON 格式输出
ownkey view --path ./vault.json --password testpw --json gj_key
```

### 删除条目

```bash
# 交互式确认删除
ownkey delete --path ./vault.json --password testpw gj_key

# 非交互删除（脚本友好）
ownkey delete --path ./vault.json --password testpw --yes gj_key
```

删除命令会：

- 若 key 不存在：提示 “No entry found for key ...”，命令成功退出（不报错）
- 若 key 存在：在未指定 `--yes` 时进行二次确认，防止误删

### 搜索条目

```bash
# 在 key 和 value 中模糊匹配关键字
ownkey search --path ./vault.json --password testpw gj

# 只输出匹配的 key 名（不带预览）
ownkey search --path ./vault.json --password testpw --exact gj
```

## 密码与安全说明

### 密码输入方式

CLI 支持三种密码来源：

1. `--password <PASSWORD>`：直接通过命令行参数传入  
2. 交互式输入（推荐）：不提供 `--password` 时会在终端以隐藏方式读取  
3. macOS 钥匙串：为 `--keychain-account <ACCOUNT>` 提供账户名时，初始化后主密码会写入/读取系统钥匙串

出于安全考虑，程序会在使用 `--password` 时打印警告，提醒：

- 命令行参数可能出现在 shell 历史、进程列表（`ps`）等位置
- 更推荐留空 `--password`，由程序以隐藏输入方式读取

### Vault 加密格式

vault 文件为 JSON 结构，包含：

- `salt`：Base64 编码的随机盐（16 字节）
- `nonce`：Base64 编码的随机 Nonce（12 字节）
- `ciphertext`：Base64 编码的密文（AES-256-GCM）

主密码经 PBKDF2-HMAC-SHA256（100_000 次迭代）派生出 256-bit 密钥，用于加解密。项目中还有兼容旧明文/`items` 结构的迁移逻辑。

### 文件权限与备份

- 在 Unix 上，每次读写都会校验权限，若不是 `600` 会自动修正并打印告警
- 每次成功写入都会更新备份：`~/.ownkey/backups/vault.json.bak`
- 可使用隐藏命令恢复：

```bash
ownkey restore-backup
```

该命令会用备份覆盖默认 vault 路径，请谨慎使用。

## 开发与测试

代码结构（简要）：

- `src/cli.rs`：命令行参数与子命令定义
- `src/commands/`：各子命令实现（init/add/list/view/delete/search/restore-backup）
- `src/vault.rs`：加密/解密逻辑、密码获取策略、旧格式兼容
- `src/vault_store.rs`：文件锁、原子写、权限控制与备份
- `src/session.rs`：会话级密钥缓存
- `src/keychain.rs`：macOS 钥匙串集成（其他平台会返回错误）
- `tests/`：CLI、可靠性与边界场景测试

运行测试：

```bash
cargo test
```

若你想参与贡献或自定义实现（例如新增 sync provider、HTTP 后端等），请参考仓库中的 `CONTRIBUTING.md` 和 `SECURITY.md`、`ROADMAP.md`。
