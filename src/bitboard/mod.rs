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
struct BitBoard(pub u64);

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
    game_info: u8
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

    pub fn add_piece(&mut self, side: Side, piece: Piece, pos: u8) {
        let mask = 1 << pos;
        let sides = [side.0, !(!1 | side.0)];
        for curr_side in sides {
            if (self.sides[curr_side].0 & mask) >> pos == 1 {
                self.sides[curr_side].0 &= !mask;
                for (index, bb) in self.pieces[side.0].iter().enumerate() {
                    if (bb.0 & mask) >> pos == 1 {
                        self.pieces[curr_side][index].0 &= !mask;
                        break;
                    }
                }
            }
    
        }
        self.sides[side.0].0 |= mask;
        self.pieces[side.0][piece.0].0 |= mask;
    }
}




impl Default for Position {
    fn default() -> Self {
        Self::new()
    }
}

impl std::fmt::Display for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}   