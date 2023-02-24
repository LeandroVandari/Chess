use std::fmt;

// Pre-computed values for relative squares for each square.
pub static UP: [u8; 64] = [8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47, 48, 49, 50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 60, 61, 62, 63, 64, 64, 64, 64, 64, 64, 64, 64];
pub static DOWN: [u8; 64] = [64, 64, 64, 64, 64, 64, 64, 64, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47, 48, 49, 50, 51, 52, 53, 54, 55];
pub static LEFT: [u8; 64] = [64, 0, 1, 2, 3, 4, 5, 6, 64, 8, 9, 10, 11, 12, 13, 14, 64, 16, 17, 18, 19, 20, 21, 22, 64, 24, 25, 26, 27, 28, 29, 30, 64, 32, 33, 34, 35, 36, 37, 38, 64, 40, 41, 42, 43, 44, 45, 46, 64, 48, 49, 50, 51, 52, 53, 54, 64, 56, 57, 58, 59, 60, 61, 62];
pub static RIGHT: [u8; 64] = [1, 2, 3, 4, 5, 6, 7, 64, 9, 10, 11, 12, 13, 14, 15, 64, 17, 18, 19, 20, 21, 22, 23, 64, 25, 26, 27, 28, 29, 30, 31, 64, 33, 34, 35, 36, 37, 38, 39, 64, 41, 42, 43, 44, 45, 46, 47, 64, 49, 50, 51, 52, 53, 54, 55, 64, 57, 58, 59, 60, 61, 62, 63, 64];
pub static DOWN_RIGHT: [u8; 64] = [64, 64, 64, 64, 64, 64, 64, 64, 1, 2, 3, 4, 5, 6, 7, 64, 9, 10, 11, 12, 13, 14, 15, 64, 17, 18, 19, 20, 21, 22, 23, 64, 25, 26, 27, 28, 29, 30, 31, 64, 33, 34, 35, 36, 37, 38, 39, 64, 41, 42, 43, 44, 45, 46, 47, 64, 49, 50, 51, 52, 53, 54, 55, 64];
pub static DOWN_LEFT: [u8; 64] = [64, 64, 64, 64, 64, 64, 64, 64, 64, 0, 1, 2, 3, 4, 5, 6, 64, 8, 9, 10, 11, 12, 13, 14, 64, 16, 17, 18, 19, 20, 21, 22, 64, 24, 25, 26, 27, 28, 29, 30, 64, 32, 33, 34, 35, 36, 37, 38, 64, 40, 41, 42, 43, 44, 45, 46, 64, 48, 49, 50, 51, 52, 53, 54];
pub static UP_RIGHT: [u8; 64] = [9, 10, 11, 12, 13, 14, 15, 64, 17, 18, 19, 20, 21, 22, 23, 64, 25, 26, 27, 28, 29, 30, 31, 64, 33, 34, 35, 36, 37, 38, 39, 64, 41, 42, 43, 44, 45, 46, 47, 64, 49, 50, 51, 52, 53, 54, 55, 64, 57, 58, 59, 60, 61, 62, 63, 64, 64, 64, 64, 64, 64, 64, 64, 64];
pub static UP_LEFT: [u8; 64] = [64, 8, 9, 10, 11, 12, 13, 14, 64, 16, 17, 18, 19, 20, 21, 22, 64, 24, 25, 26, 27, 28, 29, 30, 64, 32, 33, 34, 35, 36, 37, 38, 64, 40, 41, 42, 43, 44, 45, 46, 64, 48, 49, 50, 51, 52, 53, 54, 64, 56, 57, 58, 59, 60, 61, 62, 64, 64, 64, 64, 64, 64, 64, 64];

