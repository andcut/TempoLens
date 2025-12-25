# Metrics and formulas

## Evaluation normalization
- Engine scores are normalized to White's perspective.
- `cp_white` > 0 means White is better.

## Time equity
- `v(T_total) = alpha / (T_total + beta)`
- `tau_white_pawns = v(T_total) * (t_white - t_black) * phase_multiplier`
- `tau_white_cp = 100 * tau_white_pawns`

## Practical evaluation
- `cp_practical = cp_eval + tau_white_cp`

## Win probability
- `p = 1 / (1 + exp(-k * (cp / 100)))`

## Labels
Rule-based labels are derived from:
- think time
- complexity proxy (spread + punish)
- practical delta
