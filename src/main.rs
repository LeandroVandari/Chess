use std::{collections::HashMap};

use chess::evaluate;

fn main() {
    let board = chess::Board::new();
    /*     let all_moves = board.generate_moves(chess::Color::White);
    println!("{all_moves:?}"); */
    let mut positions = HashMap::new();
    evaluate(&board, 5, chess::Color::White, &mut positions);

    println!("{board}");
}
