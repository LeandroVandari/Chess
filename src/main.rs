use fnv::FnvHashSet;

use chess::{from_square, multi_thread_eval};

fn main() {
    let board = chess::Board::new();
        /* .make_move(
            from_square("e2") as usize,
            chess::Move::PawnAdvanceTwoSquares(from_square("e4")),
            chess::Color::White,
        )
        .make_move(
            from_square("f7") as usize,
            chess::Move::PawnAdvanceTwoSquares(from_square("f6")),
            chess::Color::Black,
        ) *///.make_move(from_square("d1") as usize, chess::Move::RegularMove(from_square("h5")), chess::Color::White);

    println!("{board}");
    let mut positions = FnvHashSet::default();
    let depth = 6;
    multi_thread_eval(&board, depth, chess::Color::White, &mut positions);
}
