use crate::{convert_to_square, Color, Move, OnePieceMoves, Piece, PositionMoves};
use fnv::FnvHashSet;
use itertools::Itertools;

use super::Board;

fn can_castle(
    board: &Board,
    each_move: Move,
    start_color: Color,
    moves_list: &mut OnePieceMoves,
    moves: &mut PositionMoves,
) -> bool {
    match each_move {
        Move::CastleKingside => {
            board.generate_moves(start_color.reverse(), moves_list, moves);
            !(if let Some(Piece::Pawn(pawn)) = if let Color::White = start_color {
                board.board[14]
            } else {
                board.board[54]
            } {
                pawn.color != start_color
            } else {
                false
            } || if let Some(Piece::Pawn(pawn)) = if let Color::White = start_color {
                board.board[12]
            } else {
                board.board[52]
            } {
                pawn.color != start_color
            } else {
                false
            } || if let Some(Piece::Pawn(pawn)) = if let Color::White = start_color {
                board.board[15]
            } else {
                board.board[55]
            } {
                pawn.color != start_color
            } else {
                false
            } || is_check(
                moves,
                if let Color::White = start_color {
                    4
                } else {
                    60
                },
            ) || is_check(
                moves,
                if let Color::White = start_color {
                    5
                } else {
                    61
                },
            ))
        }
        Move::CastleQueenside => {
            board.generate_moves(start_color.reverse(), moves_list, moves);
            !(if let Some(Piece::Pawn(pawn)) = if let Color::White = start_color {
                board.board[9]
            } else {
                board.board[50]
            } {
                pawn.color != start_color
            } else {
                false
            } || if let Some(Piece::Pawn(pawn)) = if let Color::White = start_color {
                board.board[12]
            } else {
                board.board[52]
            } {
                pawn.color != start_color
            } else {
                false
            } || if let Some(Piece::Pawn(pawn)) = if let Color::White = start_color {
                board.board[10]
            } else {
                board.board[49]
            } {
                pawn.color != start_color
            } else {
                false
            } || is_check(
                moves,
                if let Color::White = start_color {
                    4
                } else {
                    60
                },
            ) || is_check(
                moves,
                if let Color::White = start_color {
                    3
                } else {
                    59
                },
            ))
        }
        _ => true,
    }
}

pub fn multi_thread_eval(
    board: &Board,
    depth: u8,
    start_color: Color,
    positions: &mut FnvHashSet<[Option<Piece>; 64]>,
) {
    let mut moves: PositionMoves = [None; 16];
    board.generate_moves(start_color, &mut [None; 28], &mut moves);
    let mut _amount_of_moves = 0;
    let mut moves_each_tree: i32;
    let mut moves_list = [None; 28];

    if depth != 0 {
        let mut moves_iter = moves.into_iter();
        while let Some(tuple) = moves_iter.next().unwrap_or(None) {
            let mut all_moves = tuple.1.iter();
            while let Some(each_move) = *all_moves.next().unwrap_or(&None) {
                let new_board = board.make_move(tuple.0 as usize, each_move, start_color);

                //if !positions.contains(&new_board.board) {
                let should_calc =
                    can_castle(board, each_move, start_color, &mut moves_list, &mut moves);
                new_board.generate_moves(start_color.reverse(), &mut moves_list, &mut moves);

                /*  let mut should_print = false;
                if convert_to_square(*tuple.0) == "c4" && match each_move {
                    Move::RegularMove(sqr) => convert_to_square(*sqr) == "f7",
                    _=>false} {/* println!("{next_board_moves:?} <----------- WATCH THIS"); */ should_print = true} */

                if should_calc
                    && !is_check(
                        &moves,
                        if let Color::White = start_color {
                            new_board.white_king_pos
                        } else {
                            new_board.black_king_pos
                        },
                    )
                {
                    let _a = convert_to_square(tuple.0);
                    moves_each_tree = 0;
                    evaluate(
                        &new_board,
                        depth,
                        start_color.reverse(),
                        positions,
                        &mut moves,
                        &mut moves_each_tree,
                        &mut moves_list, /* {a == "f2" && match each_move {
                                         Move::RegularMove(sqr) => convert_to_square(*sqr) == "d3",
                                         _=>false} } */
                    );

                    println!("{_a}{each_move}: {moves_each_tree}");
                    _amount_of_moves += moves_each_tree;
                }
                //} else {println!("{new_board}")}
            }
        }
    }
    println!("{_amount_of_moves}")
}

fn evaluate(
    board: &Board,
    depth: u8,
    start_color: Color,
    positions: &mut FnvHashSet<[Option<Piece>; 64]>,
    moves: &mut PositionMoves,
    amount_of_moves: &mut i32,
    moves_list: &mut OnePieceMoves,
) {
    let depth = depth - 1;
    if depth != 0 {
        let moves_clone = *moves; // creating a copy of the list, otherwise it throws errors
        let mut moves_iter = moves_clone.iter();
        while let Some(tuple) = moves_iter.next().unwrap_or(&None) {
            let mut all_moves = tuple.1.iter();
            while let Some(each_move) = *all_moves.next().unwrap_or(&None) {
                /*
                let a = convert_to_square(*tuple.0);
                if should_print {println!("{should_calc}, {a}{each_move}");} */

                let new_board = board.make_move(tuple.0 as usize, each_move, start_color);
                //if !positions.contains(&new_board.board) {
                let should_calc = can_castle(board, each_move, start_color, moves_list, moves);
                new_board.generate_moves(start_color.reverse(), moves_list, moves);

                if should_calc
                    && !is_check(
                        moves,
                        if let Color::White = start_color {
                            new_board.white_king_pos
                        } else {
                            new_board.black_king_pos
                        },
                    )
                {
                    evaluate(
                        &new_board,
                        depth,
                        start_color.reverse(),
                        positions,
                        moves,
                        amount_of_moves,
                        moves_list,
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

fn is_check(moves: &PositionMoves, king_pos: u8) -> bool {
    moves
        .iter()
        .fold_while(false, |_, moves_option| {
            if let Some(tuple) = moves_option {
                if tuple
                    .1
                    .iter()
                    .fold_while(false, |_, item| {
                        if let Some(a_move) = *item {
                            let a = match a_move {
                                Move::RegularMove(a_square) => a_square == king_pos,
                                Move::PawnPromotion(a_square, _) => a_square == king_pos,
                                _ => false,
                            };
                            if a {
                                itertools::FoldWhile::Done(true)
                            } else {
                                itertools::FoldWhile::Continue(false)
                            }
                        } else {
                            itertools::FoldWhile::Done(false)
                        }
                    })
                    .into_inner()
                {
                    itertools::FoldWhile::Done(true)
                } else {
                    itertools::FoldWhile::Continue(false)
                }
            } else {
                itertools::FoldWhile::Done(false)
            }
        })
        .into_inner()
}
