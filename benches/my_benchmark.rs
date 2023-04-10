use chess::{board::*, multi_thread_eval};
use criterion::{criterion_group, criterion_main, Criterion};
use fnv::FnvHashSet;

pub fn criterion_benchmark(c: &mut Criterion) {
    let board = Board::example();
    let other_board = Board::new();

    c.bench_function("instantiate_board", |b| b.iter(|| Board::new()));
    c.bench_function("pawn", |b| {
        b.iter(|| board.board[9].unwrap().get_moves(&board, 9))
    });
    c.bench_function("knight", |b| {
        b.iter(|| board.board[19].unwrap().get_moves(&board, 19))
    });
    c.bench_function("bishop", |b| {
        b.iter(|| board.board[36].unwrap().get_moves(&board, 36))
    });
    c.bench_function("rook", |b| {
        b.iter(|| board.board[18].unwrap().get_moves(&board, 18))
    });
    c.bench_function("queen", |b| {
        b.iter(|| board.board[11].unwrap().get_moves(&board, 11))
    });
    c.bench_function("king", |b| {
        b.iter(|| board.board[17].unwrap().get_moves(&board, 17))
    });
    c.bench_function("calculate_moves white", |b| {
        b.iter(|| other_board.generate_moves(chess::Color::White))
    });
    c.bench_function("calculate_moves_black", |b| {
        b.iter(|| other_board.generate_moves(chess::Color::Black))
    });
    c.bench_function("one_move_into_the_future", |b| {
        b.iter(|| multi_thread_eval(&other_board, 1, chess::Color::White, FnvHashSet::default()))
    });
    c.bench_function("two_moves_into_the_future", |b| {
        b.iter(|| multi_thread_eval(&other_board, 2, chess::Color::White, FnvHashSet::default()))
    });
    c.bench_function("three_moves_into_the_future", |b| {
        b.iter(|| multi_thread_eval(&other_board, 3, chess::Color::White, FnvHashSet::default()))
    });
    c.bench_function("four_moves_into_the_future", |b| {
        b.iter(|| multi_thread_eval(&other_board, 4, chess::Color::White, FnvHashSet::default()))
    });
    c.bench_function("heavy_five_moves_into_the_future", |b| {
        b.iter(|| multi_thread_eval(&other_board, 5, chess::Color::White, FnvHashSet::default()))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
