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

pub struct Piece {
    board: BitBoard,
    ptype: PieceTypes,
}
pub struct Position {
    white: BitBoard,
    black: BitBoard,

    white_pawns: Piece,
    white_knights: Piece,
    white_bishops: Piece,
    white_rooks: Piece,
    white_queens: Piece,
    white_king: Piece,

    black_pawns: Piece,
    black_knights: Piece,
    black_bishops: Piece,
    black_rooks: Piece,
    black_queens: Piece,
    black_king: Piece,
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

            white_pawns: Piece {
                ptype: PieceTypes::Pawn,
                board: BitBoard(consts::STARTPOS_WHITE_PAWNS),
            },
            white_knights: Piece {
                ptype: PieceTypes::Knight,
                board: BitBoard(consts::STARTPOS_WHITE_KNIGHTS),
            },
            white_bishops: Piece {
                ptype: PieceTypes::Bishop,
                board: BitBoard(consts::STARTPOS_WHITE_BISHOPS),
            },
            white_rooks: Piece {
                ptype: PieceTypes::Rook,
                board: BitBoard(consts::STARTPOS_WHITE_ROOKS),
            },
            white_queens: Piece {
                ptype: PieceTypes::Queen,
                board: BitBoard(consts::STARTPOS_WHITE_QUEEN),
            },
            white_king: Piece {
                ptype: PieceTypes::King,
                board: BitBoard(consts::STARTPOS_WHITE_KING),
            },

            black_pawns: Piece {
                ptype: PieceTypes::Pawn,
                board: BitBoard(consts::STARTPOS_BLACK_PAWNS),
            },
            black_knights: Piece {
                ptype: PieceTypes::Knight,
                board: BitBoard(consts::STARTPOS_BLACK_KNIGHTS),
            },
            black_bishops: Piece {
                ptype: PieceTypes::Bishop,
                board: BitBoard(consts::STARTPOS_BLACK_BISHOPS),
            },
            black_rooks: Piece {
                ptype: PieceTypes::Rook,
                board: BitBoard(consts::STARTPOS_BLACK_ROOKS),
            },
            black_queens: Piece {
                ptype: PieceTypes::Queen,
                board: BitBoard(consts::STARTPOS_BLACK_QUEEN),
            },
            black_king: Piece {
                ptype: PieceTypes::King,
                board: BitBoard(consts::STARTPOS_BLACK_KING),
            },
        }
    }

    pub fn place_piece(&mut self, piece_type: PieceTypes, color: Color, mask: Mask) {
        if !self.white.has_piece(&mask) && !self.black.has_piece(&mask) {
            self.add_piece(piece_type, color, mask);
        } else {
            if self.white.has_piece(&mask) {
                if let Color::Black = color {
                    self.add_piece(piece_type, color, mask);
                    todo!()
                }
            }
        }
    }

    fn add_piece(&mut self, piece_type: PieceTypes, color: Color, mask: Mask) {
        match color {
            Color::Black => {
                self.black.add_piece(&mask);
                match piece_type {
                    PieceTypes::Pawn => self.black_pawns.board.add_piece(&mask),
                    PieceTypes::Knight => self.black_knights.board.add_piece(&mask),
                    PieceTypes::Bishop => self.black_bishops.board.add_piece(&mask),
                    PieceTypes::Rook => self.black_rooks.board.add_piece(&mask),
                    PieceTypes::Queen => self.black_queens.board.add_piece(&mask),
                    PieceTypes::King => self.black_king.board.0 = mask.0,
                }
            }

            Color::White => {
                self.white.add_piece(&mask);
                match piece_type {
                    PieceTypes::Pawn => self.white_pawns.board.add_piece(&mask),
                    PieceTypes::Knight => self.white_knights.board.add_piece(&mask),
                    PieceTypes::Bishop => self.white_bishops.board.add_piece(&mask),
                    PieceTypes::Rook => self.white_rooks.board.add_piece(&mask),
                    PieceTypes::Queen => self.white_queens.board.add_piece(&mask),
                    PieceTypes::King => self.white_king.board.0 = mask.0,
                }
            }
        }
    }

    pub fn remove_piece(&mut self, piece_type: PieceTypes, color: Color, mask: Mask) {
        match color {
            Color::Black => {
                self.black.delete_piece(&mask);
                match piece_type {
                    PieceTypes::Pawn => self.black_pawns.board.delete_piece(&mask),
                    PieceTypes::Knight => self.black_knights.board.delete_piece(&mask),
                    PieceTypes::Bishop => self.black_bishops.board.delete_piece(&mask),
                    PieceTypes::Rook => self.black_rooks.board.delete_piece(&mask),
                    PieceTypes::Queen => self.black_queens.board.delete_piece(&mask),
                    PieceTypes::King => self.black_king.board.0 = 0,
                }
            }

            Color::White => {
                self.white.delete_piece(&mask);
                match piece_type {
                    PieceTypes::Pawn => self.white_pawns.board.delete_piece(&mask),
                    PieceTypes::Knight => self.white_knights.board.delete_piece(&mask),
                    PieceTypes::Bishop => self.white_bishops.board.delete_piece(&mask),
                    PieceTypes::Rook => self.white_rooks.board.delete_piece(&mask),
                    PieceTypes::Queen => self.white_queens.board.delete_piece(&mask),
                    PieceTypes::King => self.white_king.board.0 = 0,
                }
            }
        }
    }

    pub fn locate_piece(
        &self,
        piece_type: Option<PieceTypes>,
        color: Option<Color>,
        mask: Mask,
    ) -> (Color, PieceTypes) {
        let col = match color {
            Some(c) => c,
            None => {
                if self.black.has_piece(&mask) {
                    Color::Black
                } else if self.white.has_piece(&mask) {
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
                    if self.black_pawns.board.has_piece(&mask) {
                        PieceTypes::Pawn
                    } else if self.black_knights.board.has_piece(&mask) {
                        PieceTypes::Knight
                    } else if self.black_bishops.board.has_piece(&mask) {
                        PieceTypes::Bishop
                    } else if self.black_rooks.board.has_piece(&mask) {
                        PieceTypes::Rook
                    } else if self.black_queens.board.has_piece(&mask) {
                        PieceTypes::Queen
                    } else if self.black_king.board.has_piece(&mask) {
                        PieceTypes::King
                    } else {
                        panic!("Piece not in board")
                    }
                }
                Color::White => {
                    if self.white_pawns.board.has_piece(&mask) {
                        PieceTypes::Pawn
                    } else if self.white_knights.board.has_piece(&mask) {
                        PieceTypes::Knight
                    } else if self.white_bishops.board.has_piece(&mask) {
                        PieceTypes::Bishop
                    } else if self.white_rooks.board.has_piece(&mask) {
                        PieceTypes::Rook
                    } else if self.white_queens.board.has_piece(&mask) {
                        PieceTypes::Queen
                    } else if self.white_king.board.has_piece(&mask) {
                        PieceTypes::King
                    } else {
                        panic!("Piece not in board")
                    }
                }
            },
        };
        (col, pc)
    }
}
