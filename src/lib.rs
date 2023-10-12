#![warn(clippy::pedantic)]
//! This crate has the purpose of fully simulating the game of chess, including an engine to play against (WIP). Currently, for means of speed, the crate is using `bitboards` for simulating the game. All basic simulation can be done through the [bitboard] module.

/// This module contains the game base, such as the [`Position`](bitboard::Position), moves etc.
pub mod bitboard;

#[must_use]
pub fn from_square(square: &str) -> bitboard::Mask {
    debug_assert_eq!(
        square.len(),
        2,
        "Square length should be 2 for string conversion to square"
    );
    bitboard::Mask::new(
        1 << (square
            .split("")
            .filter(|item| !item.is_empty())
            .enumerate()
            .map(|(i, letter)| {
                if i == 0 {
                    match letter {
                        "a" => 0,
                        "b" => 1,
                        "c" => 2,
                        "d" => 3,
                        "e" => 4,
                        "f" => 5,
                        "g" => 6,
                        "h" => 7,
                        _ => {
                            println!("{letter}");
                            panic!("Invalid coordinate: column is not in board")
                        }
                    }
                } else {
                    let new_val = (letter
                        .parse::<u8>()
                        .expect("Invalid coordinate: row isn't an integer")
                        - 1)
                        * 8;
                    if new_val <= 56 {
                        new_val
                    } else {
                        panic!("Invalid coordinate: row not in board")
                    }
                }
            })
            .sum::<u8>()),
    )
}
