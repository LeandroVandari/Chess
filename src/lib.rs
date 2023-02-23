use std::fmt;

// The board. Is wrapped in a struct in order to implement methods.
pub struct Board {
    board: [Option<Piece>; 64],
}


// functions that affect the board
impl Board {
    // return an empty board
    fn empty() -> Self {
        Board { board: [None; 64] }
    }

    // return a board in the starting chess position.
    pub fn new() -> Self {
        let mut board = Board::empty();
        board.add_piece(Piece::Rook(Rook {color: Color::White}), 0);
        board.add_piece(Piece::Knight(Knight {color: Color::White}), 1);
        board.add_piece(Piece::Bishop(Bishop {color: Color::White}), 2);
        board.add_piece(Piece::Queen(Queen {color: Color::White}), 3);
        board.add_piece(Piece::King(King {color: Color::White}), 4);
        board.add_piece(Piece::Bishop(Bishop {color: Color::White}), 5);
        board.add_piece(Piece::Knight(Knight {color: Color::White}), 6);
        board.add_piece(Piece::Rook(Rook {color: Color::White}), 7);
        for i in 8..=15 {
            board.add_piece(Piece::Pawn(Pawn {color: Color::White} ), i);
        }

        board.add_piece(Piece::Rook(Rook {color: Color::Black}), 63);
        board.add_piece(Piece::Knight(Knight {color: Color::Black}), 62);
        board.add_piece(Piece::Bishop(Bishop {color: Color::Black}), 61);
        board.add_piece(Piece::Queen(Queen {color: Color::Black}), 60);
        board.add_piece(Piece::King(King {color: Color::Black}), 59);
        board.add_piece(Piece::Bishop(Bishop {color: Color::Black}), 58);
        board.add_piece(Piece::Knight(Knight {color: Color::Black}), 57);
        board.add_piece(Piece::Rook(Rook {color: Color::Black}), 56);
        for i in 48..=55 {
            board.add_piece(Piece::Pawn(Pawn {color: Color::Black} ), i);
        }

        board

    }

    // add a piece to a specific board location
    fn add_piece(&mut self, piece: Piece, square_to_add_piece: usize) {
        self.board[square_to_add_piece] = Some(piece);
    } 
    
}


// Trait which every piece implements. Has only one function, which generates all possible moves for that piece.
trait PieceTrait {
    fn generate_moves(&self, square: u8, board: Board) -> Vec<u8>;
}


// A piece can be black or white.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Color {
    Black, White
}

// Functions to facilitate working with this enum
impl Color {
    // Is the color white?
    fn is_white(&self) -> bool {
        *self == Color::White
    }
}

#[derive(Clone, Copy, Debug)]
// Possible piece types
pub enum Piece {
    Pawn(Pawn),
    Knight(Knight),
    Bishop(Bishop),
    Rook(Rook),
    Queen(Queen),
    King(King),
}

// Each piece may implement different functions.
#[derive(Clone, Copy, Debug)]
pub struct Pawn {color: Color}
#[derive(Clone, Copy, Debug)]
pub struct Knight {color: Color}
#[derive(Clone, Copy, Debug)]
pub struct Bishop {color: Color}
#[derive(Clone, Copy, Debug)]
pub struct Rook {color: Color}
#[derive(Clone, Copy, Debug)]
pub struct Queen {color: Color}
#[derive(Clone, Copy, Debug)]
pub struct King {color: Color}




// Print board to the terminal
impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut board = String::new();
        for (square_counter, square) in self.board.into_iter().enumerate() {
            if let Some(piece) = square {
                match piece {
                    Piece::Pawn(piece) => board.push(if piece.color.is_white() { '♙' } else { '♟' }),
                    Piece::Knight(piece) => board.push(if piece.color.is_white() { '♘' } else { '♞' }),
                    Piece::Bishop(piece) => board.push(if piece.color.is_white() { '♗' } else { '♝' } ),
                    Piece::Rook(piece) => board.push(if piece.color.is_white() { '♖' } else {'♜' }),
                    Piece::Queen(piece) => board.push(if piece.color.is_white() { '♕' } else { '♛' }),
                    Piece::King(piece) => board.push(if piece.color.is_white() { '♔' } else { '♚' }),
                }
            } else {
                board.push('.');
            }
            board.push(' ');
            if square_counter % 8 == 7 {
                board.push('\n');
            }
        }
        write!(f, "{}", board.as_str())
    }
}

