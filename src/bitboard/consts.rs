#![allow(clippy::unreadable_literal)]
pub mod boards {
    pub mod startpos {
        pub mod black {
            /// The starting position of the black pawns.
            pub const PAWN: u64 = 0b11111111 << 48;
            /// The starting position of the black knights.
            pub const KNIGHT: u64 = 0b01000010 << 56;
            /// The starting position of the black bishops.
            pub const BISHOP: u64 = 0b00100100 << 56;
            /// The starting position of the black rooks.
            pub const ROOK: u64 = 0b10000001 << 56;
            /// The starting position of the black queen.
            pub const QUEEN: u64 = 0b00001000 << 56;
            /// The starting position of the black king.
            pub const KING: u64 = 0b00010000 << 56;
            /// The starting position of all black pieces.
            pub const ALL: u64 = PAWN | KNIGHT | BISHOP | ROOK | QUEEN | KING;
        }

        pub mod white {
            /// The starting position of the white pawns.
            pub const PAWN: u64 = 0b11111111 << 8;
            /// The starting position of the white knights.
            pub const KNIGHT: u64 = 0b01000010;
            /// The starting position of the white bishops.
            pub const BISHOP: u64 = 0b00100100;
            /// The starting position of the white rooks.
            pub const ROOK: u64 = 0b10000001;
            /// The starting position of the white queen.
            pub const QUEEN: u64 = 0b00001000;
            /// The starting position of the white king.
            pub const KING: u64 = 0b00010000;

            /// The starting position of all white pieces.
            pub const ALL: u64 = PAWN | KNIGHT | BISHOP | ROOK | QUEEN | KING;
        }
        /// The starting position of all pieces.
        pub const ALL: u64 = black::ALL | white::ALL;
    }

    pub mod castling {
        pub mod kingside {
            pub mod black {
                pub const KING_AND_ROOK_POS: u64 = 0b01100000 << 56;
                pub const MUST_BE_FREE: u64 = KING_AND_ROOK_POS;
            }
            pub mod white {
                pub const KING_AND_ROOK_POS: u64 = 0b01100000;
                pub const MUST_BE_FREE: u64 = KING_AND_ROOK_POS;
            }
        }
        pub mod queenside {
            pub mod black {
                pub const MUST_BE_FREE: u64 = 0b00001110 << 56;
                pub const KING_AND_ROOK_POS: u64 = 0b00001100 << 56;
            }
            pub mod white {

                pub const MUST_BE_FREE: u64 = 0b00001110;
                pub const KING_AND_ROOK_POS: u64 = 0b00001100;
            }
        }
    }
}

pub mod rank {

    /// The 1st rank of the board (white pieces start).
    pub const ONE: u64 = 0xFF;
    /// The 2nd rank of the board (white pawns start).
    pub const TWO: u64 = 0xFF00;
    /// The 7th rank of the board (black pawns start).
    pub const SEVEN: u64 = TWO << 40;
    /// The 8th rank of the board (black pieces start).
    pub const EIGHT: u64 = 0xFF00000000000000;
}
pub mod file {
    /// The A file of the board.
    pub const A: u64 = 0x0101010101010101;
    /// The H file of the board.
    pub const H: u64 = A << 7;
}
pub mod pawn_after_moving_two_forward {
    /// All possible white pawn positions after moving two spaces forward in the starting position.
    pub const WHITE: u64 = super::rank::TWO << 16;
    /// All possible black pawn positions after moving two spaces forward in the starting position.
    pub const BLACK: u64 = super::rank::SEVEN >> 16;
}

pub mod pieces {
    pub const PAWN: usize = 0;
    pub const KNIGHT: usize = 1;
    pub const BISHOP: usize = 2;
    pub const ROOK: usize = 3;
    pub const QUEEN: usize = 4;
    pub const KING: usize = 5;
}

pub mod sides {
    pub const BLACK: usize = 0;
    pub const WHITE: usize = 1;
}
