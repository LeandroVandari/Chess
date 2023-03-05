fn main() {
    let board = chess::Board::example();
    let all_moves = board.generate_moves();
    println!("{all_moves:?}");

    println!("{board}");
}
