#[derive(Debug)]
struct Board {
    board: [Option<Piece>; 64]
}

impl Board {
    fn empty() -> Board {
        Board {
            board: [None; 64]
        }
    }

    fn new() -> Board {
        let mut board = Self::empty();
        board.board[0] = Some(Piece::new(PieceTypes::Rook, Color::White));
        board
    }
}

#[derive(Clone, Copy, Debug)]
enum PieceTypes {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King
}

#[derive(Clone, Copy, Debug)]
enum Color {
    Black,
    White
}

#[derive(Clone, Copy, Debug)]
struct Piece {
    variant: PieceTypes,
    color: Color
}

impl Piece {
    fn new(variant: PieceTypes, color: Color) -> Piece {
        Piece {variant, color}
    }
}


fn main() {
    let board = Board::new();
    println!("{:?}", board);
}
