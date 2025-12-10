# Contributing to ownkey

Thank you for your interest in contributing! This project is a learning-friendly CLI password manager. Contributions that improve clarity, safety, and maintainability are very welcome.

## How to get started

1. **Fork and clone**
   - Fork this repository to your own GitHub account.
   - Clone your fork locally and create a feature branch.

2. **Set up your environment**
   - Install the latest stable Rust toolchain (`rustup` recommended).
   - Run the test suite to ensure everything works on your machine:
     ```bash
     cargo test
     ```

3. **Pick a task**
   - Check `TASKS.md` for suggested tasks and roadmap items.
   - You can also propose your own improvements via an issue.

4. **Implement the change**
   - Follow the existing code style and module layout.
   - Prefer small, focused changes over large refactors.
   - Avoid introducing new dependencies unless necessary and discussed.

5. **Add or update tests**
   - For CLI behavior, add integration tests under `tests/`.
   - For library code (e.g., `vault`, `vault_store`), add unit tests inside the module or in a dedicated test file.
   - Make sure `cargo test` passes before opening a pull request.

6. **Open a Pull Request**
   - Clearly describe the motivation and what was changed.
   - Link to any relevant issues or tasks from `TASKS.md`.
   - If you change user-facing behavior (CLI flags, error messages, etc.), update `README.md` accordingly.

## Code style and guidelines

- Use Rust 2021 edition idioms.
- Prefer explicit error handling (`anyhow::Result`, `thiserror`) over `unwrap`/`expect` in production code.
- Keep user-facing error messages clear and actionable.
- Do not introduce panics in normal control flow; CLI should fail gracefully with a non-zero exit code.
- Keep functions small and focused when possible.

## Security considerations

ownkey handles sensitive data. When contributing, please:

- Avoid logging secrets or raw passwords.
- Be careful when changing encryption, key derivation, or file format code.
- Document any behavior that could affect security in `SECURITY.md`.
- If you believe you have found a vulnerability, please follow the guidance in `SECURITY.md` instead of opening a public issue.

## Documentation

- Update `README.md` when user-facing behavior or flags change.
- Keep examples runnable and in sync with the current CLI.
- Larger architectural ideas and future work should go into `ROADMAP.md` or design docs.

Thank you again for helping improve ownkey! Every small improvement—tests, docs, refactors—helps make this a better learning and practical tool.

