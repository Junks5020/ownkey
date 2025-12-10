# ownkey Security Notes

ownkey is a learning-focused CLI password manager that stores secrets in an encrypted local vault. This document explains the current security model, trade-offs, and how to use the tool safely.

## Threat model (current local-only version)

- **Protected**:
  - Secrets at rest are encrypted using AES-256-GCM with a key derived from your master password using PBKDF2-HMAC-SHA256.
  - Vault files are stored with restrictive permissions (`600` on Unix-like systems) and automatically corrected when needed.
  - Atomic writes and backup files reduce the risk of corruption during crashes or power loss.
- **Not protected**:
  - An attacker with full access to your machine and the master password (or the ability to brute-force it) can decrypt the vault.
  - Clipboard contents, terminal scrollback, and shell history may leak secrets if misused.
  - Non-macOS systems currently do not have an integrated OS keychain backend.

## Encryption and key management

- The vault file is an `EncryptedVault` JSON object:
  - `salt`: Base64-encoded random salt (16 bytes).
  - `nonce`: Base64-encoded random nonce (12 bytes).
  - `ciphertext`: Base64-encoded AES-256-GCM ciphertext of the serialized vault data.
- Keys are derived via PBKDF2-HMAC-SHA256 with 100_000 iterations.
- Decryption errors (wrong password, tampering) are reported with user-friendly messages and do not crash the process.

## Password handling

- Master password input:
  - When `--password` is **not** supplied, ownkey uses hidden, interactive input via `rpassword` so the password is not echoed to the terminal.
  - When `--password <PASSWORD>` is used, ownkey prints a warning because the value may appear in:
    - Shell history (e.g., `.bash_history`, `.zsh_history`).
    - Process listings (`ps`, `top`, etc.).
  - **Recommendation**: Prefer interactive input and avoid providing passwords directly on the command line.

- macOS Keychain:
  - If `--keychain-account <ACCOUNT>` is provided, ownkey attempts to store and retrieve the master password in the macOS Keychain.
  - On non-macOS platforms, Keychain operations are not supported and will return errors.

- Session cache:
  - ownkey can cache derived encryption keys in a short-lived session file (`~/.ownkey/session`) to avoid repeated password prompts.
  - The session cache is time-limited and bound to a specific vault path.
  - You can disable session caching per command using `--no-session`.

## File permissions and backups

- Vault file permissions:
  - On Unix, ownkey enforces permissions `0o600` on the vault file.
  - If a different mode is detected, ownkey prints a warning and attempts to fix it.
- Backups:
  - Every successful write also updates a backup at `~/.ownkey/backups/vault.json.bak`.
  - You can restore from backup with `ownkey restore-backup` (this overwrites the current default vault).

## Sync and remote backends

- The current version of ownkey is primarily local-first.
- Sync backends, HTTP/cloud providers, and multi-device scenarios are **not yet fully implemented** in this repository.
- When sync features are added, they must preserve end-to-end encryption: servers should never see plaintext secrets.

## Reporting security issues

If you believe you have found a security vulnerability:

1. **Do not** open a public GitHub issue with sensitive details.
2. Instead, contact the maintainer privately (e.g., via email or a private channel listed in the repository description).
3. Provide as much detail as possible:
   - Steps to reproduce.
   - Affected versions and environment.
   - Potential impact.

We will:

- Acknowledge receipt of your report.
- Work on a fix and coordinate a disclosure timeline when appropriate.

## Safe usage tips

- Choose a strong, unique master password.
- Avoid using `--password` in shared or logged environments.
- Protect your home directory and backup location with proper OS-level permissions.
- Regularly update to the latest version of ownkey to benefit from security fixes and improvements.

This document will evolve as new features (sync, cloud backends, team sharing) are implemented. Please check `ROADMAP.md` for planned security-related work.

