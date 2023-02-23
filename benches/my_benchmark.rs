use chess::{Board};
use criterion::{criterion_group, criterion_main, Criterion};

pub fn criterion_benchmark(c: &mut Criterion) {
    
    c.bench_function("instantiate_board", |b| b.iter(|| Board::new()));
    
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
