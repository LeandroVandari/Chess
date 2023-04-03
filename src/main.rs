use fnv::FnvHashSet;

use chess::multi_thread_eval;


fn main() {
    let board = chess::Board::new();
    // println!("{board}");
    let positions = FnvHashSet::default();
    let depth = 6;
    multi_thread_eval(&board, depth, chess::Color::White, positions);
}
