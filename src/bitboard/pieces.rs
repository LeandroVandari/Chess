crate::bitboard::implement_bitboard!(Pawn, Knight, Bishop, Rook, Queen, King);

pub trait Piece: super::BitBoard {
    fn generate_moves(&self, own_side: u64, other_side: u64) -> u64;
}
pub enum PieceTypes {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}

pub struct Pawn(pub u64);
pub struct Knight(pub u64);
pub struct Bishop(pub u64);
pub struct Rook(pub u64);
pub struct Queen(pub u64);
pub struct King(pub u64);

impl Piece for Rook {
    fn generate_moves(&self, own_side: u64, other_side: u64) -> u64 {
        let pc = self.0;
        crate::bitboard::macros::move_in_line!(piece bitboard: pc, own side: own_side, opponent side: other_side, directions and conditions: [(1, crate::bitboard::consts::H_FILE, crate::bitboard::consts::A_FILE), (8, crate::bitboard::consts::LAST_ROW, crate::bitboard::consts::FIRST_ROW)])
    }
}
