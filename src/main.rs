use chess::{Board, Color, Piece, PieceTypes};

fn main() {
    /*     let board = Board::new();
    let movements = board.possible_movements();
    println!("{board}");
    println!("{movements:?}"); */

    let mut board = Board::empty();

    board.add_piece(
        Piece {
            variant: PieceTypes::Pawn,
            color: Color::White,
        },
        9,
    );
    board.add_piece(
        Piece {
            variant: PieceTypes::Rook,
            color: Color::Black,
        },
        11,
    );
    board.add_piece(
        Piece {
            variant: PieceTypes::King,
            color: Color::Black,
        },
        10,
    );

    println!("{board}");
    let movements = board.possible_movements();
    println!("{movements:?}");
}
