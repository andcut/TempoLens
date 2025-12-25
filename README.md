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
