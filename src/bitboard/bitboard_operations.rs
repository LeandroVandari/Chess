#[must_use]
pub fn has_piece(board: u64, piece: u64) -> bool {
    board & piece != 0
}

pub fn delete_piece(board: &mut u64, piece: u64) {
    *board &= !piece;
}

pub fn add_piece(board: &mut u64, piece: u64) {
    *board |= piece;
}
