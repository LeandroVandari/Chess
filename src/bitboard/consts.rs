#![cfg_attr(rustfmt, rustfmt_skip)]

pub const STARTPOS_BLACK:         u64 = 0b1111111111111111 << 48;
pub const STARTPOS_WHITE:         u64 = 0b1111111111111111;
pub const STARTPOS_ALL:           u64 = STARTPOS_BLACK | STARTPOS_WHITE;

pub const STARTPOS_BLACK_PAWNS:   u64 = 0b11111111 << 48;
pub const STARTPOS_BLACK_KNIGHTS: u64 = 0b01000010 << 56;
pub const STARTPOS_BLACK_BISHOPS: u64 = 0b00100100 << 56;
pub const STARTPOS_BLACK_ROOKS:   u64 = 0b10000001 << 56;
pub const STARTPOS_BLACK_QUEEN:   u64 = 0b00010000 << 56;
pub const STARTPOS_BLACK_KING:    u64 = 0b00001000 << 56;

pub const STARTPOS_WHITE_PAWNS:   u64 = 0b11111111;
pub const STARTPOS_WHITE_KNIGHTS: u64 = 0b01000010 << 8;
pub const STARTPOS_WHITE_BISHOPS: u64 = 0b00100100 << 8;
pub const STARTPOS_WHITE_ROOKS:   u64 = 0b10000001 << 8;
pub const STARTPOS_WHITE_QUEEN:   u64 = 0b00010000 << 8;
pub const STARTPOS_WHITE_KING:    u64 = 0b00001000 << 8;

pub const ZERO:                   u64 = 0;

pub struct ValueSides;
impl ValueSides {
    pub const BLACK:              usize = 0;
    pub const WHITE:              usize = 1;
}

pub struct ValuePieces;
impl ValuePieces {
    pub const PAWN:               usize = 0;
    pub const KNIGHT:             usize = 1;
    pub const BISHOP:             usize = 2;
    pub const ROOK:               usize = 3;
    pub const QUEEN:              usize = 4;
    pub const KING:               usize = 5;
}
