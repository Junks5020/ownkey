# ownkey — Vision Document

## Overview
ownkey is a local-first, end-to-end encrypted key & password management tool designed for developers. It provides a minimal, transparent, and extensible CLI experience while enabling optional cloud synchronization for users and teams.

ownkey follows an Open Core model:
- **Open Source (Free)** — Fully local, encrypted password/key vault.
- **Cloud (Paid)** — Secure multi-device sync, team sharing, and collaboration features.

**Core principle:** Your secrets belong to you. Encryption stays on the client. The server sees only ciphertext.

## Why ownkey?
Existing password managers often fall into extremes:
- **Closed-source SaaS** — hard to audit, vendor lock-in, not developer-oriented.
- **Self-hosted OSS** — powerful but complex to deploy; not always cleanly local-first.

Developers need to manage API keys, env secrets, tokens, SSH keys, per-project credentials, and shared secrets. Most tools are either heavy or not truly local-first. ownkey fills the gap: open-source, developer-friendly, local-first, with optional cloud sync.

## Core Principles
1. **Local-First**
   - All crypto on the client; cloud sync optional and end-to-end encrypted; fully offline capable.
2. **Developer-Centric**
   - CLI-first workflow; simple commands (init, add, list, view, delete, search); JSON + encrypted storage; extensible modules/plugins (future).
3. **End-to-End Encryption**
   - Cloud never sees plaintext; master password never leaves device; vault encrypted with PBKDF2/Argon2-derived key.
4. **Open Core**
   - Core free/open; cloud is subscription; users always own and can export vault.

## Future: ownkey Cloud (Optional Paid Service)
Extends CLI with encrypted sync.
- **Personal:** multi-device sync, encrypted storage, version history, web dashboard, secure backup/recovery, GitHub/Google login.
- **Team:** shared vaults, RBAC (Owner/Admin/Member/Read-only), audit logs, secret rotation, org SSO.

### Architecture Principles
- Server stores only ciphertext; client holds keys.
- Conflict resolution via optimistic version numbers.
- Multiple sync providers (official cloud + self-hosted).

## Open Core Model

| Feature                         | Open Source (Local) | Cloud (Paid) |
|---------------------------------|---------------------|--------------|
| Local encrypted vault           | ✔                   | ✔            |
| CLI tools                       | ✔                   | ✔            |
| Master password & PBKDF2/Argon2 | ✔                   | ✔            |
| macOS Keychain integration      | ✔                   | ✔            |
| Sync across devices             | ✘                   | ✔            |
| Team sharing                    | ✘                   | ✔            |
| Version history                 | ✘                   | ✔            |
| Audit logs                      | ✘                   | ✔            |
| Web dashboard                   | ✘                   | ✔            |
| SSO / GitHub login              | ✘                   | ✔            |
| Automated backups               | ✘                   | ✔            |

## Roadmap
### Phase 1 — Local CLI (MVP)
- ✔ Encrypted vault (AES-256-GCM)
- ✔ Commands: init, add, view, delete, list, search
- ✔ macOS Keychain support
- ✔ Backward-compatible migration logic
- ⬜ Plugin/module-based architecture
- ⬜ Local vault diff + merge logic (prep for sync)

### Phase 2 — Sync Protocol Design
- ⬜ Define `SyncBackend` trait
- ⬜ Implement ownkey login/sync/logout commands
- ⬜ Fake `file://` sync provider for validation
- ⬜ Conflict resolution rules
- ⬜ Metadata + version tagging

### Phase 3 — ownkey Cloud Backend MVP
- ⬜ REST APIs: sign-up, login, get/update vault
- ⬜ JWT auth
- ⬜ Store encrypted vault blobs
- ⬜ Demo cloud service (Docker)

### Phase 4 — Team Collaboration & Dashboard
- ⬜ Web UI (React/Next.js)
- ⬜ Team vaults
- ⬜ Permission system
- ⬜ Vault key sharing via public key crypto
- ⬜ Activity logs

### Phase 5 — Production SaaS
- ⬜ Billing (Stripe/Paddle)
- ⬜ Monitoring & metrics
- ⬜ Hardened security policies
- ⬜ SDKs
- ⬜ Desktop clients (Electron/Tauri)

## Long-Term Vision
Be the simplest, most transparent, developer-friendly secrets manager:
- Open-source core, privacy-first
- CLI-first but not CLI-only
- Fits individuals, teams, enterprises, self-hosted
- Alternative to 1Password/Bitwarden for developer workflows
- Git-like ethos: fully local, or add a remote for collaboration

## Contribution Vision
Code contributions welcome (Rust, backend, frontend).
