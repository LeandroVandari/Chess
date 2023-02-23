use std::fmt;

// Pre-computed values for relative squares for each square.
pub static UP: [Option<u8>; 64] = [Some(8), Some(9), Some(10), Some(11), Some(12), Some(13), Some(14), Some(15), Some(16), Some(17), Some(18), Some(19), Some(20), Some(21), Some(22), Some(23), Some(24), Some(25), Some(26), Some(27), Some(28), Some(29), Some(30), Some(31), Some(32), Some(33), Some(34), Some(35), Some(36), Some(37), Some(38), Some(39), Some(40), Some(41), Some(42), Some(43), Some(44), Some(45), Some(46), Some(47), Some(48), Some(49), Some(50), Some(51), Some(52), Some(53), Some(54), Some(55), Some(56), Some(57), Some(58), Some(59), Some(60), Some(61), Some(62), Some(63), None, None, None, None, None, None, None, None];
pub static DOWN: [Option<u8>; 64] = [None, None, None, None, None, None, None, None, Some(0), Some(1), Some(2), Some(3), Some(4), Some(5), Some(6), Some(7), Some(8), Some(9), Some(10), Some(11), Some(12), Some(13), Some(14), Some(15), Some(16), Some(17), Some(18), Some(19), Some(20), Some(21), Some(22), Some(23), Some(24), Some(25), Some(26), Some(27), Some(28), Some(29), Some(30), Some(31), Some(32), Some(33), Some(34), Some(35), Some(36), Some(37), Some(38), Some(39), Some(40), Some(41), Some(42), Some(43), Some(44), Some(45), Some(46), Some(47), Some(48), Some(49), Some(50), Some(51), Some(52), Some(53), Some(54), Some(55)];
pub static LEFT: [Option<u8>; 64] = [None, Some(0), Some(1), Some(2), Some(3), Some(4), Some(5), Some(6), None, Some(8), Some(9), Some(10), Some(11), Some(12), Some(13), Some(14), None, Some(16), Some(17), Some(18), Some(19), Some(20), Some(21), Some(22), None, Some(24), Some(25), Some(26), Some(27), Some(28), Some(29), Some(30), None, Some(32), Some(33), Some(34), Some(35), Some(36), Some(37), Some(38), None, Some(40), Some(41), Some(42), Some(43), Some(44), Some(45), Some(46), None, Some(48), Some(49), Some(50), Some(51), Some(52), Some(53), Some(54), None, Some(56), Some(57), Some(58), Some(59), Some(60), Some(61), Some(62)];
pub static RIGHT: [Option<u8>; 64] = [Some(1), Some(2), Some(3), Some(4), Some(5), Some(6), Some(7), None, Some(9), Some(10), Some(11), Some(12), Some(13), Some(14), Some(15), None, Some(17), Some(18), Some(19), Some(20), Some(21), Some(22), Some(23), None, Some(25), Some(26), Some(27), Some(28), Some(29), Some(30), Some(31), None, Some(33), Some(34), Some(35), Some(36), Some(37), Some(38), Some(39), None, Some(41), Some(42), Some(43), Some(44), Some(45), Some(46), Some(47), None, Some(49), Some(50), Some(51), Some(52), Some(53), Some(54), Some(55), None, Some(57), Some(58), Some(59), Some(60), Some(61), Some(62), Some(63), None];
pub static DOWN_RIGHT: [Option<u8>; 64] = [None, None, None, None, None, None, None, None, Some(1), Some(2), Some(3), Some(4), Some(5), Some(6), Some(7), None, Some(9), Some(10), Some(11), Some(12), Some(13), Some(14), Some(15), None, Some(17), Some(18), Some(19), Some(20), Some(21), Some(22), Some(23), None, Some(25), Some(26), Some(27), Some(28), Some(29), Some(30), Some(31), None, Some(33), Some(34), Some(35), Some(36), Some(37), Some(38), Some(39), None, Some(41), Some(42), Some(43), Some(44), Some(45), Some(46), Some(47), None, Some(49), Some(50), Some(51), Some(52), Some(53), Some(54), Some(55), None];
pub static DOWN_LEFT: [Option<u8>; 64] = [None, None, None, None, None, None, None, None, None, Some(0), Some(1), Some(2), Some(3), Some(4), Some(5), Some(6), None, Some(8), Some(9), Some(10), Some(11), Some(12), Some(13), Some(14), None, Some(16), Some(17), Some(18), Some(19), Some(20), Some(21), Some(22), None, Some(24), Some(25), Some(26), Some(27), Some(28), Some(29), Some(30), None, Some(32), Some(33), Some(34), Some(35), Some(36), Some(37), Some(38), None, Some(40), Some(41), Some(42), Some(43), Some(44), Some(45), Some(46), None, Some(48), Some(49), Some(50), Some(51), Some(52), Some(53), Some(54)];
pub static UP_RIGHT: [Option<u8>; 64] = [Some(9), Some(10), Some(11), Some(12), Some(13), Some(14), Some(15), None, Some(17), Some(18), Some(19), Some(20), Some(21), Some(22), Some(23), None, Some(25), Some(26), Some(27), Some(28), Some(29), Some(30), Some(31), None, Some(33), Some(34), Some(35), Some(36), Some(37), Some(38), Some(39), None, Some(41), Some(42), Some(43), Some(44), Some(45), Some(46), Some(47), None, Some(49), Some(50), Some(51), Some(52), Some(53), Some(54), Some(55), None, Some(57), Some(58), Some(59), Some(60), Some(61), Some(62), Some(63), None, None, None, None, None, None, None, None, None];
pub static UP_LEFT: [Option<u8>; 64] = [None, Some(8), Some(9), Some(10), Some(11), Some(12), Some(13), Some(14), None, Some(16), Some(17), Some(18), Some(19), Some(20), Some(21), Some(22), None, Some(24), Some(25), Some(26), Some(27), Some(28), Some(29), Some(30), None, Some(32), Some(33), Some(34), Some(35), Some(36), Some(37), Some(38), None, Some(40), Some(41), Some(42), Some(43), Some(44), Some(45), Some(46), None, Some(48), Some(49), Some(50), Some(51), Some(52), Some(53), Some(54), None, Some(56), Some(57), Some(58), Some(59), Some(60), Some(61), Some(62), None, None, None, None, None, None, None, None];

fn up(square: u8)


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

impl PieceTrait for Pawn {
    fn generate_moves(&self, square: u8, board: Board) -> Vec<u8> {
        
    }
}


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

