use crate::{convert_to_square, /* from_square, */ Color, Move, Piece, from_square};
use fnv::FnvHashSet;
use std::collections::HashMap;

use super::Board;

pub fn multi_thread_eval(
    board: &Board,
    depth: u8,
    start_color: Color,
    positions: &mut FnvHashSet<[Option<Piece>; 64]>,
) {
    let moves = board.generate_moves(start_color);
    let mut amount_of_moves = 0;
    let mut moves_each_tree: i32;
    if depth != 0 {
        for tuple in &moves {
            for each_move in tuple.1 {
                let new_board = board.make_move(*tuple.0 as usize, *each_move, start_color);

                //if !positions.contains(&new_board.board) {
                let next_board_moves = new_board.generate_moves(start_color.reverse());
               /*  let mut should_print = false;
                if convert_to_square(*tuple.0) == "c4" && match each_move {
                    Move::RegularMove(sqr) => convert_to_square(*sqr) == "f7",
                    _=>false} {/* println!("{next_board_moves:?} <----------- WATCH THIS"); */ should_print = true} */
                
                if !is_check(
                    &next_board_moves,
                    if let Color::White = start_color {
                        new_board.white_king_pos
                    } else {
                        new_board.black_king_pos
                    },
                ) {
                    let a = convert_to_square(*tuple.0);
                    moves_each_tree = 0;
                    evaluate(
                        &new_board,
                        depth - 1,
                        start_color.reverse(),
                        positions,
                        &next_board_moves,
                        &mut moves_each_tree,
                        false
                    );

                    //println!("{a}{each_move}: {moves_each_tree}");
                    amount_of_moves += moves_each_tree;
                }
            }
            //}
        }
    }
  //  println!("{amount_of_moves}")
}

fn evaluate(
    board: &Board,
    depth: u8,
    start_color: Color,
    positions: &mut FnvHashSet<[Option<Piece>; 64]>,
    moves: &HashMap<u8, Vec<Move>>,
    amount_of_moves: &mut i32,
    should_print: bool
) {
    if depth != 0 {
        for tuple in moves {
            for each_move in tuple.1 {
                let new_board = board.make_move(*tuple.0 as usize, *each_move, start_color);
                // if !positions.contains(&new_board.board) {
                let next_board_moves = new_board.generate_moves(start_color.reverse());
                if !is_check(
                    &next_board_moves,
                    if let Color::White = start_color {
                        new_board.white_king_pos
                    } else {
                        new_board.black_king_pos
                    },
                ) {
                    evaluate(
                        &new_board,
                        depth - 1,
                        start_color.reverse(),
                        positions,
                        &next_board_moves,
                        amount_of_moves,
                        should_print
                    );
                } //else {let a = convert_to_square(*tuple.0);println!("{a}{each_move}")}
                //}
            }
        }
    } else {
        *amount_of_moves += 1;
    }
    //positions.insert(board.board);
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
