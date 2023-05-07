pub mod board;

use std::fmt;

pub use board::Board;
pub mod search;
use board::CanEnPassant;
pub use search::multi_thread_eval;

// Pre-computed values for relative squares for each square.
pub static UP: [u8; 64] = [
    8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31,
    32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47, 48, 49, 50, 51, 52, 53, 54, 55,
    56, 57, 58, 59, 60, 61, 62, 63, 64, 64, 64, 64, 64, 64, 64, 64,
];
pub static DOWN: [u8; 64] = [
    64, 64, 64, 64, 64, 64, 64, 64, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17,
    18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41,
    42, 43, 44, 45, 46, 47, 48, 49, 50, 51, 52, 53, 54, 55,
];
pub static LEFT: [u8; 64] = [
    64, 0, 1, 2, 3, 4, 5, 6, 64, 8, 9, 10, 11, 12, 13, 14, 64, 16, 17, 18, 19, 20, 21, 22, 64, 24,
    25, 26, 27, 28, 29, 30, 64, 32, 33, 34, 35, 36, 37, 38, 64, 40, 41, 42, 43, 44, 45, 46, 64, 48,
    49, 50, 51, 52, 53, 54, 64, 56, 57, 58, 59, 60, 61, 62,
];
pub static RIGHT: [u8; 64] = [
    1, 2, 3, 4, 5, 6, 7, 64, 9, 10, 11, 12, 13, 14, 15, 64, 17, 18, 19, 20, 21, 22, 23, 64, 25, 26,
    27, 28, 29, 30, 31, 64, 33, 34, 35, 36, 37, 38, 39, 64, 41, 42, 43, 44, 45, 46, 47, 64, 49, 50,
    51, 52, 53, 54, 55, 64, 57, 58, 59, 60, 61, 62, 63, 64,
];
pub static DOWN_RIGHT: [u8; 64] = [
    64, 64, 64, 64, 64, 64, 64, 64, 1, 2, 3, 4, 5, 6, 7, 64, 9, 10, 11, 12, 13, 14, 15, 64, 17, 18,
    19, 20, 21, 22, 23, 64, 25, 26, 27, 28, 29, 30, 31, 64, 33, 34, 35, 36, 37, 38, 39, 64, 41, 42,
    43, 44, 45, 46, 47, 64, 49, 50, 51, 52, 53, 54, 55, 64,
];
pub static DOWN_LEFT: [u8; 64] = [
    64, 64, 64, 64, 64, 64, 64, 64, 64, 0, 1, 2, 3, 4, 5, 6, 64, 8, 9, 10, 11, 12, 13, 14, 64, 16,
    17, 18, 19, 20, 21, 22, 64, 24, 25, 26, 27, 28, 29, 30, 64, 32, 33, 34, 35, 36, 37, 38, 64, 40,
    41, 42, 43, 44, 45, 46, 64, 48, 49, 50, 51, 52, 53, 54,
];
pub static UP_RIGHT: [u8; 64] = [
    9, 10, 11, 12, 13, 14, 15, 64, 17, 18, 19, 20, 21, 22, 23, 64, 25, 26, 27, 28, 29, 30, 31, 64,
    33, 34, 35, 36, 37, 38, 39, 64, 41, 42, 43, 44, 45, 46, 47, 64, 49, 50, 51, 52, 53, 54, 55, 64,
    57, 58, 59, 60, 61, 62, 63, 64, 64, 64, 64, 64, 64, 64, 64, 64,
];
pub static UP_LEFT: [u8; 64] = [
    64, 8, 9, 10, 11, 12, 13, 14, 64, 16, 17, 18, 19, 20, 21, 22, 64, 24, 25, 26, 27, 28, 29, 30,
    64, 32, 33, 34, 35, 36, 37, 38, 64, 40, 41, 42, 43, 44, 45, 46, 64, 48, 49, 50, 51, 52, 53, 54,
    64, 56, 57, 58, 59, 60, 61, 62, 64, 64, 64, 64, 64, 64, 64, 64,
];
//Functions to get precomputed values of squares relative to any given square
fn up(square: usize) -> Option<u8> {
    if UP[square] != 64 {
        Some(UP[square])
    } else {
        None
    }
}
fn down(square: usize) -> Option<u8> {
    if DOWN[square] != 64 {
        Some(DOWN[square])
    } else {
        None
    }
}
fn left(square: usize) -> Option<u8> {
    if LEFT[square] != 64 {
        Some(LEFT[square])
    } else {
        None
    }
}
fn right(square: usize) -> Option<u8> {
    if RIGHT[square] != 64 {
        Some(RIGHT[square])
    } else {
        None
    }
}
fn down_right(square: usize) -> Option<u8> {
    if DOWN_RIGHT[square] != 64 {
        Some(DOWN_RIGHT[square])
    } else {
        None
    }
}
fn down_left(square: usize) -> Option<u8> {
    if DOWN_LEFT[square] != 64 {
        Some(DOWN_LEFT[square])
    } else {
        None
    }
}
fn up_left(square: usize) -> Option<u8> {
    if UP_LEFT[square] != 64 {
        Some(UP_LEFT[square])
    } else {
        None
    }
}
fn up_right(square: usize) -> Option<u8> {
    if UP_RIGHT[square] != 64 {
        Some(UP_RIGHT[square])
    } else {
        None
    }
}

