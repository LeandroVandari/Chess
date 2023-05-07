use crate::{convert_to_square, Color, Move, Piece};
use fnv::FnvHashSet;
use std::collections::HashMap;
use itertools::Itertools;

use super::Board;

macro_rules! can_castle {
    ($board: ident, $each_move: ident, $start_color: ident, $moves_list:ident) => {
        match $each_move {
            Move::CastleKingside => {
                let prev_moves = &$board.generate_moves($start_color.reverse(), $moves_list);
                if if let Some(Piece::Pawn(pawn)) = if let Color::White = $start_color {
                    $board.board[14]
                } else {
                    $board.board[54]
                } {
                    pawn.color != $start_color
                } else {
                    false
                } || if let Some(Piece::Pawn(pawn)) = if let Color::White = $start_color {
                    $board.board[12]
                } else {
                    $board.board[52]
                } {
                    pawn.color != $start_color
                } else {
                    false
                } || if let Some(Piece::Pawn(pawn)) = if let Color::White = $start_color {
                    $board.board[15]
                } else {
                    $board.board[55]
                } {
                    pawn.color != $start_color
                } else {
                    false
                } || is_check(
                    prev_moves,
                    if let Color::White = $start_color {
                        4
                    } else {
                        60
                    },
                ) || is_check(
                    prev_moves,
                    if let Color::White = $start_color {
                        5
                    } else {
                        61
                    },
                ) {
                    false
                } else {
                    true
                }
            }
            Move::CastleQueenside => {
                let prev_moves = &$board.generate_moves($start_color.reverse(), $moves_list);
                if if let Some(Piece::Pawn(pawn)) = if let Color::White = $start_color {
                    $board.board[9]
                } else {
                    $board.board[50]
                } {
                    pawn.color != $start_color
                } else {
                    false
                } || if let Some(Piece::Pawn(pawn)) = if let Color::White = $start_color {
                    $board.board[12]
                } else {
                    $board.board[52]
                } {
                    pawn.color != $start_color
                } else {
                    false
                } || if let Some(Piece::Pawn(pawn)) = if let Color::White = $start_color {
                    $board.board[10]
                } else {
                    $board.board[49]
                } {
                    pawn.color != $start_color
                } else {
                    false
                } || is_check(
                    prev_moves,
                    if let Color::White = $start_color {
                        4
                    } else {
                        60
                    },
                ) || is_check(
                    prev_moves,
                    if let Color::White = $start_color {
                        3
                    } else {
                        59
                    },
                ) {
                    false
                } else {
                    true
                }
            }
            _ => true,
        }
    };
}

pub fn multi_thread_eval(
    board: &Board,
    depth: u8,
    start_color: Color,
    positions: &mut FnvHashSet<[Option<Piece>; 64]>,
) {
    let moves = board.generate_moves(start_color, [None;28]);
    let mut amount_of_moves = 0;
    let mut moves_each_tree: i32;
    let moves_list = [None;28];
    if depth != 0 {
        for tuple in &moves {
            let mut all_moves = tuple.1.iter();
            while let Some(each_move) = *all_moves.next().unwrap_or(&None) {
                let new_board = board.make_move(*tuple.0 as usize, each_move, start_color);

                //if !positions.contains(&new_board.board) {

                let should_calc = can_castle!(board, each_move, start_color, moves_list);
                let next_board_moves = new_board.generate_moves(start_color.reverse(), moves_list);
                /*  let mut should_print = false;
                if convert_to_square(*tuple.0) == "c4" && match each_move {
                    Move::RegularMove(sqr) => convert_to_square(*sqr) == "f7",
                    _=>false} {/* println!("{next_board_moves:?} <----------- WATCH THIS"); */ should_print = true} */

                if should_calc
                    && !is_check(
                        &next_board_moves,
                        if let Color::White = start_color {
                            new_board.white_king_pos
                        } else {
                            new_board.black_king_pos
                        },
                    )
                {
                    let a = convert_to_square(*tuple.0);
                    moves_each_tree = 0;
                    evaluate(
                        &new_board,
                        depth - 1,
                        start_color.reverse(),
                        positions,
                        &next_board_moves,
                        &mut moves_each_tree,
                        /* {a == "f2" && match each_move {
                        Move::RegularMove(sqr) => convert_to_square(*sqr) == "d3",
                        _=>false} } */
                        false,
                    );

                   //println!("{a}{each_move}: {moves_each_tree}");
                   // amount_of_moves += moves_each_tree;
                }
                //} else {println!("{new_board}")}
            }
        }
    }
   // println!("{amount_of_moves}")
}

fn evaluate(
    board: &Board,
    depth: u8,
    start_color: Color,
    positions: &mut FnvHashSet<[Option<Piece>; 64]>,
    moves: &HashMap<u8, [Option<Move>;28]>,
    amount_of_moves: &mut i32,
    should_print: bool,
) {
    let moves_list = [None;28];
    if depth != 0 {
        for tuple in moves {
            let mut all_moves = tuple.1.iter();
            while let Some(each_move) = *all_moves.next().unwrap_or(&None) {
                /*
                let a = convert_to_square(*tuple.0);
                if should_print {println!("{should_calc}, {a}{each_move}");} */
                let new_board = board.make_move(*tuple.0 as usize, each_move, start_color);
                //if !positions.contains(&new_board.board) {
                let should_calc = can_castle!(board, each_move, start_color, moves_list);
                let next_board_moves = new_board.generate_moves(start_color.reverse(), moves_list);
                if should_calc
                    && !is_check(
                        &next_board_moves,
                        if let Color::White = start_color {
                            new_board.white_king_pos
                        } else {
                            new_board.black_king_pos
                        },
                    )
                {
                    evaluate(
                        &new_board,
                        depth - 1,
                        start_color.reverse(),
                        positions,
                        &next_board_moves,
                        amount_of_moves,
                        should_print,
                    );
                }
                //}
            }
        }
    } else {
        *amount_of_moves += 1;
    }
    //positions.insert(board.board);
}

fn is_check(moves: &HashMap<u8, [Option<Move>; 28]>, king_pos: u8) -> bool {
    moves.iter().any(|tuple| {
        tuple
            .1
            .iter()
            .fold_while(false, |_, item| 
                {if let Some(a_move) = *item{
                    let a = match a_move {
                        Move::RegularMove(a_square) => a_square == king_pos,
                        Move::PawnPromotion(a_square, _) => a_square == king_pos,
                        _ => false};
                    if a {itertools::FoldWhile::Done(true)} else {itertools::FoldWhile::Continue(false)}
                } else {itertools::FoldWhile::Done(false)}}).into_inner()
    })
}