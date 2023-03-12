use std::{collections::{HashSet}};

use chess::evaluate;

fn main() {
    let board = chess::Board::new();
    /*     let all_moves = board.generate_moves(chess::Color::White);
    println!("{all_moves:?}"); */
    let mut positions = HashSet::new();
    evaluate(&board, 2, chess::Color::White, &mut positions);
    println!("{board}");
}
