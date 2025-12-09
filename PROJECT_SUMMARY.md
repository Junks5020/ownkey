# ownkey 项目概要

## 项目定位
学习友好的命令行键值密码管理器骨架，默认加密存储，提供可扩展的模块化结构，便于继续完善安全和交互逻辑。

## 功能概览
- 命令：`init`（默认 `vault.json`，可指定密码并存入 macOS 钥匙串）、`add <key> <value>`、`list`、`view <key>`、`delete <key>`、`search <keyword>`
- 加密：PBKDF2（SHA-256）派生密钥 + AES-256-GCM 加解密，密文 JSON 存储
- 密码获取：支持 `--password` 直传、交互式输入，macOS 可通过 `--keychain-account/--keychain-service` 使用钥匙串
- 兼容旧格式：读取旧的明文/`items` 结构时自动迁移为新 `entries` 映射
- 测试：集成测试覆盖 init/add/list/view/delete/search（加密路径），CLI smoke 测试

## 目录结构
- `src/cli.rs`：命令/参数定义（含密码与钥匙串选项）
- `src/commands/`：各命令处理逻辑
- `src/vault.rs`：加密存储、PBKDF2 派生、旧格式兼容
- `src/keychain.rs`：macOS 钥匙串适配（其他平台提示不支持）
- `src/models.rs`：`Vault` 数据结构（键值映射）
- `tests/`：CLI 与集成测试
- `README.md`：使用说明与参数列表
- `TASKS.md`：已完成功能与后续待办

## 使用示例
- 初始化并存钥匙串：`ownkey init --password testpw --keychain-account myvault`
- 添加键值：`ownkey add gj_key 123123 --keychain-account myvault`（或使用 `--password`/交互输入）
- 查看：`ownkey view gj_key --keychain-account myvault`
- 列表/删除/搜索同理，默认路径 `vault.json`，可用 `-p/--path` 指定文件

## 待扩展方向
- 交互输入与校验（更友好的 `add` 流程）
- 细粒度权限/文件权限设置
- 更多平台的安全存储集成（Windows Credential Manager、Linux Secret Service 等）
- 备份/恢复、版本管理、输出格式优化
