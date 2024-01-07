#![warn(clippy::pedantic)]

//! This crate has the purpose of fully simulating the game of chess, including an engine to play against (WIP). Currently, for means of speed, the crate is using `bitboards` for simulating the game. All basic simulation can be done through the [bitboard] module.

/// This module contains the game base, such as the [`Position`](bitboard::Position), moves etc.
pub mod bitboard;

pub mod convert {

    pub mod from {

        pub mod algebraic_square {
            #[must_use]
            pub fn to_bitboard(square: &str) -> u64 {
                super::square_index::to_bitboard(to_square_index(square))
            }
            #[must_use]
            #[allow(clippy::cast_possible_truncation)]
            pub fn to_square_index(square: &str) -> u8 {
                debug_assert_eq!(
                    square.len(),
                    2,
                    "Square length should be 2 for string conversion to square"
                );
                let mut square_iter = square.chars();
                let column = match square_iter.next().unwrap() {
                    'a' => 0,
                    'b' => 1,
                    'c' => 2,
                    'd' => 3,
                    'e' => 4,
                    'f' => 5,
                    'g' => 6,
                    'h' => 7,
                    _ => panic!("Invalid column"),
                };
                let row = square_iter.next().unwrap().to_digit(10).unwrap();
                (8 * (row - 1) + column) as u8
            }
        }
        pub mod square_index {
            #[must_use]
            pub fn to_bitboard(index: u8) -> u64 {
                1 << index
            }
        }
        pub mod bitboard {
            #[must_use]
            pub fn to_algebraic_square(board: u64) -> String {
                let mut final_str = String::new();
                let mut u8val = 0;
                for i in 0..64u8 {
                    if 1 << i == board {
                        u8val = i;
                    }
                }
                final_str.push(match u8val % 8 {
                    0 => 'a',
                    1 => 'b',
                    2 => 'c',
                    3 => 'd',
                    4 => 'e',
                    5 => 'f',
                    6 => 'g',
                    7 => 'h',
                    _ => 'z',
                });

                final_str.push(char::from_digit(u32::from(u8val / 8) + 1, 10).unwrap());
                final_str
            }
        }
    }
}
