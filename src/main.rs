use chess::Board;

fn main() {
    let board = Board::new();
    let movements = board.possible_movements();
    println!("{board}");
    println!("{movements:?}");

    /* let mut board = Board::empty();
    board.add_piece(Piece { variant: PieceTypes::Knight, color: Color::White }, 3);
    board.add_piece(Piece { variant: PieceTypes::Pawn, color: Color::White }, 9);

    println!("{board}");
    let movements = board.possible_movements();
    println!("{movements:?}"); */
}
