use fnv::FnvHashSet;

use chess::{from_square, multi_thread_eval};

fn main() {
    let board = chess::Board::from_fen("r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R");
/*         .make_move(
            from_square("a1") as usize,
            chess::Move::RegularMove(from_square("b1")),
            chess::Color::White,
        ); */
    /*
         .make_move(
            from_square("f2") as usize,
            chess::Move::RegularMove(from_square("e4")),
            chess::Color::Black,
        )
    .make_move(from_square("a1") as usize, chess::Move::RegularMove(from_square("a2")), chess::Color::White); */

    println!("{board}");
    let mut positions = FnvHashSet::default();
    let depth = 4;
    multi_thread_eval(&board, depth, chess::Color::White, &mut positions);
}
