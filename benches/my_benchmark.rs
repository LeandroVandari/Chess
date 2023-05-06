use chess::{board::*, multi_thread_eval};
use criterion::{criterion_group, criterion_main, Criterion};
use fnv::FnvHashSet;

pub fn criterion_benchmark(c: &mut Criterion) {
    let board = Board::example();
    let other_board = Board::new();
    let moves_list = [None;28];

    c.bench_function("instantiate_board", |b| b.iter(|| Board::new()));
    c.bench_function("pawn", |b| {
        b.iter(|| board.board[9].unwrap().get_moves(&board, 9, moves_list))
    });
    c.bench_function("knight", |b| {
        b.iter(|| board.board[19].unwrap().get_moves(&board, 19, moves_list))
    });
    c.bench_function("bishop", |b| {
        b.iter(|| board.board[36].unwrap().get_moves(&board, 36, moves_list))
    });
    c.bench_function("rook", |b| {
        b.iter(|| board.board[18].unwrap().get_moves(&board, 18, moves_list))
    });
    c.bench_function("queen", |b| {
        b.iter(|| board.board[11].unwrap().get_moves(&board, 11, moves_list))
    });
    c.bench_function("king", |b| {
        b.iter(|| board.board[17].unwrap().get_moves(&board, 17, moves_list))
    });
    c.bench_function("calculate_moves white", |b| {
        b.iter(|| other_board.generate_moves(chess::Color::White, moves_list))
    });
    c.bench_function("calculate_moves_black", |b| {
        b.iter(|| other_board.generate_moves(chess::Color::Black, moves_list))
    });
    c.bench_function("one_move_into_the_future", |b| {
        b.iter(|| {
            multi_thread_eval(
                &other_board,
                1,
                chess::Color::White,
                &mut FnvHashSet::default(),
            )
        })
    });
    c.bench_function("two_moves_into_the_future", |b| {
        b.iter(|| {
            multi_thread_eval(
                &other_board,
                2,
                chess::Color::White,
                &mut FnvHashSet::default(),
            )
        })
    });
    c.bench_function("three_moves_into_the_future", |b| {
        b.iter(|| {
            multi_thread_eval(
                &other_board,
                3,
                chess::Color::White,
                &mut FnvHashSet::default(),
            )
        })
    });
    c.bench_function("four_moves_into_the_future", |b| {
        b.iter(|| {
            multi_thread_eval(
                &other_board,
                4,
                chess::Color::White,
                &mut FnvHashSet::default(),
            )
        })
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);