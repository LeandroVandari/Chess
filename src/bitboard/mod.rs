pub mod consts;
use consts::*;

macro_rules! bitboard_list {
    ( $( $x:ident ),* ) => {
            [
            $(
                BitBoard($x),
            )*
            ]
    };
}

#[derive(Clone, Copy, Debug)]
struct BitBoard(u64);
pub struct Mask(u64);

#[derive(Clone, Copy)]
pub struct Piece(pub usize);

#[derive(Clone, Copy)]
pub struct Side(pub usize);

#[derive(Debug)]
pub struct Position {
    sides: [BitBoard; 2],
    pieces: [[BitBoard; 6]; 2],
}

struct State {
    position: Position,
    game_info: u8,
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
    fn has_piece(&self, mask: &Mask) -> bool {
    (self.0 & mask.0) != 0
    }

    #[inline(always)]
    fn add_piece(&mut self, mask: &Mask) {
        self.0 |= mask.0
    }

    #[inline(always)]
    fn delete_piece(&mut self, mask: &Mask) {
        self.0 &= mask.reverse().0
    }
}

impl Position {
    pub fn new() -> Self {
        Self {
            sides: bitboard_list!(STARTPOS_BLACK, STARTPOS_WHITE),
            pieces: [
                bitboard_list!(
                    STARTPOS_BLACK_PAWNS,
                    STARTPOS_BLACK_KNIGHTS,
                    STARTPOS_BLACK_BISHOPS,
                    STARTPOS_BLACK_ROOKS,
                    STARTPOS_BLACK_QUEEN,
                    STARTPOS_BLACK_KING
                ),
                bitboard_list!(
                    STARTPOS_WHITE_PAWNS,
                    STARTPOS_WHITE_KNIGHTS,
                    STARTPOS_WHITE_BISHOPS,
                    STARTPOS_WHITE_ROOKS,
                    STARTPOS_WHITE_QUEEN,
                    STARTPOS_WHITE_KING
                ),
            ],
        }
    }
    pub fn empty() -> Self {
        Self {
            sides: bitboard_list!(ZERO, ZERO),
            pieces: [
                bitboard_list!(ZERO, ZERO, ZERO, ZERO, ZERO, ZERO),
                bitboard_list!(ZERO, ZERO, ZERO, ZERO, ZERO, ZERO),
            ],
        }
    }
    pub fn replace_piece(&mut self, side: Side, piece: Piece, mask: Mask) {
        for i in 0usize..=1 {
            if self.sides[i].has_piece(&mask) {
                if side.0 == i {
                    for board_index in 0usize..6 {
                        if self.pieces[i][board_index].has_piece(&mask) {
                            if !(board_index == piece.0) {
                                self.pieces[i][board_index].delete_piece(&mask);
                                break;
                            }
                        }
                    }
                } else {
                    self.sides[i].delete_piece(&mask);

                    self.sides[side.0].add_piece(&mask);
                    for index in 0..6 {
                        // Two options here, just removing from all or checking first if there is a piece and then removing.
                        // just remove: board.delete_piece(&mask);
                        if self.pieces[i][index].has_piece(&mask) {
                            self.pieces[i][index].delete_piece(&mask);
                            break;
                        }
                        
                    }
                }
            }
        }
        self.pieces[side.0][piece.0].add_piece(&mask);
    }

    pub fn convert_to_board(&self) -> crate::Board {
        let mut board = crate::Board::empty();
        for (side, piece_list) in self.pieces.iter().enumerate() {
            for (piece, bb) in piece_list.iter().enumerate() {
                if bb.0 != 0 {
                    for pos in 0..64 {
                        let mask = Mask::from_square(pos);
                        if bb.has_piece(&mask) {
                            board.add_piece(
                                crate_piece_from_piece(Piece(piece), Side(side)),
                                pos as usize,
                            )
                        }
                    }
                }
            }
        }
        board
    }
}

fn crate_piece_from_piece(piece: Piece, color: Side) -> crate::Piece {
    let piece_color = match color.0 {
        consts::ValueSides::BLACK => crate::Color::Black,
        consts::ValueSides::WHITE => crate::Color::White,
        _ => panic!("uhhhhhh"),
    };
    match piece.0 {
        consts::ValuePieces::PAWN => crate::Piece::Pawn(crate::Pawn { color: piece_color }),
        consts::ValuePieces::KNIGHT => crate::Piece::Knight(crate::Knight { color: piece_color }),
        consts::ValuePieces::BISHOP => crate::Piece::Bishop(crate::Bishop { color: piece_color }),
        consts::ValuePieces::ROOK => crate::Piece::Rook(crate::Rook { color: piece_color }),
        consts::ValuePieces::QUEEN => crate::Piece::Queen(crate::Queen { color: piece_color }),
        consts::ValuePieces::KING => crate::Piece::King(crate::King { color: piece_color }),
        _ => panic!("OOOOOPS"),
    }
}

impl Default for Position {
    fn default() -> Self {
        Self::new()
    }
}

impl std::fmt::Display for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.convert_to_board().fmt(f)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn bitboard_new_is_startpos() {
        assert_eq!(super::Position::new().convert_to_board(), crate::Board::new())
    }
}