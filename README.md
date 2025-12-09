# ownkey

ownkey is a learning-friendly CLI skeleton for a simple key/value vault. It compiles and wires up commands, but leaves the real secure storage/encryption logic as TODOs so you can implement them step by step.

## Getting started

```bash
cargo build
cargo run -- --help
cargo test -- --ignored   # run the pending TDD specs
```

## Commands (all stubbed)

- `ownkey init [file] [--password <pass>]` — scaffold a new vault file (default `~/.ownkey/vault.json`).
- `ownkey add <key> <value> [--password <pass>]` — add a new key/value.
- `ownkey list [--password <pass>]` — list stored pairs.
- `ownkey view <key> [--password <pass>]` — view one key.
- `ownkey delete <key> [--password <pass>]` — delete a key.
- `ownkey search <keyword> [--password <pass>]` — search keys/values.

## What is implemented

- Project structure with clap command parsing.
- Data models using `serde` and `uuid`.
- Vault module with load/save/ensure stubs.
- Separate command handlers under `src/commands/`.

## What you need to build

All real functionality is intentionally left to you:

- Reading/writing the vault file (consider JSON first, then encryption later).
- Implementing add/list/view/delete/search logic against the vault data.
- Input prompts and validation for new items.
- Secure storage considerations (encryption, permissions, backups).

## Suggested next steps

1. Make `vault::ensure_vault_exists` create an empty JSON file with an empty `items` array.
2. Implement `vault::load_vault` and `vault::save_vault` using `serde_json`.
3. Expand `add`/`list`/`view`/`delete`/`search` to handle richer data or encryption.
4. After the basics work, explore encryption and better ergonomics.

## Tests and TDD hints

- `tests/cli_smoke.rs` ensures `ownkey --help` works.
- `tests/vault_flow.rs` contains ignored (pending) specs describing the expected vault behavior. Unignore and implement functions to make them pass one by one.

## Passing custom data to `add`

`add` 支持可选参数，便于快速写入指定字段：

- `--path <file>` 指定 vault 文件，默认 `vault.json`
- 位置参数 `<key> <value>` 分别为键和值（必填）
- `--notes <text>` 添加备注（当前未持久化，可自定扩展）
- `--password <pass>` 直接提供密码（未提供时将交互式提示）
- 未提供 `<value>` 时将交互式安全输入并二次确认

## 可靠性与备份
- 写入使用原子重命名与 fsync，降低中断导致的损坏
- 所有读写对 vault 文件加锁（同一进程间互斥）
- 每次成功写入会生成/更新 `~/.ownkey/backups/vault.json.bak`
- 若主文件损坏，可使用隐藏命令 `ownkey restore-backup` 进行恢复（会覆盖当前 vault）
