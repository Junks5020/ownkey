# ownkey Roadmap

This document tracks planned and in-progress work for the ownkey project. It mirrors and expands on `TASKS.md`.

## Phase 1 — Local CLI (MVP)

Status: **in progress**

- [x] Task 1: Improve CLI command UX
  - Help messages and usage examples.
  - Delete confirmation and graceful error messages.
- [x] Task 2: Strengthen CLI security
  - Hidden password input.
  - Encrypted vault format.
  - File permissions enforced to `600`.
  - Warnings when using `--password` on the command line.
- [x] Task 3: Error handling and stability
  - Replace `unwrap`/`expect` in production code.
  - Centralized error reporting and exit codes.
- [x] Task 4: Test coverage
  - Unit tests for vault encryption/decryption.
  - CLI integration tests for happy paths and edge cases.
- [ ] Task 5: Documentation
  - This file (`ROADMAP.md`).
  - `README.md` with install & usage.
  - `CONTRIBUTING.md`, `SECURITY.md`.

## Phase 2 — Sync Foundation

Goal: add a configurable sync layer while keeping encryption client-side.

Planned tasks (aligned with `TASKS.md` 6–11):

- [ ] Config support (`~/.ownkey/config.toml`)
  - Define a `Config` struct.
  - Load or create a default config file on startup.
  - Support `sync_provider = "local_only" | "file" | "http"`.
- [ ] SyncBackend trait and implementations
  - Finalize `SyncBackend` trait design (login/logout/is_logged_in/pull_vault/push_vault).
  - Improve `SyncError` to cover network, file, and auth errors.
  - Implement `NoopSyncBackend` for `local_only` (already partially available).
- [ ] FileSyncBackend
  - For `sync_provider = "file"`:
    - `login`: validate path and create `remote_vault.json` if needed.
    - `logout`: optional clean-up/no-op.
    - `push_vault`: copy local `vault.json` to remote path.
    - `pull_vault`: overwrite local `vault.json` with remote.
- [ ] CLI sync commands
  - `ownkey login`, `ownkey logout`, `ownkey sync`.
  - Wire CLI to selected `SyncBackend` according to config.
  - Start with placeholders that return clear “Not implemented yet” messages, then implement real flows.
- [ ] Sync tests
  - Unit tests for `FileSyncBackend`.
  - Integration tests for `ownkey sync` end-to-end behavior.
  - Edge cases: missing files, unwritable paths, not logged in.

## Phase 3 — Cloud & Multi-Device (Future)

These items are aspirational and may become a separate service/repo:

- [ ] Define a simple HTTP sync protocol for encrypted vault blobs.
- [ ] Implement a reference cloud backend (REST API + DB).
- [ ] Add auth (JWT, OAuth/SSO) and device pairing.
- [ ] Add conflict resolution rules and version history.

## Phase 4 — Team & Dashboard (Future)

- [ ] Web dashboard for browsing and managing secrets.
- [ ] Team vaults with role-based permissions.
- [ ] Audit logs and secret rotation workflows.

## Contribution & tracking

- Small, self-contained tasks are listed in `TASKS.md`.
- Larger, architectural or multi-step efforts should be reflected here.
- If you propose substantial changes (e.g., new sync backend, new crypto primitives), please:
  - Open an issue describing the design and rationale.
  - Update `ROADMAP.md` once the direction is agreed upon.

This roadmap is intentionally lightweight. It is meant to help contributors and users see where ownkey is headed without being a strict commitment.