// Trait which every piece implements. Has only one function, which generates all possible moves for that piece.
trait PieceTrait {
    fn generate_moves(
        &self,
        board: &Board,
        square: u8,
        moves_list: &mut [Option<Move>; 28],
    );
}

// For the pieces that move straight until they find an enemy piece (queen, rook, bishop)
trait MovesInALine {
    fn move_in_line(
        &self,
        direction: fn(usize) -> Option<u8>,
        board: &Board,
        square: u8,
        moves: &mut [Option<Move>; 28],
        own_color: Color,
        moves_index: &mut usize,
    ) {
        let mut next_square = direction(square as usize);
        // While there is a next valid square
        while let Some(square_in_line) = next_square {
            // What is in the square.
            let piece_in_square = board.board[square_in_line as usize];
            // if there is a piece
            if let Some(piece) = piece_in_square {
                // if  color is different, add that as a move and stop loop, else, stop loop
                if piece.get_color() != own_color {
                    moves[*moves_index] = Some(Move::RegularMove(square_in_line));
                    *moves_index += 1;
                }
                break;
            } else {
                moves[*moves_index] = Some(Move::RegularMove(square_in_line));
                *moves_index += 1;
            }
            // go to next square in line
            next_square = direction(next_square.unwrap() as usize);
        }
        moves[*moves_index] = None;
    }
}
#[derive(Clone, Copy, Debug)]
pub enum Move {
    RegularMove(u8),
    CastleKingside,
    CastleQueenside,
    EnPassant(u8),
    PawnAdvanceTwoSquares(u8),
    PawnPromotion(u8, Piece),
}

// A piece can be black or white.
#[derive(Clone, Copy, Debug, PartialEq, Hash, Eq, Ord, PartialOrd)]
pub enum Color {
    Black,
    White,
}

