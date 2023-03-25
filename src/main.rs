use fnv::FnvHashSet;

use chess::{from_square, multi_thread_eval};

fn main() {
    let board = chess::Board::from_fen("rnbq1k1r/pp1Pbppp/2p5/8/2B5/8/PPP1NnPP/RNBQK2R");
    /* .make_move(
            from_square("a2") as usize,
            chess::Move::RegularMove(from_square("a3")),
            chess::Color::White,
        )
         .make_move(
            from_square("f2") as usize,
            chess::Move::RegularMove(from_square("e4")),
            chess::Color::Black,
        )
    .make_move(from_square("a1") as usize, chess::Move::RegularMove(from_square("a2")), chess::Color::White); */

    println!("{board}");
    let mut positions = FnvHashSet::default();
    let depth = 5;
    multi_thread_eval(&board, depth, chess::Color::White, &mut positions);
}
