//! This crate has the purpose of fully simulating the game of chess, including an engine to play against (WIP). Currently, for means of speed, the crate is using `bitboards` for simulating the game. All basic simulation can be done through the [bitboard] module.

/// This module contains the game base, such as the [`Position`](bitboard::Position), moves etc.
pub mod bitboard;
