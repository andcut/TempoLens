# TempoLens

TempoLens is an offline chess analysis tool that treats time as a resource and blends engine evaluation with clock equity.

## Workspace layout

- `crates/core`: analysis library (PGN parsing, clocks, engine, metrics)
- `crates/cli`: CLI to analyze PGNs and emit JSON
- `apps/desktop`: Tauri desktop app (stub UI)
- `assets`: sample PGNs and engine binaries (optional)
- `docs`: product and metric notes

## Quick start (CLI)

1. Put a Stockfish binary on disk.
2. Run:

```bash
cargo run -p timelens-cli -- --engine /path/to/stockfish --pgn assets/sample_pgn/lichess_3plus2.pgn
```

The CLI prints JSON analysis to stdout.

If your PGN lacks a `TimeControl` tag, pass a fallback:

```bash
cargo run -p timelens-cli -- --engine /path/to/stockfish --pgn game.pgn --time-control 180+2
```

To fetch recent games from Lichess or Chess.com (cached locally):

```bash
cargo run -p timelens-cli -- --engine /path/to/stockfish --lichess-user yourname --games 5
cargo run -p timelens-cli -- --engine /path/to/stockfish --chesscom-user yourname --games 3
```

Use `TIMELENS_CACHE_DIR` or `--cache-dir` to override the cache location.

## Engine licensing

Stockfish is GPLv3. If you distribute this app with Stockfish binaries, include the license and provide source access. See `docs/licenses/stockfish.md`.

## Docs

- `docs/README.md` index
- `docs/cli.md` CLI reference
- `docs/schema.md` JSON output
- `docs/architecture.md` pipeline overview
- `docs/getting_started.md` walkthrough
- `docs/faq.md` FAQ
- `docs/decisions.md` design decisions
- `CHANGELOG.md` changelog
