use std::collections::HashSet;

use chess::evaluate;

fn main() {
    let board = chess::Board::example();
    /*     let all_moves = board.generate_moves(chess::Color::White);
    println!("{all_moves:?}"); */
    let mut positions = HashSet::with_capacity(10000);

    evaluate(&board, 1, chess::Color::Black, &mut positions);
    positions.iter().for_each(|position| {
        let mut board = chess::Board::new();
        board.board = *position;
        println!("{board}")
    })
}