// Functions to facilitate working with this enum
impl Color {
    // Is the color white?
    fn is_white(&self) -> bool {
        *self == Color::White
    }
    // Get the opposite of the color that came in
    pub fn reverse(&self) -> Color {
        if *self == Color::White {
            return Color::Black;
        }
        Color::White
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Hash, Eq, Ord, PartialOrd)]
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
    // Returns the piece's color
    fn get_color(&self) -> Color {
        match *self {
            Piece::Pawn(piece) => piece.color,
            Piece::Knight(piece) => piece.color,
            Piece::Bishop(piece) => piece.color,
            Piece::Rook(piece) => piece.color,
            Piece::Queen(piece) => piece.color,
            Piece::King(piece) => piece.color,
        }
    }
    //returns the piece's moves
    pub fn get_moves(
        &self,
        board: &Board,
        piece_square: u8,
        moves: &mut [Option<Move>; 28],
    ) {
        match *self {
            Piece::Pawn(piece) => piece.generate_moves(board, piece_square, moves),
            Piece::Knight(piece) => piece.generate_moves(board, piece_square, moves),
            Piece::Bishop(piece) => piece.generate_moves(board, piece_square, moves),
            Piece::Rook(piece) => piece.generate_moves(board, piece_square, moves),
            Piece::Queen(piece) => piece.generate_moves(board, piece_square, moves),
            Piece::King(piece) => piece.generate_moves(board, piece_square, moves),
        }
    }
    //Returns a bool corresponding to whether piece a is laterally adjacent to piece b
    fn is_to_the_side_of(own_square: usize, other_square: u8) -> bool {
        if let Some(square) = left(own_square) {
            if square == other_square {
                return true;
            }
        }
        if let Some(square) = right(own_square) {
            if square == other_square {
                return true;
            }
        }
        false
    }
}

// Each piece may implement different functions.
#[derive(Clone, Copy, Debug, PartialEq, Hash, Eq, Ord, PartialOrd)]
pub struct Pawn {
    color: Color,
}
#[derive(Clone, Copy, Debug, PartialEq, Hash, Eq, Ord, PartialOrd)]
pub struct Knight {
    color: Color,
}
#[derive(Clone, Copy, Debug, PartialEq, Hash, Eq, Ord, PartialOrd)]
pub struct Bishop {
    color: Color,
}
#[derive(Clone, Copy, Debug, PartialEq, Hash, Eq, Ord, PartialOrd)]
pub struct Rook {
    color: Color,
}
#[derive(Clone, Copy, Debug, PartialEq, Hash, Eq, Ord, PartialOrd)]
pub struct Queen {
    color: Color,
}
#[derive(Clone, Copy, Debug, PartialEq, Hash, Eq, Ord, PartialOrd)]
pub struct King {
    pub color: Color,
}

