fn main() {
    let board = chess::Board::new();
    board.generate_moves();

    println!("{board}");
}
