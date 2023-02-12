use chess::{Board, Color, Piece, PieceTypes};

fn main() {
    /*     let board = Board::new();
    let movements = board.possible_movements();
    println!("{board}");
    println!("{movements:?}"); */

    let board = Board::example();

    println!("{board}");
    let movements = board.possible_movements();
    println!("{movements:?}");
}
