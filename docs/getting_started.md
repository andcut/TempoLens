# Getting started

This is a quick end-to-end walkthrough to analyze a PGN and inspect the JSON.

## 1) Prepare an engine

Download Stockfish and note its path:

```
/path/to/stockfish
```

## 2) Analyze a PGN

```bash
cargo run -p timelens-cli -- --engine /path/to/stockfish --pgn assets/sample_pgn/lichess_3plus2.pgn
```

## 3) Save output

```bash
cargo run -p timelens-cli -- --engine /path/to/stockfish --pgn assets/sample_pgn/lichess_3plus2.pgn --output out.json
```

## 4) Use a fallback time control (if missing)

```bash
cargo run -p timelens-cli -- --engine /path/to/stockfish --pgn game.pgn --time-control 180+2
```

## 5) Fetch recent games

```bash
cargo run -p timelens-cli -- --engine /path/to/stockfish --lichess-user yourname --games 3
```
