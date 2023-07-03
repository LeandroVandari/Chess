pub mod consts;
pub mod macros;
pub mod pieces;

pub use macros::implement_bitboard;

pub trait BitBoard {
    fn has_piece(&self, mask: &Mask) -> bool;

    fn add_piece(&mut self, mask: &Mask);

    fn delete_piece(&mut self, mask: &Mask);

    fn get_board(&self) -> u64;
}

pub struct Side(u64);
macros::implement_bitboard!(Side);

pub struct Mask(u64);

pub enum Color {
    White,
    Black,
}

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
    #[inline(always)]
    pub fn from_square(square: u8) -> Self {
        Mask(1 << square)
    }

    #[inline(always)]
    fn reverse(&self) -> Self {
        Self(!self.0)
    }
}

impl Position {
    #[must_use]
    pub fn new() -> Self {
        Position {
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

    pub fn locate_piece(
        &self,
        piece_type: Option<pieces::PieceTypes>,
        color: Option<Color>,
        mask: &Mask,
    ) -> (Color, pieces::PieceTypes) {
        let col = match color {
            Some(c) => c,
            None => {
                if self.black.has_piece(mask) {
                    Color::Black
                } else if self.white.has_piece(mask) {
                    Color::White
                } else {
                    panic!("Piece not in board")
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
                        panic!("Piece not in board")
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
                        panic!("Piece not in board")
                    }
                }
            },
        };
        (col, pc)
    }

    pub fn place_piece(&mut self, piece_type: pieces::PieceTypes, color: Color, mask: Mask) {
        if !self.white.has_piece(&mask) && !self.black.has_piece(&mask) {
            self.add_piece(piece_type, color, mask);
        } else {
            let (col, ptype) = self.locate_piece(None, None, &mask);
            self.remove_piece(ptype, col, &mask);
            self.add_piece(piece_type, color, mask);
        }
    }

    pub fn remove_piece(&mut self, piece_type: pieces::PieceTypes, color: Color, mask: &Mask) {
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

    fn add_piece(&mut self, piece_type: pieces::PieceTypes, color: Color, mask: Mask) {
        match color {
            Color::Black => {
                self.black.add_piece(&mask);
                match piece_type {
                    pieces::PieceTypes::Pawn => self.black_pawns.add_piece(&mask),
                    pieces::PieceTypes::Knight => self.black_knights.add_piece(&mask),
                    pieces::PieceTypes::Bishop => self.black_bishops.add_piece(&mask),
                    pieces::PieceTypes::Rook => self.black_rooks.add_piece(&mask),
                    pieces::PieceTypes::Queen => self.black_queens.add_piece(&mask),
                    pieces::PieceTypes::King => self.black_king.0 = mask.0,
                }
            }

            Color::White => {
                self.white.add_piece(&mask);
                match piece_type {
                    pieces::PieceTypes::Pawn => self.white_pawns.add_piece(&mask),
                    pieces::PieceTypes::Knight => self.white_knights.add_piece(&mask),
                    pieces::PieceTypes::Bishop => self.white_bishops.add_piece(&mask),
                    pieces::PieceTypes::Rook => self.white_rooks.add_piece(&mask),
                    pieces::PieceTypes::Queen => self.white_queens.add_piece(&mask),
                    pieces::PieceTypes::King => self.white_king.0 = mask.0,
                }
            }
        }
    }
}

impl Default for Position {
    fn default() -> Self {
        Position::new()
    }
}
