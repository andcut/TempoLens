# PGN formats

TempoLens expects PGN clock comments such as:

- Lichess: `{ [%clk 0:02:31] }`
- Chess.com: `{ [%clk 2:31] }`

The parser searches comments for `[%clk H:MM:SS]` or `[%clk M:SS]`.
