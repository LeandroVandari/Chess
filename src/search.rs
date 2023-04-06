use crate::{Color, Move, Piece};
use fnv::FnvHashSet;
use std::collections::HashMap;
use std::thread;

use super::Board;

macro_rules! can_castle {
    ($board: ident, $each_move: ident, $start_color: ident) => {
        match $each_move {
            Move::CastleKingside => {
                let prev_moves = &$board.generate_moves($start_color.reverse());
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
                let prev_moves = &$board.generate_moves($start_color.reverse());
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
    positions: FnvHashSet<[Option<Piece>; 64]>,
) { 
    let moves = board.generate_moves(start_color);
    let mut handles = vec![];
    let positions_read = &positions;
    let (tx, rx) = flume::unbounded();

    if depth != 0 {
        for tuple in moves {
            let board = board.clone();
            let tx1 = tx.clone();
            let handle = thread::spawn(move || {
                for each_move in tuple.1 {
                    let new_board = board.make_move(tuple.0 as usize, each_move, start_color);
                    if !positions_read.contains(&new_board.board) {
                        let should_calc = can_castle!(board, each_move, start_color);
                        let next_board_moves = new_board.generate_moves(start_color.reverse());
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
                            /* let _a = convert_to_square(tuple.0);
                            moves_each_tree = 0; */
                            evaluate(
                                &new_board,
                                depth - 1,
                                start_color.reverse(),
                                &positions_read,
                                &next_board_moves,
                                /* {a == "f2" && match each_move {
                                Move::RegularMove(sqr) => convert_to_square(*sqr) == "d3",
                                _=>false} } */
                                false,
                                tx1.clone(),
                            );

                            // println!("{_a}{each_move}");
                            /* _amount_of_moves += moves_each_tree; */
                        }
                    }
                }
            });
            handles.push(handle);
        }
        for received in rx {
            todo!()
        }
        
    }
    for handle in handles {
        handle.join().unwrap()
    }
    //println!("{_amount_of_moves} positions analyzed")
}

fn evaluate(
    board: &Board,
    depth: u8,
    start_color: Color,
    positions: &FnvHashSet<[Option<Piece>; 64]>,
    moves: &HashMap<u8, Vec<Move>>,
    should_print: bool,
    tx: flume::Sender<[Option<Piece>; 64]>,
) {
    if depth != 0 {
        for tuple in moves {
            for each_move in tuple.1 {
                /*
                let a = convert_to_square(*tuple.0);
                if should_print {println!("{should_calc}, {a}{each_move}");} */
                let new_board = board.make_move(*tuple.0 as usize, *each_move, start_color);
                if !positions.contains(&new_board.board) {
                    let should_calc = can_castle!(board, each_move, start_color);
                    let next_board_moves = new_board.generate_moves(start_color.reverse());
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

                            should_print,
                            tx.clone(),
                        );
                    }
                }
            }
        }
    } 
    tx.send(board.board);
}

fn is_check(moves: &HashMap<u8, Vec<Move>>, king_pos: u8) -> bool {
    moves.iter().any(|tuple| {
        tuple.1.iter().any(|end_square| match end_square {
            Move::RegularMove(a_square) => *a_square == king_pos,
            Move::PawnPromotion(a_square, _) => *a_square == king_pos,
            _ => false,
        })
    })
}
