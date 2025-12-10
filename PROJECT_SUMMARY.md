# ownkey 项目概要

## 项目定位
学习友好的命令行键值密码管理器骨架，默认加密存储，提供可扩展的模块化结构，便于继续完善安全和交互逻辑。

## 功能概览（当前状态）
- 命令：`init`（默认 `~/.ownkey/vault.json`，可指定密码并存入 macOS 钥匙串，支持隐藏 `restore-backup`）、`add <key> [value]`（缺省值时交互式输入并确认）、`list`（仅输出 key）、`view <key>`（默认只输出值，`--json` 输出结构化）、`delete <key>`、`search <keyword>`（`--exact` 只输出 key）
- 加密：PBKDF2（SHA-256）派生密钥 + AES-256-GCM，加密存储；兼容旧明文/`items` 结构并自动迁移
- 密码获取：`--password`、交互式输入、macOS 钥匙串；会话缓存可选（可禁用），不改变密文格式
- 可靠性：读写加文件锁；原子写（临时文件+fsync+rename）；自动备份 `~/.ownkey/backups/vault.json.bak`；权限检查并修复为 600（Unix）
- 测试：`cargo test` 全通过；覆盖 CLI smoke、加密流程、默认路径与权限等可靠性测试

## 目录结构
- `src/cli.rs`：命令/参数定义（含密码与钥匙串选项）
- `src/commands/`：各命令处理逻辑
- `src/vault.rs`：加密存储、PBKDF2 派生、旧格式兼容
- `src/vault_store.rs`：文件锁、原子写、备份与权限修复
- `src/session.rs`：会话缓存
- `src/keychain.rs`：macOS 钥匙串适配（其他平台提示不支持）
- `src/models.rs`：`Vault` 数据结构（键值映射）
- `tests/`：CLI 与集成测试
- `README.md`：使用说明与参数列表
- `TASKS.md`：已完成功能与后续待办

## 使用示例
- 初始化并存钥匙串：`ownkey init --password testpw --keychain-account myvault`
- 添加键值：`ownkey add gj_key 123123 --keychain-account myvault`（或省略 value 交互输入；也可用 `--password`/会话缓存）
- 查看：`ownkey view gj_key --json --keychain-account myvault`
- 列表/删除/搜索同理，默认路径 `~/.ownkey/vault.json`，可用 `-p/--path` 指定文件
- 恢复备份（隐藏命令）：`ownkey restore-backup`

## 待扩展方向
- 交互输入与校验（更友好的 `add` 流程）——已实现基础交互，仍可优化体验
- 更多平台的安全存储集成（Windows Credential Manager、Linux Secret Service 等）
- 备份/恢复与版本管理优化（多版本、自动清理策略）
- 同步/团队功能按 VISION/Roadmap 推进
