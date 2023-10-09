use chess::bitboard as bb;

/* use fnv::FnvHashSet;

use chess::multi_thread_eval; */

fn main() {
    /* let board = chess::Board::new(); //.make_move(from_square("c2") as usize, chess::Move::RegularMove(from_square("c3")), chess::Color::White).make_move(from_square("a7") as usize, chess::Move::RegularMove(from_square("a5")), chess::Color::Black).make_move(from_square("d1") as usize, chess::Move::RegularMove(from_square("a4")), chess::Color::White);
    println!("{board}");
    let mut positions = FnvHashSet::default();
    let depth = 5;
    multi_thread_eval(&board, depth, chess::Color::White, &mut positions); */
    const POSS_MOVE: Option<bb::PossiblePieceMoves> = None;
    const POSITION: bb::Position = bb::Position::new();

    let mut moves_list: [Option<bb::PossiblePieceMoves>; 16] = [POSS_MOVE; 16];
    let mut pieces_list: [u64; 16] = [0; 16];
    let mut positions_list: [bb::Position; 27] = [POSITION;27];
    let color = bb::Color::White;

    let board = bb::Position::example();

    //board.place_piece(&pieces::PieceTypes::Knight, &Color::White, &Mask::from_square(36));

    let moves_struct = board.generate_moves(&mut moves_list, &mut pieces_list, None, &color);
    moves_struct.to_list_of_positions(&mut positions_list, &board);
  /*   for mov in moves_struct.pieces_list {
        println!("{}", bb::Mask::new(*mov));
    } */

    println!("{board}\n");
    for item in positions_list {
        println!("{item}\n\n");
    }
}
