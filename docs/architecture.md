# Architecture overview

TempoLens is split into a Rust core library and thin UIs.

## Pipeline

1) Parse PGN and clocks
- `crates/core/src/pgn.rs`

2) Build legal positions and FENs
- `crates/core/src/analysis/position.rs`

3) Derive clock before/think times
- `crates/core/src/clocks.rs`

4) Run engine analysis (UCI)
- `crates/core/src/engine/uci.rs`

5) Compute metrics and labels
- `crates/core/src/analysis/eval.rs`
- `crates/core/src/analysis/time_equity.rs`
- `crates/core/src/analysis/labeling.rs`

6) Aggregate summary
- `crates/core/src/analysis/pipeline.rs`

## Data flow

PGN -> ParsedGame -> PlyRecord -> EngineSummary -> MoveMetrics -> Label -> GameAnalysis
