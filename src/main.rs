use std::collections::HashMap;

use chess::multi_thread_eval;

fn main() {
    let board = chess::Board::new(); //.make_move(9, chess::Move::PawnAdvanceTwoSquares(25), chess::Color::White);
                                     /*     let all_moves = board.generate_moves(chess::Color::White);
                                     println!("{all_moves:?}"); */
    let mut positions = HashMap::new();
    let depth = 3;
    multi_thread_eval(&board, depth, chess::Color::White, &mut positions);
    /*     let mut i = 0;
    positions.iter().for_each(|position| {
        i += 1;
        let mut board = chess::Board::new();
        board.board = *position;
        println!("{board}\n{i}")
    }); */
}
