# Repository Guidelines

## Project Structure & Module Organization
- Workspace root: `Cargo.toml` defines Rust workspace members.
- Core library: `crates/core` (PGN parsing, clocks, engine UCI, metrics, labels).
- CLI: `crates/cli` (batch analysis, remote PGN fetch, JSON output).
- Desktop (Tauri): `apps/desktop/src-tauri` for commands; UI lives in `apps/desktop/ui`.
- Tests: Rust tests in `crates/core/tests`.
- Assets and docs: `assets/` and `docs/` (including licenses).

## Build, Test, and Development Commands
- `cargo build` builds the Rust workspace.
- `cargo test -p timelens-core` runs core unit/integration tests.
- `cargo run -p timelens-cli -- --engine /path/to/stockfish --pgn game.pgn` analyzes a PGN.
- `cargo run -p timelens-cli -- --engine /path/to/stockfish --lichess-user USER --games 5` fetches recent games.
- UI dev (optional): `npm install` then `npm run dev` in `apps/desktop/ui`.

## Coding Style & Naming Conventions
- Rust: follow `rustfmt` (4-space indentation). Use idiomatic naming (`snake_case`, `CamelCase`).
- TypeScript/JSON: follow existing formatting (2-space indentation, double quotes in JSON).
- Prefer small, focused modules and keep serialization structs in `crates/core/src/model.rs`.

## Testing Guidelines
- Rust tests use the standard test harness. Place new tests under `crates/core/tests`.
- Name tests for behavior (e.g., `parse_and_fen_basic`).
- Run `cargo test -p timelens-core` before submitting core changes.

## Commit & Pull Request Guidelines
- Git history currently has a single commit; no strict convention is established.
- Recommended: short, imperative subject lines (e.g., `Add clock policy inference`).
- PRs should include a clear summary, testing notes, and sample output or screenshots when changing CLI/desktop UX.

## Security & Configuration Tips
- Stockfish is GPLv3; if bundling binaries, include license text and source access (see `docs/licenses/stockfish.md`).
- Engine paths should be supplied via CLI args or settings; do not hardcode local paths.
