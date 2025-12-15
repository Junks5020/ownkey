# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.1.0] - 2025-12-15

### Added

- **Core CLI Commands**
  - `init` - Initialize an encrypted vault at the default or specified path
  - `add` - Add a key-value entry to the vault (supports interactive hidden input)
  - `list` - List all keys in the vault
  - `view` - View the value of a specific key (supports `--json` output)
  - `delete` - Delete a key with confirmation prompt (use `--yes` to skip)
  - `search` - Search keys and values by keyword
  - `copy` - Copy a value to the system clipboard
  - `rotate-password` - Change the vault master password

- **Security Features**
  - AES-256-GCM encryption with PBKDF2-HMAC-SHA256 key derivation (100,000 iterations)
  - Hidden password input via terminal
  - File permissions enforced to `600` on Unix systems
  - Warnings when using `--password` on command line

- **macOS Integration**
  - Keychain support for storing master password (`--keychain-account`)

- **Reliability**
  - Atomic file writes with file locking
  - Automatic backups to `~/.ownkey/backups/vault.json.bak`
  - `restore-backup` command (hidden) for recovery

- **Session Management**
  - Optional session caching to avoid repeated password prompts
  - Disable with `--no-session` flag

- **Configuration**
  - Config file support at `~/.ownkey/config.toml`
  - Sync provider selection (local_only, file, http placeholder)

- **Sync Framework** (Foundation)
  - `SyncBackend` trait for extensible sync implementations
  - `NoopSyncBackend` for local-only mode
  - `FileSyncBackend` for file-based sync
  - CLI commands `sync`, `login`, `logout` (placeholder implementations)

### Documentation

- README with usage examples
- SECURITY.md with security model explanation
- CONTRIBUTING.md with contributor guidelines
- ROADMAP.md with planned features
- FEATURES.md with detailed feature documentation
