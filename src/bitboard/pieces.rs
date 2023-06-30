pub trait Piece {
    fn generate_moves(&self) -> u64;
}
pub enum PieceTypes {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King
}

pub struct Pawn(pub u64);
pub struct Knight(pub u64);
pub struct Bishop(pub u64);
pub struct Rook(pub u64);
pub struct Queen(pub u64);
pub struct King(pub u64);

impl super::BitBoard for Pawn{}
impl super::BitBoard for Bishop{}
impl super::BitBoard for Knight{}
impl super::BitBoard for Rook{}
impl super::BitBoard for Queen{}
impl super::BitBoard for King{}
