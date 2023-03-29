use fnv::FnvHashSet;

use chess::{multi_thread_eval};

fn main() {
    let board = chess::Board::from_fen("r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R");
    println!("{board}");
    let mut positions = FnvHashSet::default();
    let depth = 6;
    multi_thread_eval(&board, depth, chess::Color::White, &mut positions);
}
