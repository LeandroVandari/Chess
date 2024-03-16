use chess::bitboard as bb;

/* use fnv::FnvHashSet;

use chess::multi_thread_eval; */

fn main() {
    /* let board = chess::Board::new(); //.make_move(from_square("c2") as usize, chess::Move::RegularMove(from_square("c3")), chess::Color::White).make_move(from_square("a7") as usize, chess::Move::RegularMove(from_square("a5")), chess::Color::Black).make_move(from_square("d1") as usize, chess::Move::RegularMove(from_square("a4")), chess::Color::White);
    println!("{board}");
    let mut positions = FnvHashSet::default();
    let depth = 5;
    multi_thread_eval(&board, depth, chess::Color::White, &mut positions); */

    const DEPTH: usize = 6;
    const OTHER_DEPTH: usize = DEPTH - 1;
/*     const POSS_MOVE: Option<bb::PossiblePieceMoves> = None;
    const POSITION: Option<bb::move_generation::Move> = None;
const POSITIONS_LIST: [Option<bb::move_generation::Move>; 219] = [POSITION; 219]; */

static MAP: once_cell::sync::Lazy<chashmap::CHashMap<bb::Position, u32>> = once_cell::sync::Lazy::new(chashmap::CHashMap::new);

/*     let mut moves_list: [Option<bb::PossiblePieceMoves>; 16] = [POSS_MOVE; 16];
    let mut pieces_list: [u64; 16] = [0; 16];
    let mut positions_list_list: [[Option<bb::move_generation::Move>; 219]; DEPTH] =
        [POSITIONS_LIST; DEPTH]; */

    let board = bb::Position::from_fen(
        "rnbq1k1r/pp1Pbppp/2p5/8/2B5/8/PPP1NnPP/RNBQK2R w KQ - 1 8",
    ).new_with_move(&bb::move_generation::Move::Regular { piece_type: bb::pieces::PieceTypes::Queen, start_square: std::num::NonZeroU64::new(chess::convert::from::algebraic_square::to_bitboard("d1")).unwrap(), end_square: std::num::NonZeroU64::new(chess::convert::from::algebraic_square::to_bitboard("d4")).unwrap() });
   let board = bb::Position::new();

    println!("{board}\n");

    let start = std::time::Instant::now();
    let total_positions = board.multi_thread_perft::<OTHER_DEPTH>(Some(&MAP));
    let time = (std::time::Instant::now() - start).as_millis();
    println!("Amount of positions: {total_positions}\nTime elapsed: {time}ms");
}
