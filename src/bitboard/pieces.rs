use crate::bitboard::consts;
use crate::bitboard::macros;

macros::implement_bitboard_trait!(Pawn, Knight, Bishop, Rook, Queen, King);

pub trait Piece: super::BitBoard {
    fn generate_moves(
        &self,
        moves_list: &mut [super::Move; 16],
        offset: &mut usize,
        own_side: u64,
        other_side: u64,
        own_color: super::Color,
        can_en_passant: &super::EnPassant,
    );
}

#[derive(Debug, PartialEq)]
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
        moves_list: &mut [super::Move; 16],
        offset: &mut usize,
        own_side: u64,
        other_side: u64,
        own_color: super::Color,
        can_en_passant: &super::EnPassant,
    ) {
        let all_pieces = own_side | other_side;
        let is_white = super::Color::White == own_color;
        let move_two_start_row = if is_white {
            consts::PAWN_WHITE_AFTER_MOVE_TWO_FORWARD
        } else {
            consts::PAWN_BLACK_AFTER_MOVE_TWO_FORWARD
        };
        let mut left_to_loop = self.0;
        let mut current_piece: u64;
        let other_side_plus_en_passant = other_side | can_en_passant.0;

        // For each pawn
        while left_to_loop != 0 {
            current_piece = 1 << left_to_loop.trailing_zeros();
            //let mut moves = 0;
            // advance pawn
            let one_square = if is_white {
                current_piece << 8
            } else {
                current_piece >> 8
            };
            let two_squares = if is_white {
                current_piece << 16
            } else {
                current_piece >> 16
            };

            let forward = (one_square | (two_squares & move_two_start_row)) & !all_pieces;

            // captures
            let capture_left = if is_white {
                (current_piece & !consts::A_FILE) << 7
            } else {
                (current_piece & !consts::H_FILE) >> 7
            };
            let capture_right = if is_white {
                (current_piece & !consts::H_FILE) << 9
            } else {
                (current_piece & !consts::A_FILE) >> 9
            };

            let captures = (capture_left & other_side_plus_en_passant)
                | (capture_right & other_side_plus_en_passant);

            let moves = captures | forward;

            moves_list[*offset] = super::Move(moves);
            *offset += 1;
            left_to_loop &= !current_piece;
        }
    }
}

impl Piece for Knight {
    fn generate_moves(
        &self,
        moves_list: &mut [super::Move; 16],
        offset: &mut usize,
        own_side: u64,
        _other_side: u64,
        _own_color: super::Color,
        _can_en_passant: &super::EnPassant,
    ) {
        let piece = self.0;
        crate::bitboard::macros::jump_moves!(
            moves_list,
            offset,
            piece,
            own_side,
            [
                (
                    10,
                    consts::H_FILE | consts::H_FILE >> 1 | consts::RANK_EIGHT, // 1 up 2 right
                    consts::A_FILE | consts::A_FILE << 1 | consts::RANK_ONE  // 1 down 2 left
                ),
                (
                    6,
                    consts::A_FILE | consts::A_FILE << 1 | consts::RANK_EIGHT, // 1 up 2 left
                    consts::H_FILE | consts::H_FILE >> 1 | consts::RANK_ONE  // 1 down 2 right
                ),
                (
                    15,
                    consts::RANK_EIGHT | consts::RANK_EIGHT >> 8 | consts::A_FILE, // 2 up 1 left
                    consts::RANK_ONE | consts::RANK_ONE << 8 | consts::H_FILE  // 2 down 1 right
                ),
                (
                    17,
                    consts::RANK_EIGHT | consts::RANK_EIGHT >> 8 | consts::H_FILE, // 2 up 1 right
                    consts::RANK_ONE | consts::RANK_ONE << 8 | consts::A_FILE  // 2 down 1 left
                )
            ]
        );
    }
}

impl Piece for Bishop {
    fn generate_moves(
        &self,
        moves_list: &mut [super::Move; 16],
        offset: &mut usize,
        own_side: u64,
        other_side: u64,
        _own_color: super::Color,
        _can_en_passant: &super::EnPassant,
    ) {
        let piece = self.0;
        crate::bitboard::macros::move_in_line!(
            moves_list,
            offset,
            piece,
            own_side,
            other_side,
            [
                (7, consts::A_AND_8, consts::H_AND_1),
                (9, consts::H_AND_8, consts::A_AND_1)
            ]
        );
    }
}

impl Piece for Rook {
    fn generate_moves(
        &self,
        moves_list: &mut [super::Move; 16],
        offset: &mut usize,
        own_side: u64,
        other_side: u64,
        _own_color: super::Color,
        _can_en_passant: &super::EnPassant,
    ) {
        let piece = self.0;
        crate::bitboard::macros::move_in_line!(
            moves_list,
            offset,
            piece,
            own_side,
            other_side,
            [
                (1, consts::H_FILE, consts::A_FILE),
                (8, consts::RANK_EIGHT, consts::RANK_ONE)
            ]
        );
    }
}

impl Piece for Queen {
    fn generate_moves(
        &self,
        moves_list: &mut [super::Move; 16],
        offset: &mut usize,
        own_side: u64,
        other_side: u64,
        _own_color: super::Color,
        _can_en_passant: &super::EnPassant,
    ) {
        let piece = self.0;
        crate::bitboard::macros::move_in_line!(
            moves_list,
            offset,
            piece,
            own_side,
            other_side,
            [
                (1, consts::H_FILE, consts::A_FILE),
                (8, consts::RANK_EIGHT, consts::RANK_ONE),
                (7, consts::A_AND_8, consts::H_AND_1),
                (9, consts::H_AND_8, consts::A_AND_1)
            ]
        );
    }
}

impl Piece for King {
    fn generate_moves(
        &self,
        moves_list: &mut [super::Move; 16],
        offset: &mut usize,
        own_side: u64,
        _other_side: u64,
        _own_color: super::Color,
        _can_en_passant: &super::EnPassant,
    ) {
        let piece = self.0;
        crate::bitboard::macros::jump_moves!(
            moves_list,
            offset,
            piece,
            own_side,
            [
                (1, consts::H_FILE, consts::A_FILE),
                (8, consts::RANK_EIGHT, consts::RANK_ONE),
                (7, consts::A_AND_8, consts::H_AND_1),
                (9, consts::H_AND_8, consts::A_AND_1)
            ]
        );
    }
}