impl PieceTrait for Pawn {
    // Generate possible moves for a pawn
    fn generate_moves(
        &self,
        board: &Board,
        piece_square: u8,
        moves: &mut [Option<Move>; 28],
    ){
        let mut moves_index = 0;
        // Create the vector which will be returned
        // First possibility for the next square (up if white, down if black)
        let end_square = if let Color::White = self.color {
            up(piece_square as usize)
        } else {
            down(piece_square as usize)
        };
        // If there is a square next, proceed
        if let Some(end_square) = end_square {
            // the next square in the board
            let end_square_in_board = board.board[end_square as usize];
            //if the square is empty, (i. e. there are no pieces in it), proceed
            if end_square_in_board.is_none() {
                //if it's in the last rank, we can add in its promotions
                if Board::get_row(end_square) == if let Color::White = self.color { 7 } else { 0 } {
                    let own_color = self.color;
                    moves[0] = Some(Move::PawnPromotion(
                        end_square,
                        Piece::Bishop(Bishop { color: own_color }),
                    ));
                    moves[1] = Some(Move::PawnPromotion(
                        end_square,
                        Piece::Knight(Knight { color: own_color }),
                    ));
                    moves[2] = Some(Move::PawnPromotion(
                        end_square,
                        Piece::Rook(Rook { color: own_color }),
                    ));
                    moves[3] = Some(Move::PawnPromotion(
                        end_square,
                        Piece::Queen(Queen { color: own_color }),
                    ));
                    moves_index = 4;
                }
                // if the pawn is in it's initial rank, proceed
                else if Board::get_row(piece_square)
                    == if let Color::White = self.color { 1 } else { 6 }
                {
                    // Create a next square, as the upper (or the one below) the previous square
                    let next_square = if let Color::White = self.color {
                        up(end_square as usize).unwrap()
                    } else {
                        down(end_square as usize).unwrap()
                    };
                    // if there are no pieces in that square, add the square to the list of moves
                    if board.board[next_square as usize].is_none() {
                        moves[moves_index] = Some(Move::PawnAdvanceTwoSquares(next_square));
                        moves_index += 1;
                    }
                }
                // And we can add a move forward no matter the rank, because the square ahead is empty.
                if Board::get_row(end_square) != if let Color::White = self.color { 7 } else { 0 } {
                    moves[moves_index] = Some(Move::RegularMove(end_square));
                    moves_index += 1;
                }
            }
        }
        // Check if the pawn can take anything, i'll probably make a macro or new function to avoid repetition, after i fix bugs

        if let Some(square) = if self.color.is_white() {
            up_right(piece_square as usize)
        } else {
            down_right(piece_square as usize)
        } {
            // If there is a piece to the diagonal right of the square
            if let Some(piece) = board.board[square as usize] {
                // if it's not an ally
                if piece.get_color() != self.color {
                    // promote it if needed, else just add it as a regular move
                    if Board::get_row(square) == if let Color::White = self.color { 7 } else { 0 } {
                        let own_color = self.color;
                        moves[moves_index] = Some(Move::PawnPromotion(
                            square,
                            Piece::Bishop(Bishop { color: own_color }),
                        ));
                        moves[moves_index + 1] = Some(Move::PawnPromotion(
                            square,
                            Piece::Knight(Knight { color: own_color }),
                        ));
                        moves[moves_index + 2] = Some(Move::PawnPromotion(
                            square,
                            Piece::Rook(Rook { color: own_color }),
                        ));
                        moves[moves_index + 3] = Some(Move::PawnPromotion(
                            square,
                            Piece::Queen(Queen { color: own_color }),
                        ));
                        moves_index += 4;
                    } else {
                        moves[moves_index] = Some(Move::RegularMove(square));
                        moves_index += 1;
                    }
                }
            }
        }
        if let Some(square) = if self.color.is_white() {
            up_left(piece_square as usize)
        } else {
            down_left(piece_square as usize)
        } {
            // if there's a piece to the diagonal left of the square
            if let Some(piece) = board.board[square as usize] {
                // if it's not an ally
                if piece.get_color() != self.color {
                    // add promotion if needed, else a regular move.
                    if Board::get_row(square) == if let Color::White = self.color { 7 } else { 0 } {
                        let own_color = self.color;
                        moves[moves_index] = Some(Move::PawnPromotion(
                            square,
                            Piece::Bishop(Bishop { color: own_color }),
                        ));
                        moves[moves_index + 1] = Some(Move::PawnPromotion(
                            square,
                            Piece::Knight(Knight { color: own_color }),
                        ));
                        moves[moves_index + 2] = Some(Move::PawnPromotion(
                            square,
                            Piece::Rook(Rook { color: own_color }),
                        ));
                        moves[moves_index + 3] = Some(Move::PawnPromotion(
                            square,
                            Piece::Queen(Queen { color: own_color }),
                        ));
                        moves_index += 4;
                    } else {
                        moves[moves_index] = Some(Move::RegularMove(square));
                        moves_index += 1;
                    }
                }
            }
        }

        // Check if the pawn can en passant
        if let CanEnPassant::Yes(square) = board.can_en_passant {
            if Piece::is_to_the_side_of(piece_square as usize, square) {
                moves[moves_index] = Some(if self.color.is_white() {
                    Move::EnPassant(up(square as usize).unwrap())
                } else {
                    Move::EnPassant(down(square as usize).unwrap())
                });
                moves_index += 1;
            }
        }
        moves[moves_index] = None;
    }
}

