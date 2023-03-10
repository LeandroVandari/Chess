use crate::Color;

use super::Board;

pub fn evaluate(board: &Board, depth: u8, start_color: Color) {
    let moves = board.generate_moves(start_color);
    let _white_king = board.white_king_pos.get();
    let _black_king = board.black_king_pos.get();
    let mut positions: Vec<Board> = Vec::new();
    if depth != 0 {
        for tuple in moves {
            for sqr in tuple.1 {
                let new_board = board.make_move(tuple.0 as usize, sqr as usize);
                if !positions.contains(&new_board) {
                    positions.push(new_board.clone());
                }
                evaluate(&new_board, depth - 1, start_color.reverse());
            }
        }
    }
    for item in positions {
        println!("{item}");
    }
}
