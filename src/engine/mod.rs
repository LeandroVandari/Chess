use crate::bitboard::{
    consts,
    move_generation::Move,
    Board, Position,
};

pub fn get_best_move(pos: &Position) -> (Move, i32) {
    
}

fn evaluate_board(position: &Position, attacks: u64) -> i32 {
    let current_color = &position.to_move;
    if position.board.is_check(attacks, &current_color.reversed()) {
        return if current_color.is_white() {
            i32::MAX
        } else {
            i32::MIN
        };
    }

    sum_piece_values(&position.board)
}

fn sum_piece_values(board: &Board) -> i32 {
    fn sum_values_color(color: usize, board: &Board) -> u32 {
        board.pieces[color]
            .iter()
            .enumerate()
            .fold(0, |acc, (idx, piece)| {
                acc + (consts::pieces::values::LIST[idx] * piece.inner().count_ones())
            })
    }
    let white = sum_values_color(consts::sides::WHITE, board);
    let black = sum_values_color(consts::sides::BLACK, board);

    white as i32 - black as i32
}