impl PieceTrait for Knight {
    fn generate_moves(
        &self,
        board: &Board,
        square: u8,
        moves: &mut [Option<Move>; 28],
    ){
        let mut moves_index = 0;
        // list all possible 8 knight moves, Some variant exists in board, None doesn't.
        let possible_knight_moves = [
            up_left(up(square as usize).unwrap_or(0) as usize),
            up_right(up(square as usize).unwrap_or(63) as usize),
            down_left(down(square as usize).unwrap_or(0) as usize),
            down_right(down(square as usize).unwrap_or(0) as usize),
            up_right(right(square as usize).unwrap_or(63) as usize),
            down_right(right(square as usize).unwrap_or(0) as usize),
            up_left(left(square as usize).unwrap_or(63) as usize),
            down_left(left(square as usize).unwrap_or(0) as usize),
        ];
        // for each possible move, that exists on the board,
        for poss_move in possible_knight_moves.into_iter().flatten() {
            // if there is a piece in the square
            if let Some(piece) = board.board[poss_move as usize] {
                // if the color of the Knight and piece in the square are different
                if piece.get_color() != self.color {
                    // add that as a possible move for the knight
                    moves[moves_index] = Some(Move::RegularMove(poss_move));
                    moves_index += 1;
                }
            } else {
                // else (if there are no pieces in the valid square), add a possible move for the knight.
                moves[moves_index] = Some(Move::RegularMove(poss_move));
                moves_index += 1;
            }
        }
        moves[moves_index] = None;
    }
}

impl PieceTrait for Bishop {
    fn generate_moves(
        &self,
        board: &Board,
        square: u8,
        moves: &mut [Option<Move>; 28],
    ) {
        let directions: [fn(usize) -> Option<u8>; 4] = [up_left, up_right, down_left, down_right];
        let mut moves_index = 0;
        for function in directions {
            self.move_in_line(
                function,
                board,
                square,
                moves,
                self.color,
                &mut moves_index,
            );
        }

    }
}

impl PieceTrait for Rook {
    fn generate_moves(
        &self,
        board: &Board,
        square: u8,
        moves: &mut [Option<Move>; 28],
    ){
        let directions: [fn(usize) -> Option<u8>; 4] = [up, down, left, right];
        let mut moves_index = 0;
        for function in directions {
            self.move_in_line(
                function,
                board,
                square,
                moves,
                self.color,
                &mut moves_index,
            );
        }

    }
}

impl PieceTrait for Queen {
    fn generate_moves(
        &self,
        board: &Board,
        square: u8,
        moves: &mut [Option<Move>; 28],
    ) {
        let directions: [fn(usize) -> Option<u8>; 8] = [
            up_left, up_right, down_left, down_right, up, down, left, right,
        ];
        let mut moves_index = 0;
        for function in directions {
            self.move_in_line(
                function,
                board,
                square,
                moves,
                self.color,
                &mut moves_index,
            );
        }


    }
}
impl MovesInALine for Queen {}
impl MovesInALine for Bishop {}
impl MovesInALine for Rook {}

impl King {
    fn get_adjacent_squares(king: usize) -> [Option<u8>; 8] {
        [
            up(king),
            down(king),
            left(king),
            right(king),
            up_left(king),
            up_right(king),
            down_left(king),
            down_right(king),
        ]
    }
}

