# AGENTS.md

## Project overview
- `secretik` is a single-binary Rust CLI for encrypting/decrypting secret text, generating random labels, and printing terminal QR codes.
- Core technologies: rust, `clap` cli, `aes-gcm` (AES-256-GCM), `argon2` (Argon2id KDF), `base64`, `qrcode`, `anyhow`.
- High-level architecture is modular but small: command parsing and dispatch in `src/main.rs`, subcommand handlers in `src/cmd/`, I/O helpers in `src/cli/`, crypto primitives/encoding in `src/crypt/`.

## Repository layout
- `src/main.rs`: CLI app entrypoint; parses args and dispatches `encrypt` / `decrypt` / `label` / `qr`.
- `src/cmd/`: user-facing command handlers
- `src/cli/mod.rs`: terminal/file I/O helpers and password prompt/validation.
- `src/crypt/mod.rs`: encryption/decryption implementation and base64 packing format.
- `src/crypt/tests.rs`: round-trip crypto tests.
- `Makefile`: local convenience targets (`test`, `build`, `install`, `fmt`, `lint`).

## Local development
- Prerequisites:
  - Rust toolchain (stable).
  - For Linux ARM cross-compilation: `gcc-aarch64-linux-gnu` (also installed in release workflow).
- Setup:
  - Install dependencies via Rust toolchain only; there is no additional service setup or `.env` file.
- Run locally:
  - `cargo run -- <command>`
  - Examples:
    - `cargo run -- encrypt "my secret"`
    - `cargo run -- decrypt "BASE64..."`
    - `cargo run -- qr "text"`
- Convenience commands:
  - `make build`
  - `make test`
  - `make fmt`
  - `make lint`
  - `make install`
- Environment/config notes:
  - No runtime environment variables or config files are required.
  - Password input is interactive (hidden prompt via `rpassword`).

## Coding conventions
- Error handling uses `anyhow::Result` and explicit `bail!` for user-facing validation errors.
- CLI definitions use `clap` derive structs with doc comments and aliases (`#[clap(visible_aliases = ...)]`).
- Modules are split by concern (command handler vs I/O helper vs crypto), with minimal cross-module coupling.
- Tests are colocated with logic:
  - Internal unit tests inside `src/cmd/encrypt.rs`.
  - Dedicated crypto tests in `src/crypt/tests.rs`.
- Output conventions:
  - Successful command output generally goes to stdout.
  - Failures are wrapped and printed in `main` with colored error text and exit code `1`.

## Agent workflow guidance
- Before editing command behavior, inspect all of:
  - `src/main.rs`
  - `src/cmd/mod.rs`
  - target command file in `src/cmd/`
  - shared helpers in `src/cli/mod.rs` and `src/crypt/mod.rs` if affected.
- Prefer preserving existing flow:
  - parse args in command-specific struct,
  - resolve input (`arg` vs `stdin` / file),
  - execute logic,
  - print/surface errors through `anyhow`.
- When changing crypto or serialization logic, run at least:
  - `cargo test`
  - ideally `cargo test --all-features --all-targets`
- When changing CLI flags/semantics, update command help text and verify README accuracy.
- Keep changes localized to the module responsible for that concern unless refactor is clearly needed.

## Integration points and dependencies
- External crates with behavior impact:
  - `aes-gcm`, `argon2`, `rand`, `base64` (crypto/data format)
  - `clap` (CLI contract)
  - `qrcode` (terminal QR rendering)
  - `rpassword` (interactive password input)
  - `names` (random label generation)
  - `colored` (terminal color output)
- There are no external services/databases/queues/API backends in this repository.
- Important internal boundary:
  - `src/crypt/mod.rs` defines the encrypted payload format used by CLI commands: `nonce || salt || ciphertext`, then base64-encoded.

## Recommended first reads
- `README.md`
- `Cargo.toml`
- `Makefile`
- `src/main.rs`
- `src/cmd/mod.rs`
- `src/cmd/encrypt.rs`
- `src/cmd/decrypt.rs`
- `src/cli/mod.rs`
- `src/crypt/mod.rs`

## Rules for future agents
- NEVER change crypto parameters or payload layout in `src/crypt/mod.rs` without adding/adjusting tests.
- Reuse existing helpers in `src/cli/mod.rs` and `src/cmd/mod.rs` before introducing new input-handling paths.
- Run relevant checks before finishing: at minimum `cargo test`; for CI parity run fmt/clippy/test CI commands.
- Keep CLI UX changes synchronized across clap docs/help text and `README.md`.
- Keep module boundaries intact: command orchestration in `src/cmd`, shared I/O in `src/cli`, cryptographic primitives in `src/crypt`.
- Avoid adding dependencies unless the existing crate set cannot reasonably support the change.
