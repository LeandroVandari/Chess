use chess::{Board, Piece};
use criterion::{criterion_group, criterion_main, Criterion};

pub fn criterion_benchmark(c: &mut Criterion) {
    let board = Board::new();
    let other_board = Board::example();
    let movements = other_board.possible_movements();
    c.bench_function("instantiate_board", |b| b.iter(|| Board::new()));
    c.bench_function("pawn", |b| {
        b.iter(|| {
            other_board.pawn_moves(
                Piece {
                    variant: chess::PieceTypes::Pawn,
                    color: chess::Color::White,
                },
                9,
            )
        })
    });
    c.bench_function("knight", |b| {
        b.iter(|| {
            other_board.knight_moves(
                Piece {
                    variant: chess::PieceTypes::Knight,
                    color: chess::Color::White,
                },
                19,
            )
        })
    });
    c.bench_function("bishop", |b| {
        b.iter(|| {
            other_board.bishop_moves(
                Piece {
                    variant: chess::PieceTypes::Bishop,
                    color: chess::Color::White,
                },
                36,
            )
        })
    });
    c.bench_function("rook", |b| {
        b.iter(|| {
            other_board.rook_moves(
                Piece {
                    variant: chess::PieceTypes::Rook,
                    color: chess::Color::Black,
                },
                18,
            )
        })
    });
    c.bench_function("queen", |b| {
        b.iter(|| {
            other_board.queen_moves(
                Piece {
                    variant: chess::PieceTypes::Queen,
                    color: chess::Color::White,
                },
                11,
            )
        })
    });
    c.bench_function("king", |b| {
        b.iter(|| {
            other_board.king_moves(
                Piece {
                    variant: chess::PieceTypes::King,
                    color: chess::Color::Black,
                },
                17,
                &movements,
            )
        })
    });

    c.bench_function("calculate_moves", |b| b.iter(|| board.possible_movements()));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
