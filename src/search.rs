use crate::{Color, Piece};
use std::collections::HashSet;

use super::Board;

pub fn evaluate(
    board: &Board,
    depth: u8,
    start_color: Color,
    positions: &mut HashSet<[Option<Piece>; 64]>,
) {
    let moves = board.generate_moves(start_color);

    if depth != 0 {
        for tuple in &moves {
            for sqr in tuple.1 {
                let new_board = board.make_move(*tuple.0 as usize, *sqr, start_color);
                if !positions.contains(&new_board.board) {
                    evaluate(&new_board, depth - 1, start_color.reverse(), positions);
                }
            }
        }
    }
    positions.insert(board.board);
}
