crate::bitboard::implement_bitboard_trait!(Pawn, Knight, Bishop, Rook, Queen, King);

pub trait Piece: super::BitBoard {
    fn generate_moves(
        &self,
        moves_list: &mut [u64; 16],
        offset: &mut usize,
        own_side: u64,
        other_side: u64,
        own_color: Option<super::Color>,
        can_en_passant: Option<bool>
    );
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

impl Piece for Pawn {
    fn generate_moves(
            &self,
            moves_list: &mut [u64; 16],
            offset: &mut usize,
            own_side: u64,
            other_side: u64,
            own_color: Option<super::Color>,
            can_en_passant: Option<bool>
        )  {
        let mut left_to_loop = self.0;
        let mut current_piece: u64;

        while left_to_loop != 0 {}
    }
}

impl Piece for Bishop {
    fn generate_moves(
            &self,
            moves_list: &mut [u64; 16],
            offset: &mut usize,
            own_side: u64,
            other_side: u64,
            _own_color: Option<super::Color>,
            _can_en_passant: Option<bool>
        ) {
        
        let piece = self.0;
        crate::bitboard::macros::move_in_line!(
            moves_list,
            offset,
            piece,
            own_side,
            other_side,
            [
                (
                    7,
                    crate::bitboard::consts::LEFT_AND_UP,
                    crate::bitboard::consts::RIGHT_AND_DOWN
                ),
                (
                    9,
                    crate::bitboard::consts::RIGHT_AND_UP,
                    crate::bitboard::consts::LEFT_AND_DOWN
                )
            ]
        );
    }
}

impl Piece for Rook {
    fn generate_moves(
            &self,
            moves_list: &mut [u64; 16],
            offset: &mut usize,
            own_side: u64,
            other_side: u64,
            _own_color: Option<super::Color>,
            _can_en_passant: Option<bool>
        ) {
        
        let piece = self.0;
        crate::bitboard::macros::move_in_line!(
            moves_list,
            offset,
            piece,
            own_side,
            other_side,
            [
                (
                    1,
                    crate::bitboard::consts::H_FILE,
                    crate::bitboard::consts::A_FILE
                ),
                (
                    8,
                    crate::bitboard::consts::LAST_ROW,
                    crate::bitboard::consts::FIRST_ROW
                )
            ]
        );
    }
}

impl Piece for Queen {
    fn generate_moves(
            &self,
            moves_list: &mut [u64; 16],
            offset: &mut usize,
            own_side: u64,
            other_side: u64,
            _own_color: Option<super::Color>,
            _can_en_passant: Option<bool>
        ) {
        
        let piece = self.0;
        crate::bitboard::macros::move_in_line!(
            moves_list,
            offset,
            piece,
            own_side,
            other_side,
            [
                (
                    1,
                    crate::bitboard::consts::H_FILE,
                    crate::bitboard::consts::A_FILE
                ),
                (
                    8,
                    crate::bitboard::consts::LAST_ROW,
                    crate::bitboard::consts::FIRST_ROW
                ),
                (
                    7,
                    crate::bitboard::consts::LEFT_AND_UP,
                    crate::bitboard::consts::RIGHT_AND_DOWN
                ),
                (
                    9,
                    crate::bitboard::consts::RIGHT_AND_UP,
                    crate::bitboard::consts::LEFT_AND_DOWN
                )
            ]
        );
    }
}
