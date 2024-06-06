use chess::bitboard as bb;

/* use fnv::FnvHashSet;

use chess::multi_thread_eval; */

fn main() {
    /* let board = chess::Board::new(); //.make_move(from_square("c2") as usize, chess::Move::RegularMove(from_square("c3")), chess::Color::White).make_move(from_square("a7") as usize, chess::Move::RegularMove(from_square("a5")), chess::Color::Black).make_move(from_square("d1") as usize, chess::Move::RegularMove(from_square("a4")), chess::Color::White);
    println!("{board}");
    let mut positions = FnvHashSet::default();
    let depth = 5;
    multi_thread_eval(&board, depth, chess::Color::White, &mut positions); */

    static MAP: once_cell::sync::Lazy<chashmap::CHashMap<bb::Position, u32>> =
        once_cell::sync::Lazy::new(chashmap::CHashMap::new);

    let mut board =
        bb::Position::from_fen("rnbq1k1r/pp1Pbppp/2p5/8/2B5/8/PPP1NnPP/RNBQK2R w KQ - 1 8");

    #[cfg(feature="hashmap")]
    let mut map = ahash::AHashMap::new();
    println!("{board}\n");
    /* {
        bb::macros::make_variables_for_perft!(3, moves_list_list, moves_list, pieces_list);
        board.perft(
            &mut moves_list_list,
            &mut moves_list,
            &mut pieces_list,
            &mut map,
        );
    }
    println!("{}", board.halfmoves);
    board.make_move(&bb::move_generation::Move::Regular {
        piece_type: bb::pieces::PieceTypes::Pawn,
        start_square: unsafe {
            std::num::NonZeroU64::new_unchecked(
                chess::convert::from::algebraic_square::to_bitboard("a2"),
            )
        },
        end_square: unsafe {
            std::num::NonZeroU64::new_unchecked(
                chess::convert::from::algebraic_square::to_bitboard("a3"),
            )
        },
    });
    println!("{}", board.halfmoves); */
    bb::macros::make_variables_for_perft!(4, moves_list_list, moves_list, pieces_list);
    let total_positions = board.perft(
        &mut moves_list_list,
        &mut moves_list,
        &mut pieces_list,
        #[cfg(feature = "hashmap")]
        &mut map,
    );
    let start = std::time::Instant::now();

    let time = (std::time::Instant::now() - start).as_millis();
    println!("Amount of positions: {total_positions}\nTime elapsed: {time}ms");
}
