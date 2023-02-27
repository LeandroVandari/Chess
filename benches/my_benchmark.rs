use chess::board::*;
use criterion::{criterion_group, criterion_main, Criterion};

pub fn criterion_benchmark(c: &mut Criterion) {
    let board = Board::example();
    c.bench_function("instantiate_board", |b| b.iter(|| Board::new()));
    c.bench_function("pawn", |b| {
        b.iter(|| board.board[9].unwrap().get_moves(&board.board, 9))
    });
    c.bench_function("knight", |b| {
        b.iter(|| board.board[19].unwrap().get_moves(&board.board, 19))
    });
    c.bench_function("bishop", |b| {
        b.iter(|| board.board[36].unwrap().get_moves(&board.board, 36))
    });
    c.bench_function("rook", |b| {
        b.iter(|| board.board[18].unwrap().get_moves(&board.board, 18))
    });
    c.bench_function("queen", |b| {
        b.iter(|| board.board[11].unwrap().get_moves(&board.board, 11))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
