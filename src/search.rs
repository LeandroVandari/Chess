use crate::{Color, Move, Piece};
use fnv::FnvHashSet;
use std::collections::HashMap;
use std::thread;

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

#[derive(Clone, Copy)]
enum HashSetRead {
    First,
    Second,
}

pub fn multi_thread_eval(
    board: &Board,
    depth: u8,
    start_color: Color,
    positions: FnvHashSet<[Option<Piece>; 64]>,
) {
    let moves = [None; 28];
    let moves = board.generate_moves(start_color, moves);
    let (tx, rx) = flume::unbounded();
    let positions_hashset_1 = &positions;
    let positions_hashset_2 = &positions.clone();
    let hashset_read = HashSetRead::First;

    thread::scope(|s| {
        if depth != 0 {
            for tuple in moves {
                let board = board.clone();
                let tx1 = tx.clone();
                s.spawn(move || {
                    let moves_list = [None;28];
                    let positions_list = [positions_hashset_1 as *const FnvHashSet<[Option<Piece>; 64]>, positions_hashset_2 as *const FnvHashSet<[Option<Piece>; 64]>];
                    while let Some(each_move) = *tuple.1.iter().next().unwrap_or(&None) {
                        let new_board = board.make_move(tuple.0 as usize, each_move, start_color);
                        if !(*positions_hashset_1).contains(&new_board.board) {
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
                                /* let _a = convert_to_square(tuple.0);
                                moves_each_tree = 0; */
                                evaluate(
                                    &new_board,
                                    depth - 1,
                                    start_color.reverse(),
                                    &next_board_moves,
                                    /* {a == "f2" && match each_move {
                                    Move::RegularMove(sqr) => convert_to_square(*sqr) == "d3",
                                    _=>false} } */
                                    false,
                                    tx1.clone(),
                                    &positions_list,
                                    &hashset_read, moves_list
                                );

                                // println!("{_a}{each_move}");
                                /* _amount_of_moves += moves_each_tree; */
                            }
                        }
                    }
                });
            }
            let mut board_counter = 0;
            let mut temp_list: [[Option<Piece>; 64]; 50] = [[None; 64]; 50];
            loop {
                match rx.try_recv() {
                    Ok(board) => {
                        temp_list[board_counter] = board;
                        board_counter += 1;
                    }

                    Err(e) => {
                        if let flume::TryRecvError::Disconnected = e {
                            panic!("Uhhhhhh, something didn't work")
                        }
                    }
                }
                if board_counter == 50 {}
            }
        }
    });
    //println!("{_amount_of_moves} positions analyzed")
}

fn evaluate(
    board: &Board,
    depth: u8,
    start_color: Color,
    moves: &HashMap<u8, [Option<Move>; 28]>,
    should_print: bool,
    tx: flume::Sender<[Option<Piece>; 64]>,
    positions_hashset_list: &[*const FnvHashSet<[Option<Piece>; 64]>; 2],
    hashset_read: &HashSetRead,
    moves_list: [Option<Move>; 28],
) {
    let positions = if let HashSetRead::First = hashset_read {
        positions_hashset_list[0]
    } else {
        positions_hashset_list[1]
    };
    if depth != 0 {
        for tuple in moves {
            while let Some(each_move) = *tuple.1.iter().next().unwrap_or(&None) {
                /*
                let a = convert_to_square(*tuple.0);
                if should_print {println!("{should_calc}, {a}{each_move}");} */

                let new_board = board.make_move(*tuple.0 as usize, each_move, start_color);
                if !(unsafe { &*positions }).contains(&new_board.board) {
                    let should_calc = can_castle!(board, each_move, start_color, moves_list);
                    let next_board_moves =
                        new_board.generate_moves(start_color.reverse(), moves_list);
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
                            &next_board_moves,
                            should_print,
                            tx.clone(),
                            positions_hashset_list,
                            hashset_read,
                            moves_list,
                        );
                    }
                } else {
                    let _ = tx.send(new_board.board);
                }
            }
        }
    }
    tx.send(board.board).unwrap();
}
fn is_check(moves: &HashMap<u8, [Option<Move>; 28]>, king_pos: u8) -> bool {
    moves.iter().any(|tuple| {
        tuple
            .1
            .iter()
            .map_while(|item| {
                if let Some(_) = *item {
                    return *item;
                } else {
                    return None;
                }
            })
            .any(|end_square| match end_square {
                Move::RegularMove(a_square) => a_square == king_pos,
                Move::PawnPromotion(a_square, _) => a_square == king_pos,
                _ => false,
            })
    })
}
