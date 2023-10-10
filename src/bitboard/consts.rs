#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(clippy::unreadable_literal)]

/// The starting position of the black pawns.
pub const STARTPOS_BLACK_PAWNS:                u64 = 0b11111111 << 48;
/// The starting position of the black knights.
pub const STARTPOS_BLACK_KNIGHTS:              u64 = 0b01000010 << 56;
/// The starting position of the black bishops.
pub const STARTPOS_BLACK_BISHOPS:              u64 = 0b00100100 << 56;
/// The starting position of the black rooks.
pub const STARTPOS_BLACK_ROOKS:                u64 = 0b10000001 << 56;
/// The starting position of the black queen.
pub const STARTPOS_BLACK_QUEEN:                u64 = 0b00001000 << 56;
/// The starting position of the black king.
pub const STARTPOS_BLACK_KING:                 u64 = 0b00010000 << 56;
pub const MUST_BE_FREE_CASTLE_KINGSIDE_BLACK:  u64 = 0b01100000 << 56;
pub const MUST_BE_FREE_CASTLE_QUEENSIDE_BLACK: u64 = 0b00001110 << 56;
pub const CASTLE_KINGSIDE_BLACK:               u64 = 0b01100000 << 56;
pub const CASTLE_QUEENSIDE_BLACK:              u64 = 0b00001100 << 56;

/// The starting position of the white pawns.
pub const STARTPOS_WHITE_PAWNS:                u64 = 0b11111111 << 8;
/// The starting position of the white knights.
pub const STARTPOS_WHITE_KNIGHTS:              u64 = 0b01000010;
/// The starting position of the white bishops.
pub const STARTPOS_WHITE_BISHOPS:              u64 = 0b00100100;
/// The starting position of the white rooks.
pub const STARTPOS_WHITE_ROOKS:                u64 = 0b10000001;
/// The starting position of the white queen.
pub const STARTPOS_WHITE_QUEEN:                u64 = 0b00001000;
/// The starting position of the white king.
pub const STARTPOS_WHITE_KING:                 u64 = 0b00010000;
pub const MUST_BE_FREE_CASTLE_KINGSIDE_WHITE:  u64 = 0b01100000;
pub const MUST_BE_FREE_CASTLE_QUEENSIDE_WHITE: u64 = 0b00001110;
pub const CASTLE_KINGSIDE_WHITE:               u64 = 0b01100000;
pub const CASTLE_QUEENSIDE_WHITE:              u64 = 0b00001100;

/// The starting position of all black pieces.
pub const STARTPOS_BLACK:                      u64 = STARTPOS_BLACK_PAWNS | STARTPOS_BLACK_KNIGHTS | STARTPOS_BLACK_BISHOPS | STARTPOS_BLACK_ROOKS | STARTPOS_BLACK_QUEEN | STARTPOS_BLACK_KING;
/// The starting position of all white pieces.
pub const STARTPOS_WHITE:                      u64 = STARTPOS_WHITE_PAWNS | STARTPOS_WHITE_KNIGHTS | STARTPOS_WHITE_BISHOPS | STARTPOS_WHITE_ROOKS | STARTPOS_WHITE_QUEEN | STARTPOS_WHITE_KING;
/// The starting position of all pieces.
pub const STARTPOS_ALL:                        u64 = STARTPOS_BLACK | STARTPOS_WHITE;

/// The 1st rank of the board (white pieces start).
pub const RANK_ONE:                            u64 = 0xFF;
/// The 2nd rank of the board (white pawns start).
pub const RANK_TWO:                            u64 = 0xFF00;
/// The 7th rank of the board (black pawns start).
pub const RANK_SEVEN:                          u64 = RANK_TWO << 40;
/// The 8th rank of the board (black pieces start).
pub const RANK_EIGHT:                          u64 = 0xFF00000000000000;

/// All possible black pawn positions after moving two spaces forward in the starting position.
pub const PAWN_BLACK_AFTER_MOVE_TWO_FORWARD:   u64 = RANK_SEVEN >> 16;
/// All possible white pawn positions after moving two spaces forward in the starting position.
pub const PAWN_WHITE_AFTER_MOVE_TWO_FORWARD:   u64 = RANK_TWO << 16;

/// The A file of the board.
pub const A_FILE:                              u64 = 0x0101010101010101;
/// The H file of the board.
pub const H_FILE:                              u64 = A_FILE << 7;

/// The H file and 8th rank.
pub const H_AND_8:                             u64 = H_FILE | RANK_EIGHT;
/// The H file and 1st rank.
pub const H_AND_1:                             u64 = H_FILE | RANK_ONE;

/// The A file and 8th rank.
pub const A_AND_8:                             u64 = A_FILE | RANK_EIGHT;
/// The A file and 1st rank.
pub const A_AND_1:                             u64 = A_FILE | RANK_ONE;


pub const PAWN:                            usize = 0;
pub const KNIGHT:                          usize = 1;
pub const BISHOP:                          usize = 2;
pub const ROOK:                            usize = 3;
pub const QUEEN:                           usize = 4;
pub const KING:                            usize = 5;

pub const BLACK:                           usize = 0;
pub const WHITE:                           usize = 1;