#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(clippy::unreadable_literal)]


pub const STARTPOS_BLACK_PAWNS:              u64 = 0b11111111 << 48;
pub const STARTPOS_BLACK_KNIGHTS:            u64 = 0b01000010 << 56;
pub const STARTPOS_BLACK_BISHOPS:            u64 = 0b00100100 << 56;
pub const STARTPOS_BLACK_ROOKS:              u64 = 0b10000001 << 56;
pub const STARTPOS_BLACK_QUEEN:              u64 = 0b00001000 << 56;
pub const STARTPOS_BLACK_KING:               u64 = 0b00010000 << 56;

pub const STARTPOS_WHITE_PAWNS:              u64 = 0b11111111 << 8;
pub const STARTPOS_WHITE_KNIGHTS:            u64 = 0b01000010;
pub const STARTPOS_WHITE_BISHOPS:            u64 = 0b00100100;
pub const STARTPOS_WHITE_ROOKS:              u64 = 0b10000001;
pub const STARTPOS_WHITE_QUEEN:              u64 = 0b00001000;
pub const STARTPOS_WHITE_KING:               u64 = 0b00010000;

pub const STARTPOS_BLACK:                    u64 = STARTPOS_BLACK_PAWNS | STARTPOS_BLACK_KNIGHTS | STARTPOS_BLACK_BISHOPS | STARTPOS_BLACK_ROOKS | STARTPOS_BLACK_QUEEN | STARTPOS_BLACK_KING;
pub const STARTPOS_WHITE:                    u64 = STARTPOS_WHITE_PAWNS | STARTPOS_WHITE_KNIGHTS | STARTPOS_WHITE_BISHOPS | STARTPOS_WHITE_ROOKS | STARTPOS_WHITE_QUEEN | STARTPOS_WHITE_KING;
pub const STARTPOS_ALL:                      u64 = STARTPOS_BLACK | STARTPOS_WHITE;

pub const ZERO:                              u64 = 0;


pub const FIRST_ROW:                         u64 = 0xFF;
pub const SECOND_ROW:                        u64 = 0xFF00;
pub const SEVENTH_ROW:                       u64 = SECOND_ROW << 40;
pub const LAST_ROW:                          u64 = 0xFF00000000000000;

pub const BLACK_PAWN_AFTER_MOVE_TWO_FORWARD: u64 = SEVENTH_ROW >> 16;
pub const WHITE_PAWN_AFTER_MOVE_TWO_FORWARD: u64 = SECOND_ROW << 16;

pub const A_FILE:                            u64 = 0x0101010101010101;
pub const H_FILE:                            u64 = A_FILE << 7;


pub const RIGHT_AND_UP:                      u64 = H_FILE | LAST_ROW;
pub const RIGHT_AND_DOWN:                    u64 = H_FILE | FIRST_ROW;

pub const LEFT_AND_UP:                       u64 = A_FILE | LAST_ROW;
pub const LEFT_AND_DOWN:                     u64 = A_FILE | FIRST_ROW;
