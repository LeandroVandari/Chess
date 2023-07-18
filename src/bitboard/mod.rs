pub mod consts;
pub mod macros;
pub mod pieces;

pub use macros::implement_bitboard_trait;

use pieces::Piece;
/// The trait implemented by a struct containing a `u64`, representing a bitboard. Should be implemented using the [`macros::implement_bitboard_trait`] macro.
pub trait BitBoard {
    /// Check if the bitboard has a piece in a given position (& operator).
    fn has_piece(&self, mask: &Mask) -> bool;

    /// Add a piece at a given position (| operator).
    fn add_piece(&mut self, mask: &Mask);

    /// Remove a piece at a given position (& and ! operators).
    fn delete_piece(&mut self, mask: &Mask);

    /// Return the inner `u64`.
    fn get_board(&self) -> u64;
}

/// Represent a side (white or black).
pub struct Side(u64);

#[derive(Clone, Copy)]
pub struct Move(pub u64);

pub struct EnPassant(pub u64);

macros::implement_bitboard_trait!(Side, Move, EnPassant);

/// Newtype on a `u64` to do basic operations and pass in functions.
pub struct Mask(u64);

/// Deal with game order, piece side etc.
#[derive(PartialEq)]
pub enum Color {
    White,
    Black,
}

/// Contains all bitboards fundamental to a position.
pub struct Position {
    white: Side,
    black: Side,

    white_pawns: pieces::Pawn,
    white_knights: pieces::Knight,
    white_bishops: pieces::Bishop,
    white_rooks: pieces::Rook,
    white_queens: pieces::Queen,
    white_king: pieces::King,

    black_pawns: pieces::Pawn,
    black_knights: pieces::Knight,
    black_bishops: pieces::Bishop,
    black_rooks: pieces::Rook,
    black_queens: pieces::Queen,
    black_king: pieces::King,
}

impl Mask {
    #[must_use]
    pub fn from_square(square: u8) -> Self {
        Mask(1 << square)
    }

    #[must_use]
    fn reverse(&self) -> Self {
        Self(!self.0)
    }
}

impl Position {
    /// Returns a [Position] containing the starting position of chess.
    #[must_use]
    pub fn new() -> Self {
        Self {
            white: Side(consts::STARTPOS_WHITE),
            black: Side(consts::STARTPOS_BLACK),

            white_pawns: pieces::Pawn(consts::STARTPOS_WHITE_PAWNS),
            white_knights: pieces::Knight(consts::STARTPOS_WHITE_KNIGHTS),
            white_bishops: pieces::Bishop(consts::STARTPOS_WHITE_BISHOPS),
            white_rooks: pieces::Rook(consts::STARTPOS_WHITE_ROOKS),
            white_queens: pieces::Queen(consts::STARTPOS_WHITE_QUEEN),
            white_king: pieces::King(consts::STARTPOS_WHITE_KING),

            black_pawns: pieces::Pawn(consts::STARTPOS_BLACK_PAWNS),
            black_knights: pieces::Knight(consts::STARTPOS_BLACK_KNIGHTS),
            black_bishops: pieces::Bishop(consts::STARTPOS_BLACK_BISHOPS),
            black_rooks: pieces::Rook(consts::STARTPOS_BLACK_ROOKS),
            black_queens: pieces::Queen(consts::STARTPOS_BLACK_QUEEN),
            black_king: pieces::King(consts::STARTPOS_BLACK_KING),
        }
    }

    /// Returns an empty [Position] that can be worked upon.
    #[must_use]
    pub fn empty() -> Self {
        Self {
            white: Side(0),
            black: Side(0),

            white_pawns: pieces::Pawn(0),
            white_knights: pieces::Knight(0),
            white_bishops: pieces::Bishop(0),
            white_rooks: pieces::Rook(0),
            white_queens: pieces::Queen(0),
            white_king: pieces::King(0),

            black_pawns: pieces::Pawn(0),
            black_knights: pieces::Knight(0),
            black_bishops: pieces::Bishop(0),
            black_rooks: pieces::Rook(0),
            black_queens: pieces::Queen(0),
            black_king: pieces::King(0),
        }
    }