impl PieceTrait for King {
    fn generate_moves(
        &self,
        board: &Board,
        square: u8,
        moves: &mut [Option<Move>; 28],
    ) {
        let mut moves_index = 0;
        let is_white = self.color.is_white();
        let kingside: bool;
        let kingisde_pieces: [usize; 2] = if is_white { [5, 6] } else { [61, 62] };
        let queenside_pieces: [usize; 3] = if is_white { [3, 2, 1] } else { [59, 58, 57] };
        let queenside: bool;
        for square in Self::get_adjacent_squares(square as usize)
            .into_iter()
            .flatten()
            .filter(|sqr| {
                if let Some(piece) = board.board[*sqr as usize] {
                    piece.get_color() != self.color
                } else {
                    true
                }
            })
            .map(Move::RegularMove)
            .collect::<Vec<Move>>()
        {
            moves[moves_index] = Some(square);
            moves_index += 1;
        }

        // Check for castling
        if square == if is_white { 4 } else { 60 } {
            let can_castle = board.can_castle;
            if is_white {
                kingside = can_castle.white_kingside;
                queenside = can_castle.white_queenside;
            } else {
                kingside = can_castle.black_kingside;
                queenside = can_castle.black_queenside;
            }
            if kingside {
                if let Some(Piece::Rook(Rook { color })) =
                    board.board[if is_white { 7 } else { 63 }]
                {
                    if self.color == color
                        && kingisde_pieces
                            .iter()
                            .all(|sqr| board.board[*sqr].is_none())
                    {
                        moves[moves_index] = Some(Move::CastleKingside);
                        moves_index += 1;
                    }
                }
            }
            if queenside {
                if let Some(Piece::Rook(Rook { color })) =
                    board.board[if is_white { 0 } else { 56 }]
                {
                    if self.color == color
                        && queenside_pieces
                            .iter()
                            .all(|sqr| board.board[*sqr].is_none())
                    {
                        moves[moves_index] = Some(Move::CastleQueenside);
                        moves_index += 1;
                    }
                }
            }
        }
        moves[moves_index] = None;
    }
}

pub fn convert_to_square(num: u8) -> String {
    let column = match num % 8 {
        0 => "a",
        1 => "b",
        2 => "c",
        3 => "d",
        4 => "e",
        5 => "f",
        6 => "g",
        7 => "h",
        _ => panic!("Number higher than squares in a chess board"),
    };
    let coordinate = column.to_string() + ((num / 8) + 1).to_string().as_str();
    coordinate
}

pub fn from_square(square: &str) -> u8 {
    if !square.len() == 2 {
        panic!("Square length should be 2 for string conversion to square");
    }
    square
        .split("")
        .filter(|item| !item.is_empty())
        .enumerate()
        .map(|(i, letter)| {
            if i == 0 {
                match letter {
                    "a" => 0,
                    "b" => 1,
                    "c" => 2,
                    "d" => 3,
                    "e" => 4,
                    "f" => 5,
                    "g" => 6,
                    "h" => 7,
                    _ => {
                        println!("{letter}");
                        panic!("Invalid coordinate: column is not in board")
                    }
                }
            } else {
                let new_val = (letter
                    .parse::<u8>()
                    .expect("Invalid coordinate: row isn't an integer")
                    - 1)
                    * 8;
                if new_val <= 56 {
                    new_val
                } else {
                    panic!("Invalid coordinate: row not in board")
                }
            }
        })
        .sum()
}

impl fmt::Display for Move {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Move::RegularMove(sqr) => {
                let square = convert_to_square(sqr);
                write!(f, "{square}")
            }
            Move::CastleKingside => write!(f, "O-O"),
            Move::CastleQueenside => write!(f, "O-O-O"),
            Move::EnPassant(sqr) => {
                let square = convert_to_square(sqr);
                write!(f, "{square}")
            }
            Move::PawnAdvanceTwoSquares(sqr) => {
                let square = convert_to_square(sqr);
                write!(f, "{square}")
            }
            Move::PawnPromotion(sqr, piece) => {
                let square = convert_to_square(sqr);
                let piece_result = match piece {
                    Piece::Queen(_) => "q",
                    Piece::Rook(_) => "r",
                    Piece::Bishop(_) => "b",
                    Piece::Knight(_) => "n",
                    Piece::Pawn(_) => panic!("Pawn tried to promote to pawn"),
                    Piece::King(_) => panic!("Pawn tried to promote to king"),
                };
                write!(f, "{square}{piece_result}")
            }
        }
    }
}
