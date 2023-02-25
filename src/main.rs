fn main() {
    let board = chess::Board::example();
    board.generate_moves();

    println!("{board}");
}
