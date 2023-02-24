use chess::{Board};
use criterion::{criterion_group, criterion_main, Criterion};

pub fn criterion_benchmark(c: &mut Criterion) {
    let mut board = Board::example();
    c.bench_function("instantiate_board", |b| b.iter(|| Board::new()));
/*     c.bench_function("pawn", |b| b.iter(|| board.board[9].unwrap().generate_moves())); */
    
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
