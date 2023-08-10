use chess::bitboard as bb;

/* use fnv::FnvHashSet;

use chess::multi_thread_eval; */

fn main() {
    /* let board = chess::Board::new(); //.make_move(from_square("c2") as usize, chess::Move::RegularMove(from_square("c3")), chess::Color::White).make_move(from_square("a7") as usize, chess::Move::RegularMove(from_square("a5")), chess::Color::Black).make_move(from_square("d1") as usize, chess::Move::RegularMove(from_square("a4")), chess::Color::White);
    println!("{board}");
    let mut positions = FnvHashSet::default();
    let depth = 5;
    multi_thread_eval(&board, depth, chess::Color::White, &mut positions); */
    const MOVE: bb::Move = bb::Move(0);

    let mut moves_list: [bb::Move; 16] = [MOVE; 16];
    let en_passant = bb::EnPassant(0);
    let color = bb::Color::White;

    let board = bb::Position::example();

    //board.place_piece(&pieces::PieceTypes::Knight, &Color::White, &Mask::from_square(36));

    board.generate_moves(&mut moves_list, &en_passant, &color);

    println!("{board}\n");
    for item in moves_list {
        println!("{item}\n\n");
    }
}
