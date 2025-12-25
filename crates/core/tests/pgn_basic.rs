use timelens_core::analysis::position::build_ply_records_with_fens;
use timelens_core::pgn::parse_single_game;

#[test]
fn parse_and_fen_basic() {
    let pgn = r#"
[Event "Test"]
[Site "https://lichess.org/xxxx"]
[Date "2025.12.25"]
[Round "-"]
[White "White"]
[Black "Black"]
[Result "*"]
[TimeControl "180+0"]

1. e4 { [%clk 0:03:00] } e5 { [%clk 0:03:00] }
2. Nf3 { [%clk 0:02:58] } Nc6 { [%clk 0:02:59] }
*
"#;

    let game = parse_single_game(pgn).expect("parse");
    assert_eq!(game.plies.len(), 4);

    assert!(game.plies[0].clock_after_secs.unwrap() > 0.0);

    let plies = build_ply_records_with_fens(&game).expect("fen");
    assert_eq!(plies.len(), 4);
    assert!(plies[0]
        .fen_before
        .starts_with("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR"));
    assert!(plies[0].fen_after.contains("4P3"));
}

#[test]
fn parse_clk_allows_single_digit_fields() {
    let pgn = r#"
[Event "Test"]
[Site "https://lichess.org/xxxx"]
[Date "2025.12.25"]
[Round "-"]
[White "White"]
[Black "Black"]
[Result "*"]
[TimeControl "60+0"]

1. e4 { [%clk 0:3:0] } e5 { [%clk 0:2:5] }
*
"#;

    let game = parse_single_game(pgn).expect("parse");
    assert_eq!(game.plies.len(), 2);
    assert_eq!(game.plies[0].clock_after_secs, Some(180.0));
    assert_eq!(game.plies[1].clock_after_secs, Some(125.0));
}
