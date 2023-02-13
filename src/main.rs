use chess::{Board, Color, Piece, PieceTypes};

fn main() {
    /*     let board = Board::new();
    let movements = board.possible_movements();
    println!("{board}");
    println!("{movements:?}"); */
    Board::start_program();
    let board = Board::example();

    println!("{board}");
    let movements = board.possible_movements();
    println!("{movements:?}");
}