    /// Gets a pieces color and type given a `Mask` that contains the piece location. If piece type or color are already known, they can be specified with the `Some` variant.
    /// If the piece can't be located, it will return `None`.
    #[must_use]
    pub fn locate_piece(
        &self,
        piece_type: Option<pieces::PieceTypes>,
        color: Option<Color>,
        mask: &Mask,
    ) -> Option<(Color, pieces::PieceTypes)> {
        let col = match color {
            Some(c) => c,
            None => {
                if self.black.has_piece(mask) {
                    Color::Black
                } else if self.white.has_piece(mask) {
                    Color::White
                } else {
                    return None;
                }
            }
        };

        let pc = match piece_type {
            Some(p) => p,
            None => match col {
                Color::Black => {
                    if self.black_pawns.has_piece(mask) {
                        pieces::PieceTypes::Pawn
                    } else if self.black_knights.has_piece(mask) {
                        pieces::PieceTypes::Knight
                    } else if self.black_bishops.has_piece(mask) {
                        pieces::PieceTypes::Bishop
                    } else if self.black_rooks.has_piece(mask) {
                        pieces::PieceTypes::Rook
                    } else if self.black_queens.has_piece(mask) {
                        pieces::PieceTypes::Queen
                    } else if self.black_king.has_piece(mask) {
                        pieces::PieceTypes::King
                    } else {
                        return None;
                    }
                }
                Color::White => {
                    if self.white_pawns.has_piece(mask) {
                        pieces::PieceTypes::Pawn
                    } else if self.white_knights.has_piece(mask) {
                        pieces::PieceTypes::Knight
                    } else if self.white_bishops.has_piece(mask) {
                        pieces::PieceTypes::Bishop
                    } else if self.white_rooks.has_piece(mask) {
                        pieces::PieceTypes::Rook
                    } else if self.white_queens.has_piece(mask) {
                        pieces::PieceTypes::Queen
                    } else if self.white_king.has_piece(mask) {
                        pieces::PieceTypes::King
                    } else {
                        return None;
                    }
                }
            },
        };
        Some((col, pc))
    }

    /// Places a piece in the board, replacing any piece that is already there.
    pub fn place_piece(&mut self, piece_type: &pieces::PieceTypes, color: &Color, mask: &Mask) {
        let piece_in_board = self.locate_piece(None, None, mask);
        match piece_in_board {
            None => self.add_piece(piece_type, color, mask),
            Some((col, ptype)) => {
                self.remove_piece(&ptype, &col, mask);
                self.add_piece(piece_type, color, mask);
            }
        }
    }

    pub fn remove_piece(&mut self, piece_type: &pieces::PieceTypes, color: &Color, mask: &Mask) {
        match color {
            Color::Black => {
                self.black.delete_piece(mask);
                match piece_type {
                    pieces::PieceTypes::Pawn => self.black_pawns.delete_piece(mask),
                    pieces::PieceTypes::Knight => self.black_knights.delete_piece(mask),
                    pieces::PieceTypes::Bishop => self.black_bishops.delete_piece(mask),
                    pieces::PieceTypes::Rook => self.black_rooks.delete_piece(mask),
                    pieces::PieceTypes::Queen => self.black_queens.delete_piece(mask),
                    pieces::PieceTypes::King => self.black_king.0 = 0,
                }
            }

            Color::White => {
                self.white.delete_piece(mask);
                match piece_type {
                    pieces::PieceTypes::Pawn => self.white_pawns.delete_piece(mask),
                    pieces::PieceTypes::Knight => self.white_knights.delete_piece(mask),
                    pieces::PieceTypes::Bishop => self.white_bishops.delete_piece(mask),
                    pieces::PieceTypes::Rook => self.white_rooks.delete_piece(mask),
                    pieces::PieceTypes::Queen => self.white_queens.delete_piece(mask),
                    pieces::PieceTypes::King => self.white_king.0 = 0,
                }
            }
        }
    }

    fn add_piece(&mut self, piece_type: &pieces::PieceTypes, color: &Color, mask: &Mask) {
        match color {
            Color::Black => {
                self.black.add_piece(mask);
                match piece_type {
                    pieces::PieceTypes::Pawn => self.black_pawns.add_piece(mask),
                    pieces::PieceTypes::Knight => self.black_knights.add_piece(mask),
                    pieces::PieceTypes::Bishop => self.black_bishops.add_piece(mask),
                    pieces::PieceTypes::Rook => self.black_rooks.add_piece(mask),
                    pieces::PieceTypes::Queen => self.black_queens.add_piece(mask),
                    pieces::PieceTypes::King => self.black_king.0 = mask.0,
                }
            }

            Color::White => {
                self.white.add_piece(mask);
                match piece_type {
                    pieces::PieceTypes::Pawn => self.white_pawns.add_piece(mask),
                    pieces::PieceTypes::Knight => self.white_knights.add_piece(mask),
                    pieces::PieceTypes::Bishop => self.white_bishops.add_piece(mask),
                    pieces::PieceTypes::Rook => self.white_rooks.add_piece(mask),
                    pieces::PieceTypes::Queen => self.white_queens.add_piece(mask),
                    pieces::PieceTypes::King => self.white_king.0 = mask.0,
                }
            }
        }
    }

