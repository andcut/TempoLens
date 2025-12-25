# Decisions

## Multi-game PGN handling

- `timelens-cli` accepts multi-game PGN input from files or API fetches.
- Output is a JSON array when more than one game is present.
- Output is a single object when exactly one game is present.

Rationale: preserves a simple default for single games while supporting batch analysis without new flags.

## Chess.com archive pagination

- The CLI now walks archives from newest to oldest until it gathers the requested count.
- Games are pulled from each archive in reverse order so the newest games are selected.
- This avoids silently returning fewer games when the current month has too few games.

## Clock rate metrics

- `time_trouble_rate` uses total plies as the denominator.
- `time_trouble_rate_known` uses only plies with known clocks.

Rationale: avoids hiding missing clock data while still providing a consistent rate.

## TimeControl parsing

- `TimeControl` values without `+` are treated as `base+0`.

Rationale: some PGNs omit explicit increment but still provide base seconds.

## Engine reuse for multi-game analysis

- `analyze_pgns` reuses a single Stockfish process and sends `ucinewgame` between games.

Rationale: reduces startup overhead in batch analysis.

## UI mock analysis behavior

- The desktop UI only uses mock analysis when `VITE_USE_MOCK_ANALYSIS=true`.
- Otherwise it errors if Tauri IPC is unavailable.

Rationale: avoids silently analyzing the sample Opera House game.
