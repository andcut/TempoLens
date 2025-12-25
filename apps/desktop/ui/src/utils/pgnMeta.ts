export interface PgnMeta {
  white?: string;
  black?: string;
  date?: string;
  result?: string;
}

export function parsePgnMeta(pgn: string): PgnMeta {
  const extract = (tag: string) => {
    const match = pgn.match(new RegExp(`\\[${tag} "([^"]*)"\\]`));
    return match?.[1];
  };

  return {
    white: extract("White"),
    black: extract("Black"),
    date: extract("Date"),
    result: extract("Result")
  };
}

export function formatPgnLabel(meta: PgnMeta): string {
  const white = meta.white ?? "White";
  const black = meta.black ?? "Black";
  const result = meta.result ?? "*";
  const date = meta.date ? ` (${meta.date})` : "";
  return `${white} vs ${black} · ${result}${date}`;
}
