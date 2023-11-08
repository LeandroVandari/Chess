use chess::bitboard as bb;

/* use fnv::FnvHashSet;

use chess::multi_thread_eval; */

fn main() {
    /* let board = chess::Board::new(); //.make_move(from_square("c2") as usize, chess::Move::RegularMove(from_square("c3")), chess::Color::White).make_move(from_square("a7") as usize, chess::Move::RegularMove(from_square("a5")), chess::Color::Black).make_move(from_square("d1") as usize, chess::Move::RegularMove(from_square("a4")), chess::Color::White);
    println!("{board}");
    let mut positions = FnvHashSet::default();
    let depth = 5;
    multi_thread_eval(&board, depth, chess::Color::White, &mut positions); */

    const DEPTH: usize = 2;
    const OTHER_DEPTH: usize = DEPTH - 1;

    let board = bb::Position::from_fen(&chess::bitboard::Fen::new(
        "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1",
    ));

    /*board
    .make_move(&bb::Move::Promotion {
        target_piece: bb::pieces::PieceTypes::Bishop,
        start_square: chess::from_square("g2"),
        end_square: chess::from_square("h1"),
    })
        .make_move(&bb::Move::Regular {
            piece_type: bb::pieces::PieceTypes::Bishop,
            start_square: chess::from_square("a6"),
            end_square: chess::from_square("b5"),
        })
        .make_move(&bb::Move::Regular {
            piece_type: bb::pieces::PieceTypes::Pawn,
            start_square: chess::from_square("e6"),
            end_square: chess::from_square("d7"),
        }); *//*
        .make_move(&bb::Move::Regular {
            piece_type: bb::pieces::PieceTypes::Knight,
            start_square: chess::from_square("g1"),
            end_square: chess::from_square("e2"),
        })
        .make_move(&bb::Move::Regular {
            piece_type: bb::pieces::PieceTypes::King,
            start_square: chess::from_square("e8"),
            end_square: chess::from_square("e7"),});*/
    println!("{board}\n");
    //board.place_piece(&pieces::PieceTypes::Knight, &Color::White, &Mask::from_square(36));

    let start = std::time::Instant::now();
    let total_positions = board.multi_thread_perft::<OTHER_DEPTH>();
    let time = (std::time::Instant::now() - start).as_micros();
    println!("Amount of positions: {total_positions}\nTime elapsed: {time}");
}
