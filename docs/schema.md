# JSON schema (v0)

The CLI returns a JSON object shaped by `timelens-core` models.

## Top-level

When the input contains multiple games, the CLI emits a JSON array of `GameAnalysis`. For single-game inputs, it emits a single object.

```json
[
  {
  "meta": { "white": "...", "black": "...", "result": "...", "platform": "Lichess" },
  "plies": [
    {
      "ply": { "ply_index": 1, "san": "e4", "uci": "e2e4", "mover": "White" },
      "engine_before": { "depth": 14, "lines": [] },
      "metrics": { "cp_eval_before": 12, "cp_practical_after": 20, "dp_practical_mover": 0.02 },
      "label": { "kind": "Neutral", "title": "Neutral" }
    }
  ],
  "summary": {
    "total_plies": 40,
    "time_trouble_rate": 0.18,
    "phase_time_share": { "opening": 0.2, "middlegame": 0.7, "endgame": 0.1 }
  }
  }
]
```

## Notes

- `mover` is `White` or `Black`.
- `cp_*` values are centipawns from White's perspective.
- `summary` includes aggregate stats (time‑trouble, time share, averages).
- See `docs/sample_output.json` for a concrete single-game example.
