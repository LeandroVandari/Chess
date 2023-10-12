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
    const POSITION: Option<bb::Position> = None;
    const POSITIONS_LIST: [Option<bb::Position>; 219] = [POSITION; 219];

    const DEPTH: usize = 3;

    let mut moves_list: [Option<bb::PossiblePieceMoves>; 16] = [POSS_MOVE; 16];
    let mut pieces_list: [u64; 16] = [0; 16];
    let mut positions_list_list: [[Option<bb::Position>; 219]; DEPTH] = [POSITIONS_LIST; DEPTH];

    let board = bb::Position::new();
    println!("{board}\n");
    //board.place_piece(&pieces::PieceTypes::Knight, &Color::White, &Mask::from_square(36));

    let mut total_moves = 0;
    board.perft(
        &mut positions_list_list,
        &mut moves_list,
        &mut pieces_list,
        &mut total_moves,
    );
    println!("{total_moves}");
}
