use fnv::FnvHashSet;

use chess::{multi_thread_eval};

fn main() {
    let board = chess::Board::new();
    println!("{board}");
    let mut positions = FnvHashSet::default();
    let depth = 7;
    multi_thread_eval(&board, depth, chess::Color::White, &mut positions);
}