

pub mod consts;

#[derive(Debug)]
pub struct BitBoard(u64);

pub struct Mask(u64);

#[derive(Clone, Copy, Debug)]
pub enum PieceTypes {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}

pub enum Color {
    White,
    Black,
}

pub struct Position {
    white: BitBoard,
    black: BitBoard,

    white_pawns: BitBoard,
    white_knights: BitBoard,
    white_bishops: BitBoard,
    white_rooks: BitBoard,
    white_queens: BitBoard,
    white_king: BitBoard,

    black_pawns: BitBoard,
    black_knights: BitBoard,
    black_bishops: BitBoard,
    black_rooks: BitBoard,
    black_queens: BitBoard,
    black_king: BitBoard,
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

impl BitBoard {
    #[inline(always)]
    pub fn has_piece(&self, mask: &Mask) -> bool {
        (self.0 & mask.0) != 0
    }

    #[inline(always)]
    pub fn add_piece(&mut self, mask: &Mask) {
        self.0 |= mask.0
    }

    #[inline(always)]
    pub fn delete_piece(&mut self, mask: &Mask) {
        self.0 &= mask.reverse().0
    }

    #[inline(always)]
    pub fn get_board(&self) -> u64 {
        self.0
    }
}

impl Position {
    #[must_use]
    pub fn new() -> Self {
        Position {
            white: BitBoard(consts::STARTPOS_WHITE),
            black: BitBoard(consts::STARTPOS_BLACK),

            white_pawns: BitBoard(consts::STARTPOS_WHITE_PAWNS),
            white_knights: BitBoard(consts::STARTPOS_WHITE_KNIGHTS),
            white_bishops: BitBoard(consts::STARTPOS_WHITE_BISHOPS),
            white_rooks: BitBoard(consts::STARTPOS_WHITE_ROOKS),
            white_queens: BitBoard(consts::STARTPOS_WHITE_QUEEN),
            white_king: BitBoard(consts::STARTPOS_WHITE_KING),

            black_pawns: BitBoard(consts::STARTPOS_BLACK_PAWNS),
            black_knights: BitBoard(consts::STARTPOS_BLACK_KNIGHTS),
            black_bishops: BitBoard(consts::STARTPOS_BLACK_BISHOPS),
            black_rooks: BitBoard(consts::STARTPOS_BLACK_ROOKS),
            black_queens: BitBoard(consts::STARTPOS_BLACK_QUEEN),
            black_king: BitBoard(consts::STARTPOS_BLACK_KING),
        }
    }

    pub fn locate_piece(
        &self,
        piece_type: Option<PieceTypes>,
        color: Option<Color>,
        mask: &Mask,
    ) -> (Color, PieceTypes) {
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
                        PieceTypes::Pawn
                    } else if self.black_knights.has_piece(mask) {
                        PieceTypes::Knight
                    } else if self.black_bishops.has_piece(mask) {
                        PieceTypes::Bishop
                    } else if self.black_rooks.has_piece(mask) {
                        PieceTypes::Rook
                    } else if self.black_queens.has_piece(mask) {
                        PieceTypes::Queen
                    } else if self.black_king.has_piece(mask) {
                        PieceTypes::King
                    } else {
                        panic!("Piece not in board")
                    }
                }
                Color::White => {
                    if self.white_pawns.has_piece(mask) {
                        PieceTypes::Pawn
                    } else if self.white_knights.has_piece(mask) {
                        PieceTypes::Knight
                    } else if self.white_bishops.has_piece(mask) {
                        PieceTypes::Bishop
                    } else if self.white_rooks.has_piece(mask) {
                        PieceTypes::Rook
                    } else if self.white_queens.has_piece(mask) {
                        PieceTypes::Queen
                    } else if self.white_king.has_piece(mask) {
                        PieceTypes::King
                    } else {
                        panic!("Piece not in board")
                    }
                }
            },
        };
        (col, pc)
    }

    pub fn place_piece(&mut self, piece_type: PieceTypes, color: Color, mask: Mask) {
        if !self.white.has_piece(&mask) && !self.black.has_piece(&mask) {
            self.add_piece(piece_type, color, mask);
        } else {
            let (col, ptype) = self.locate_piece(None, None, &mask);
            self.remove_piece(ptype, col, &mask);
            self.add_piece(piece_type, color, mask);
        }
    }

    pub fn remove_piece(&mut self, piece_type: PieceTypes, color: Color, mask: &Mask) {
        match color {
            Color::Black => {
                self.black.delete_piece(mask);
                match piece_type {
                    PieceTypes::Pawn => self.black_pawns.delete_piece(mask),
                    PieceTypes::Knight => self.black_knights.delete_piece(mask),
                    PieceTypes::Bishop => self.black_bishops.delete_piece(mask),
                    PieceTypes::Rook => self.black_rooks.delete_piece(mask),
                    PieceTypes::Queen => self.black_queens.delete_piece(mask),
                    PieceTypes::King => self.black_king.0 = 0,
                }
            }

            Color::White => {
                self.white.delete_piece(mask);
                match piece_type {
                    PieceTypes::Pawn => self.white_pawns.delete_piece(mask),
                    PieceTypes::Knight => self.white_knights.delete_piece(mask),
                    PieceTypes::Bishop => self.white_bishops.delete_piece(mask),
                    PieceTypes::Rook => self.white_rooks.delete_piece(mask),
                    PieceTypes::Queen => self.white_queens.delete_piece(mask),
                    PieceTypes::King => self.white_king.0 = 0,
                }
            }
        }
    }

    fn add_piece(&mut self, piece_type: PieceTypes, color: Color, mask: Mask) {
        match color {
            Color::Black => {
                self.black.add_piece(&mask);
                match piece_type {
                    PieceTypes::Pawn => self.black_pawns.add_piece(&mask),
                    PieceTypes::Knight => self.black_knights.add_piece(&mask),
                    PieceTypes::Bishop => self.black_bishops.add_piece(&mask),
                    PieceTypes::Rook => self.black_rooks.add_piece(&mask),
                    PieceTypes::Queen => self.black_queens.add_piece(&mask),
                    PieceTypes::King => self.black_king.0 = mask.0,
                }
            }

            Color::White => {
                self.white.add_piece(&mask);
                match piece_type {
                    PieceTypes::Pawn => self.white_pawns.add_piece(&mask),
                    PieceTypes::Knight => self.white_knights.add_piece(&mask),
                    PieceTypes::Bishop => self.white_bishops.add_piece(&mask),
                    PieceTypes::Rook => self.white_rooks.add_piece(&mask),
                    PieceTypes::Queen => self.white_queens.add_piece(&mask),
                    PieceTypes::King => self.white_king.0 = mask.0,
                }
            }
        }
    }
}

impl BitBoard {
    pub fn generate_rook_moves(&self) -> BitBoard {
        let directions = [8, 1];
        let num = 0;
        for d in directions {
            while let Some(bb) = self.0.checked_shl(d) {
                
            }
        }

        BitBoard(num)
    }
}

impl Default for Position {
    fn default() -> Self {
        Position::new()
    }
}
