use chess::bitboard::Position;
use criterion::{criterion_group, criterion_main, Criterion};

pub fn criterion_benchmark(c: &mut Criterion) {
    // let board = Position::example();
    // let other_board = Position::new();
    // let mut moves_list: [u64; 6] = [0; 6];

    c.bench_function("instantiate_board", |b| b.iter(|| Position::new()));
    /*  c.bench_function("pawn", |b| {
        b.iter(|| {
            board.board[9]
                .unwrap()
                .get_moves(&board, 9, &mut moves_list)
        })
    });
    c.bench_function("knight", |b| {
        b.iter(|| {
            board.board[19]
                .unwrap()
                .get_moves(&board, 19, &mut moves_list)
        })
    });
    c.bench_function("bishop", |b| {
        b.iter(|| {
            board.board[36]
                .unwrap()
                .get_moves(&board, 36, &mut moves_list)
        })
    });
    c.bench_function("rook", |b| {
        b.iter(|| {
            board.board[18]
                .unwrap()
                .get_moves(&board, 18, &mut moves_list)
        })
    });
    c.bench_function("queen", |b| {
        b.iter(|| {
            board.board[11]
                .unwrap()
                .get_moves(&board, 11, &mut moves_list)
        })
    });
    c.bench_function("king", |b| {
        b.iter(|| {
            board.board[17]
                .unwrap()
                .get_moves(&board, 17, &mut moves_list)
        })
    });
    c.bench_function("calculate_moves white", |b| {
        b.iter(|| {
            other_board.generate_moves(
                chess::Color::White,
                &mut moves_list,
                &mut all_pieces_moves_list,
            )
        })
    });
    c.bench_function("calculate_moves_black", |b| {
        b.iter(|| {
            other_board.generate_moves(
                chess::Color::Black,
                &mut moves_list,
                &mut all_pieces_moves_list,
            )
        })
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
    c.bench_function("five_moves_into_the_future", |b| {
        b.iter(|| {
            multi_thread_eval(
                &other_board,
                5,
                chess::Color::White,
                &mut FnvHashSet::default(),
            )
        })
    }); */
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
