use chess::bitboard::{pieces::PieceTypes, Color, Move, Moves, Position, PossiblePieceMoves};
use criterion::{criterion_group, criterion_main, Criterion};

pub fn criterion_benchmark(c: &mut Criterion) {

    const POSS_MOVE: Option<PossiblePieceMoves> = None;


    let board = Position::example();
    let other_board = Position::new();
    let mut moves_list: [Option<PossiblePieceMoves>; 16] = [POSS_MOVE; 16];
    let mut moves_list2: [Option<PossiblePieceMoves>; 16] = [POSS_MOVE; 16];
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

    c.bench_function("instantiate_board", |b| b.iter(Position::new));

    c.bench_function("pawn", |b| {
        b.iter(|| {
            board
                .get_piece(&Color::White, PieceTypes::Pawn)
                .generate_pawn_moves(&mut moves_struct);
            moves_struct.offset = 0;
        })
    });
    moves_struct.clear(None, None, None, None);
    c.bench_function("knight", |b| {
        b.iter(|| {
            board
                .get_piece(&Color::White, PieceTypes::Knight)
                .generate_knight_moves(&mut moves_struct);
            moves_struct.offset = 0;
        })
    });
    moves_struct.clear(None, None, None, None);
    c.bench_function("bishop", |b| {
        b.iter(|| {
            board
                .get_piece(&Color::White, PieceTypes::Bishop)
                .generate_bishop_moves(&mut moves_struct);
            moves_struct.offset = 0;
        })
    });
    moves_struct.clear(
        Some(&Color::Black),
        Some(board.get_board(&Color::Black, None)),
        Some(board.get_board(&Color::Black, None)),
        None,
    );
    c.bench_function("rook", |b| {
        b.iter(|| {
            board
                .get_piece(&Color::Black, PieceTypes::Rook)
                .generate_rook_moves(&mut moves_struct);
            moves_struct.offset = 0;
        })
    });
    moves_struct.clear(
        Some(&Color::White),
        Some(board.get_board(&Color::White, None)),
        Some(board.get_board(&Color::Black, None)),
        None,
    );
    c.bench_function("queen", |b| {
        b.iter(|| {
            board
                .get_piece(&Color::White, PieceTypes::Queen)
                .generate_queen_moves(&mut moves_struct);
            moves_struct.offset = 0;
        })
    });
    moves_struct.clear(
        Some(&Color::Black),
        Some(board.get_board(&Color::Black, None)),
        Some(board.get_board(&Color::Black, None)),
        None,
    );
    c.bench_function("king", |b| {
        b.iter(|| {
            board
                .get_piece(&Color::Black, PieceTypes::King)
                .generate_king_moves(&mut moves_struct);
            moves_struct.offset = 0;
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
    println!("\n\n\nSTARTPOS:");
    chess::bitboard::macros::benchmark_position!(c, "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1", "startpos", [1, 2, 3, 4, 5]);
    println!("\n\n\nPOSITION 2:");
    chess::bitboard::macros::benchmark_position!(c, "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1", "2", [1, 2, 3, 4]);
    println!("\n\n\nPOSITION 3:");
    chess::bitboard::macros::benchmark_position!(c, "8/2p5/3p4/KP5r/1R3p1k/8/4P1P1/8 w - - 0 1", "3", [1, 2, 3, 4, 5]);
    println!("\n\n\nPOSITION 4:");
    chess::bitboard::macros::benchmark_position!(c, "r3k2r/Pppp1ppp/1b3nbN/nP6/BBP1P3/q4N2/Pp1P2PP/R2Q1RK1 w kq - 0 1", "4", [1, 2, 3, 4, 5]);
    println!("\n\n\nPOSITION 5:");
    chess::bitboard::macros::benchmark_position!(c, "rnbq1k1r/pp1Pbppp/2p5/8/2B5/8/PPP1NnPP/RNBQK2R w KQ - 1 8", "5", [1, 2, 3, 4]);
    println!("\n\n\nPOSITION 6:");
    chess::bitboard::macros::benchmark_position!(c, "r4rk1/1pp1qppp/p1np1n2/2b1p1B1/2B1P1b1/P1NP1N2/1PP1QPPP/R4RK1 w - - 0 10", "6", [1, 2, 3, 4]);
    

    
    
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
