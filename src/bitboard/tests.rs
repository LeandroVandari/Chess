#![cfg(test)]
#![allow(clippy::unreadable_literal)]

const STARTPOS: super::Position = super::Position::new();
const POSS_MOVE: Option<super::PossiblePieceMoves> = None;
const POSITION: Option<super::Move> = None;
const POSITIONS_LIST: [Option<super::Move>; 219] = [POSITION; 219];

#[test]
fn perft_1() {
    const DEPTH: usize = 1;
    let mut moves_list: [Option<super::PossiblePieceMoves>; 16] = [POSS_MOVE; 16];
    let mut pieces_list: [u64; 16] = [0; 16];
    let mut positions_list_list: [[Option<super::Move>; 219]; DEPTH] = [POSITIONS_LIST; DEPTH];

    assert_eq!(
        STARTPOS.perft(&mut positions_list_list, &mut moves_list, &mut pieces_list),
        20
    );
}

#[test]
fn perft_2() {
    const DEPTH: usize = 2;
    let mut moves_list: [Option<super::PossiblePieceMoves>; 16] = [POSS_MOVE; 16];
    let mut pieces_list: [u64; 16] = [0; 16];
    let mut positions_list_list: [[Option<super::Move>; 219]; DEPTH] = [POSITIONS_LIST; DEPTH];

    assert_eq!(
        STARTPOS.perft(&mut positions_list_list, &mut moves_list, &mut pieces_list),
        400
    );
}

#[test]
fn perft_3() {
    const DEPTH: usize = 3;
    let mut moves_list: [Option<super::PossiblePieceMoves>; 16] = [POSS_MOVE; 16];
    let mut pieces_list: [u64; 16] = [0; 16];
    let mut positions_list_list: [[Option<super::Move>; 219]; DEPTH] = [POSITIONS_LIST; DEPTH];

    assert_eq!(
        STARTPOS.perft(&mut positions_list_list, &mut moves_list, &mut pieces_list),
        8902
    );
}

#[test]
fn perft_4() {
    const DEPTH: usize = 4;
    let mut moves_list: [Option<super::PossiblePieceMoves>; 16] = [POSS_MOVE; 16];
    let mut pieces_list: [u64; 16] = [0; 16];
    let mut positions_list_list: [[Option<super::Move>; 219]; DEPTH] = [POSITIONS_LIST; DEPTH];

    assert_eq!(
        STARTPOS.perft(&mut positions_list_list, &mut moves_list, &mut pieces_list),
        197281
    );
}

#[test]
fn perft_5() {
    const DEPTH: usize = 5;
    let mut moves_list: [Option<super::PossiblePieceMoves>; 16] = [POSS_MOVE; 16];
    let mut pieces_list: [u64; 16] = [0; 16];
    let mut positions_list_list: [[Option<super::Move>; 219]; DEPTH] = [POSITIONS_LIST; DEPTH];

    assert_eq!(
        STARTPOS.perft(&mut positions_list_list, &mut moves_list, &mut pieces_list),
        4865609
    );
}

#[test]
fn perft_6() {
    const DEPTH: usize = 6;
    let mut moves_list: [Option<super::PossiblePieceMoves>; 16] = [POSS_MOVE; 16];
    let mut pieces_list: [u64; 16] = [0; 16];
    let mut positions_list_list: [[Option<super::Move>; 219]; DEPTH] = [POSITIONS_LIST; DEPTH];

    assert_eq!(
        STARTPOS.perft(&mut positions_list_list, &mut moves_list, &mut pieces_list),
        119060324
    );
}

#[test]
#[ignore]
fn perft_7() {
    const DEPTH: usize = 7;
    let mut moves_list: [Option<super::PossiblePieceMoves>; 16] = [POSS_MOVE; 16];
    let mut pieces_list: [u64; 16] = [0; 16];
    let mut positions_list_list: [[Option<super::Move>; 219]; DEPTH] = [POSITIONS_LIST; DEPTH];

    assert_eq!(
        STARTPOS.perft(&mut positions_list_list, &mut moves_list, &mut pieces_list),
        3195901860
    );
}

#[test]
fn position_2() {
    super::macros::perft_for_position_stable!("r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1", [48 2039 97862 4085603 193690690]);
}

#[test]
fn position_3() {
    super::macros::perft_for_position_stable!("8/2p5/3p4/KP5r/1R3p1k/8/4P1P1/8 w - - 0 1", [14 191 2812 43238 674624]);
}

#[test]
fn position_4() {
    super::macros::perft_for_position_stable!("r3k2r/Pppp1ppp/1b3nbN/nP6/BBP1P3/q4N2/Pp1P2PP/R2Q1RK1 w kq - 0 1", [6 264 9467 422333 15833292]);
}

#[test]
fn position_5() {
    super::macros::perft_for_position_stable!("rnbq1k1r/pp1Pbppp/2p5/8/2B5/8/PPP1NnPP/RNBQK2R w KQ - 1 8", [44 1486 62379 2103487 89941194]);
}

#[test]
fn position_6() {
    super::macros::perft_for_position_stable!("r4rk1/1pp1qppp/p1np1n2/2b1p1B1/2B1P1b1/P1NP1N2/1PP1QPPP/R4RK1 w - - 0 10", [46 2079 89890 3894594 164075551]);
}