    pub fn generate_moves(
        &self,
        moves_list: &mut [Move; 16],
        en_passant: &EnPassant,
        color: &Color,
    ) {
        let mut offset = 0;
        match color {
            Color::Black => {
                self.black_pawns.generate_moves(
                    moves_list,
                    &mut offset,
                    self.black.0,
                    self.white.0,
                    Color::Black,
                    en_passant,
                );
                self.black_knights.generate_moves(
                    moves_list,
                    &mut offset,
                    self.black.0,
                    self.white.0,
                    Color::Black,
                    en_passant,
                );
                self.black_bishops.generate_moves(
                    moves_list,
                    &mut offset,
                    self.black.0,
                    self.white.0,
                    Color::Black,
                    en_passant,
                );
                self.black_rooks.generate_moves(
                    moves_list,
                    &mut offset,
                    self.black.0,
                    self.white.0,
                    Color::Black,
                    en_passant,
                );
                self.black_queens.generate_moves(
                    moves_list,
                    &mut offset,
                    self.black.0,
                    self.white.0,
                    Color::Black,
                    en_passant,
                );
                self.black_king.generate_moves(
                    moves_list,
                    &mut offset,
                    self.black.0,
                    self.white.0,
                    Color::Black,
                    en_passant,
                );
            }
            Color::White => {
                self.white_pawns.generate_moves(
                    moves_list,
                    &mut offset,
                    self.white.0,
                    self.black.0,
                    Color::White,
                    en_passant,
                );
                self.white_knights.generate_moves(
                    moves_list,
                    &mut offset,
                    self.white.0,
                    self.black.0,
                    Color::White,
                    en_passant,
                );
                self.white_bishops.generate_moves(
                    moves_list,
                    &mut offset,
                    self.white.0,
                    self.black.0,
                    Color::White,
                    en_passant,
                );
                self.white_rooks.generate_moves(
                    moves_list,
                    &mut offset,
                    self.white.0,
                    self.black.0,
                    Color::White,
                    en_passant,
                );
                self.white_queens.generate_moves(
                    moves_list,
                    &mut offset,
                    self.white.0,
                    self.black.0,
                    Color::White,
                    en_passant,
                );
                self.white_king.generate_moves(
                    moves_list,
                    &mut offset,
                    self.white.0,
                    self.black.0,
                    Color::White,
                    en_passant,
                );
            }
        }
    }
}

impl Default for Position {
    fn default() -> Self {
        Position::new()
    }
}

impl std::fmt::Display for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut board = String::new();

        for i in 0..64 {
            if i % 8 == 0 && i != 0 {
                board.push('\n');
            }
            let mask = &Mask(1 << i);
            if self.white.has_piece(mask) {
                if self.white_pawns.has_piece(mask) {
                    board.push('♙');
                } else if self.white_knights.has_piece(mask) {
                    board.push('♘');
                } else if self.white_bishops.has_piece(mask) {
                    board.push('♗');
                } else if self.white_rooks.has_piece(mask) {
                    board.push('♖');
                } else if self.white_queens.has_piece(mask) {
                    board.push('♕');
                } else if self.white_king.has_piece(mask) {
                    board.push('♔');
                }
            } else if self.black.has_piece(mask) {
                if self.black_pawns.has_piece(mask) {
                    board.push('♟');
                } else if self.black_knights.has_piece(mask) {
                    board.push('♞');
                } else if self.black_bishops.has_piece(mask) {
                    board.push('♝');
                } else if self.black_rooks.has_piece(mask) {
                    board.push('♜');
                } else if self.black_queens.has_piece(mask) {
                    board.push('♛');
                } else if self.black_king.has_piece(mask) {
                    board.push('♚');
                }
            } else {
                board.push('.');
            }
            board.push(' ');
        }
        write!(f, "{}", board.as_str())
    }
}
