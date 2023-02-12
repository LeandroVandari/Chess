use chess::Board;
use criterion::{criterion_group, criterion_main, Criterion};

pub fn criterion_benchmark(c: &mut Criterion) {
    let board = Board::new();
    c.bench_function("instantiate_board", |b| b.iter(|| Board::new()));
    c.bench_function("calculate_moves", |b| b.iter(|| board.possible_movements()));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
