use crate::{Color, Piece, Move};
use std::collections::HashMap;

use super::Board;

pub fn evaluate(board: &Board, depth: u8, start_color: Color, positions: &mut HashMap<[Option<Piece>; 64], HashMap<u8, Vec<Move>>>) {
    let moves = board.generate_moves(start_color);
    let _white_king = board.white_king_pos;
    let _black_king = board.black_king_pos;
    if depth != 0 {
        for tuple in &moves {
            for sqr in tuple.1 {
                let new_board = board.make_move(*tuple.0 as usize, *sqr, start_color);
                evaluate(&new_board, depth - 1, start_color.reverse(), positions);
            }
        }
    }
    positions.insert(board.board, moves);

}