fn up(square: usize) -> Option<u8> {
    if UP[square] != 64 {Some(UP[square])} else {None}
}
fn down(square: usize) -> Option<u8> {
    if DOWN[square] != 64 {Some(DOWN[square])} else {None}
}
fn left(square: usize) -> Option<u8> {
    if LEFT[square] != 64 {Some(LEFT[square])} else {None}
}
fn right(square: usize) -> Option<u8> {
    if RIGHT[square] != 64 {Some(RIGHT[square])} else {None}
}
fn down_right(square: usize) -> Option<u8> {
    if DOWN_RIGHT[square] != 64 {Some(DOWN_RIGHT[square])} else {None}
}
fn down_left(square: usize) -> Option<u8> {
    if DOWN_LEFT[square] != 64 {Some(DOWN_LEFT[square])} else {None}
}
fn up_left(square: usize) -> Option<u8> {
    if UP_LEFT[square] != 64 {Some(UP_LEFT[square])} else {None}
}
fn up_right(square: usize) -> Option<u8> {
    if UP_RIGHT[square] != 64 {Some(UP_RIGHT[square])} else {None}
}

// The board. Is wrapped in a struct in order to implement methods.
pub struct Board {
    pub board: [Option<Piece>; 64],
}


// functions that affect the board
impl Board {
    // return an empty board
    fn empty() -> Self {
        Board { board: [None; 64] }
    }

    // return a board in the starting chess position.
    pub fn new() -> Self {
        let mut board = Self::empty();
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

    pub fn example() -> Self {
        let mut board = Self::empty();

        board.add_piece(Piece::Pawn(Pawn {color: Color::White}), 9);
        board.add_piece(Piece::Rook(Rook {color: Color::Black}), 18);
        board.add_piece(Piece::King(King {color: Color::Black}), 17);
        board.add_piece(Piece::Knight(Knight {color: Color::White}), 19);
        board.add_piece(Piece::King(King {color: Color::White}), 63);
        board.add_piece(Piece::Queen(Queen {color: Color::White}), 11);
        board.add_piece(Piece::Bishop(Bishop {color: Color::Black}), 2);
        board

    }

    // add a piece to a specific board location
    fn add_piece(&mut self, piece: Piece, square_to_add_piece: usize) {
        self.board[square_to_add_piece] = Some(piece);
    }

    fn get_row(square: u8) -> u8 {
        square / 8
    }
    fn get_column(square: u8) -> u8 {
        square % 8
    }
    
}


// Trait which every piece implements. Has only one function, which generates all possible moves for that piece.
trait PieceTrait {
    fn generate_moves(&self, board: &[Option<Piece>; 64], square: u8) -> Vec<u8>;
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


impl Piece {
    fn get_color(&self) -> Color {
        match *self {
            Piece::Pawn(piece) => piece.color,
            Piece::Knight(piece) => piece.color,
            Piece::Bishop(piece) => piece.color,
            Piece::Rook(piece) => piece.color,
            Piece::Queen(piece) => piece.color,
            Piece::King(piece) => piece.color
        }
    }
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

impl PieceTrait for Pawn {
    fn generate_moves(&self, board: &[Option<Piece>; 64], piece_square: u8) -> Vec<u8> {
        let mut moves = Vec::new();
        let end_square = if self.color.is_white() {up(piece_square as usize)} else {down(piece_square as usize)};
        if let Some(end_square) = end_square {
            if let None = board[end_square as usize] {
                moves.push(end_square);
                if Board::get_row(piece_square) == if self.color.is_white(){1} else {6} {
                    let next_square = if self.color.is_white() {up(end_square as usize).unwrap()} else {down(end_square as usize).unwrap()};
                    if let None = board[next_square as usize] {
                        moves.push(next_square);
                    }
                }
            }
        } 


        moves
    }

}

/* impl PieceTrait for Knight {
    fn generate_moves(&self, board: &[Option<Piece>; 64], square: u8) -> Vec<u8> {
        Vec::new()
    }
    
}
impl PieceTrait for Bishop {
    fn generate_moves(&self, board: &[Option<Piece>; 64], square: u8) -> Vec<u8> {
        Vec::new()
    }
    
}
impl PieceTrait for Rook {
    fn generate_moves(&self, board: &[Option<Piece>; 64], square: u8) -> Vec<u8> {
        Vec::new()
    }
    
}
impl PieceTrait for Queen {
    fn generate_moves(&self, board: &[Option<Piece>; 64], square: u8) -> Vec<u8> {
        Vec::new()
    }
    
}
impl PieceTrait for King {
    fn generate_moves(&self, board: &[Option<Piece>; 64], square: u8) -> Vec<u8> {
        Vec::new()
    }
    
} */

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

