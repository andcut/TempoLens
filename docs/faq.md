# FAQ

## Can I bundle Stockfish with TempoLens?
Stockfish is GPLv3. If you distribute a build that includes Stockfish, you must provide the GPL license text and access to the Stockfish source. See `docs/licenses/stockfish.md`.

## Why does Stockfish complain about missing NNUE?
Some Stockfish builds expect an NNUE network file in the working directory. Use an engine build that bundles NNUE, or place the network file where the engine expects it.

## The PGN has no clocks. What happens?
TempoLens can still analyze positions, but time‑based metrics will be missing. Use `--time-control` to provide a base+increment when possible.

## Why are mate scores huge centipawn values?
Mate scores are mapped to large CP values internally to keep the metrics consistent.
