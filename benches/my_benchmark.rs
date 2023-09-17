use chess::bitboard::{pieces::PieceTypes, Color, Move, Moves, Position};
use criterion::{criterion_group, criterion_main, Criterion};

pub fn criterion_benchmark(c: &mut Criterion) {
    const MOVE: Move = Move(0);
    let board = Position::example();
    let other_board = Position::new();
    let mut moves_list: [Move; 16] = [MOVE; 16];
    let mut moves_list2: [Move; 16] = [MOVE; 16];
    let mut pieces_list: [u64; 16] = [0; 16];
    let b_white = board.get_board(&Color::White, None);
    let b_black = board.get_board(&Color::Black, None);

    let mut moves_struct = Moves::new(
        b_white,
        b_black,
        &mut moves_list,
        &mut pieces_list,
        None,
        &Color::White,
    );

    c.bench_function("instantiate_board", |b| b.iter(|| Position::new()));

    c.bench_function("pawn", |b| {
        b.iter(|| {
            board
                .get_piece(&Color::White, PieceTypes::Pawn)
                .generate_pawn_moves(&mut moves_struct);
        })
    });
    moves_struct.clear();
    c.bench_function("knight", |b| {
        b.iter(|| {
            board
                .get_piece(&Color::White, PieceTypes::Knight)
                .generate_knight_moves(&mut moves_struct);
        })
    });
    moves_struct.clear();
    c.bench_function("bishop", |b| {
        b.iter(|| {
            board
                .get_piece(&Color::White, PieceTypes::Bishop)
                .generate_bishop_moves(&mut moves_struct);
        })
    });
    moves_struct.clear();
    c.bench_function("rook", |b| {
        b.iter(|| {
            board
                .get_piece(&Color::Black, PieceTypes::Rook)
                .generate_rook_moves(&mut moves_struct);
        })
    });
    moves_struct.clear();
    c.bench_function("queen", |b| {
        b.iter(|| {
            board
                .get_piece(&Color::White, PieceTypes::Queen)
                .generate_queen_moves(&mut moves_struct);
        })
    });
    moves_struct.clear();
    c.bench_function("king", |b| {
        b.iter(|| {
            board
                .get_piece(&Color::Black, PieceTypes::King)
                .generate_king_moves(&mut moves_struct);
        })
    });
    c.bench_function("calculate_moves white", |b| {
        b.iter(|| {
            let _ =
                other_board.generate_moves(&mut moves_list2, &mut pieces_list, None, &Color::White);
        })
    });
    c.bench_function("calculate_moves_black", |b| {
        b.iter(|| {
            let _ =
                other_board.generate_moves(&mut moves_list2, &mut pieces_list, None, &Color::Black);
        })
    });
    /*c.bench_function("one_move_into_the_future", |b| {
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
