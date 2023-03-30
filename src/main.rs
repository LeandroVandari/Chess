use fnv::FnvHashSet;

use chess::multi_thread_eval;
use std::sync::{Mutex, Arc};

fn main() {
    let board = chess::Board::new();
    // println!("{board}");
    let positions = Arc::new(Mutex::new(FnvHashSet::default()));
    let depth = 1;
    multi_thread_eval(&board, depth, chess::Color::White, &positions);
}
