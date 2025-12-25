# CLI reference

`timelens-cli` analyzes PGNs and emits JSON.

## Basic usage

```bash
cargo run -p timelens-cli -- --engine /path/to/stockfish --pgn game.pgn
```

## Input sources

Provide exactly one source:

- `--pgn PATH`
- `--lichess-user USER`
- `--chesscom-user USER`

Example:

```bash
cargo run -p timelens-cli -- --engine /path/to/stockfish --lichess-user yourname --games 5
```

## Output

- `--output PATH` writes JSON to a file; otherwise stdout.
- Single-game input -> one JSON object.
- Multi-game input -> JSON array of game objects.

## Engine controls

- `--depth N` (default 14)
- `--multipv K` (default 4)
- `--movetime-ms MS` (time per position)
- `--threads N`
- `--hash-mb MB`

## Time modeling

- `--time-control BASE+INC` fallback when PGN lacks TimeControl
- `--alpha` and `--beta` for base time equity
- `--time-pressure-pivot` (default 30s)
- `--time-pressure-scale` (default 8s)
- `--time-pressure-boost` (default 3.0)
- `--k-sigmoid` for win-probability slope

## Caching remote PGNs

- `--cache-dir PATH`
- `--refresh-cache`
- `TIMELENS_CACHE_DIR` env var overrides cache location
