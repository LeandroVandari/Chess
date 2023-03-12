use crate::{Color, Move, Piece};
use std::collections::{HashMap, HashSet};

use super::Board;

pub fn multi_thread_eval(
    board: &Board,
    depth: u8,
    start_color: Color,
    positions: &mut HashSet<[Option<Piece>; 64]>,
) {
    let moves = board.generate_moves(start_color);
    let mut amount_of_moves = 0;
    if depth != 0 {

        for tuple in &moves {
            for sqr in tuple.1 {
                let new_board = board.make_move(*tuple.0 as usize, *sqr, start_color);

                if !positions.contains(&new_board.board) {
                    let next_board_moves = new_board.generate_moves(start_color.reverse());
                    if !is_check(
                        &next_board_moves,
                        if let Color::White = start_color {
                            new_board.white_king_pos
                        } else {
                            new_board.black_king_pos
                        },
                    ) {
                        amount_of_moves += evaluate(
                            &new_board,
                            depth - 1,
                            start_color.reverse(),
                            positions,
                            &next_board_moves,
                        );
                    }
                }
            }
        }
    }
}

fn evaluate(
    board: &Board,
    depth: u8,
    start_color: Color,
    positions: &mut HashSet<[Option<Piece>; 64]>,
    moves: &HashMap<u8, Vec<Move>>,
) -> i32{
    let mut amount_of_moves = 0;
    if depth != 0 {
        for tuple in moves {
            for sqr in tuple.1 {
                let new_board = board.make_move(*tuple.0 as usize, *sqr, start_color);
                if !positions.contains(&new_board.board) {
                    let next_board_moves = board.generate_moves(start_color.reverse());
                    if !is_check(
                        &next_board_moves,
                        if let Color::White = start_color {
                            new_board.white_king_pos
                        } else {
                            new_board.black_king_pos
                        },
                    ) {
                        amount_of_moves += 1;
                        amount_of_moves += evaluate(
                            &new_board,
                            depth - 1,
                            start_color.reverse(),
                            positions,
                            &next_board_moves,
                        );
                    }
               }
            }
        }
    }
    positions.insert(board.board);
    amount_of_moves
}

fn is_check(moves: &HashMap<u8, Vec<Move>>, king_pos: u8) -> bool {
    moves.iter().any(|tuple| {
        tuple.1.iter().any(|end_square| {
            if let Move::RegularMove(a_square) = end_square {
                *a_square == king_pos
            } else {
                false
            }
        })
    })
}
