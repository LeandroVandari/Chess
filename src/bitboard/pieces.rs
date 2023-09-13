use crate::bitboard::consts;
use crate::bitboard::macros;

macros::implement_bitboard_trait!(Piece);

#[derive(Debug, PartialEq)]
pub enum PieceTypes {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}

macros::implement_from_for_corresponding_values!(usize "Usize has many possible values, that one has no equivalent PieceType", PieceTypes {{consts::PAWN => PieceTypes::Pawn,
    consts::KNIGHT => PieceTypes::Knight,
    consts::BISHOP => PieceTypes::Bishop,
    consts::ROOK => PieceTypes::Rook,
    consts::QUEEN => PieceTypes::Queen,
    consts::KING => PieceTypes::King}});

#[derive(PartialEq, Eq, Debug)]
pub struct Piece(u64);

impl Piece {
    pub fn generate_piece_moves(
        &self,
        current_piece_type: &PieceTypes,
        moves_list: &mut [super::Move; 16],
        offset: &mut usize,
        own_side: u64,
        other_side: u64,
        own_color: &super::Color,
        can_en_passant: &super::EnPassant,
        moves_struct: &mut super::Moves
    ) {
        match current_piece_type {
            PieceTypes::Pawn => {
                self.generate_pawn_moves(
                    offset,
                    moves_struct
                );
            }
            PieceTypes::Knight => {
                self.generate_knight_moves(moves_list, offset, own_side);
            }
            PieceTypes::Bishop => {
                self.generate_bishop_moves(moves_list, offset, own_side, other_side);
            }
            PieceTypes::Rook => {
                self.generate_rook_moves(moves_list, offset, own_side, other_side);
            }
            PieceTypes::Queen => {
                self.generate_queen_moves(moves_list, offset, own_side, other_side);
            }
            PieceTypes::King => {
                self.generate_king_moves(moves_list, offset, own_side);
            }
        }
    }

    pub fn generate_pawn_moves(
        &self,
        offset: &mut usize,
        moves_struct: &mut super::Moves
    ) {

        let all_pieces = moves_struct.own_side | moves_struct.other_side;
        let is_white = super::Color::White == *moves_struct.color;
        let move_two_start_row = if is_white {
            consts::PAWN_WHITE_AFTER_MOVE_TWO_FORWARD
        } else {
            consts::PAWN_BLACK_AFTER_MOVE_TWO_FORWARD
        };
        let mut left_to_loop = self.0;
        let mut current_piece: u64;
        let other_side_plus_en_passant = moves_struct.other_side | moves_struct.en_passant_take.unwrap_or(0);

        if left_to_loop != 0 {
            moves_struct.pawn_start = Some(*offset);
        }
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

            moves_struct.moves_list[*offset] = super::Move(moves);
            *offset += 1;
            left_to_loop &= !current_piece;
        }

    }

    pub fn generate_knight_moves(
        &self,
        moves_list: &mut [super::Move; 16],
        offset: &mut usize,
        own_side: u64,
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
                    consts::A_FILE | consts::A_FILE << 1 | consts::RANK_ONE    // 1 down 2 left
                ),
                (
                    6,
                    consts::A_FILE | consts::A_FILE << 1 | consts::RANK_EIGHT, // 1 up 2 left
                    consts::H_FILE | consts::H_FILE >> 1 | consts::RANK_ONE    // 1 down 2 right
                ),
                (
                    15,
                    consts::RANK_EIGHT | consts::RANK_EIGHT >> 8 | consts::A_FILE, // 2 up 1 left
                    consts::RANK_ONE | consts::RANK_ONE << 8 | consts::H_FILE      // 2 down 1 right
                ),
                (
                    17,
                    consts::RANK_EIGHT | consts::RANK_EIGHT >> 8 | consts::H_FILE, // 2 up 1 right
                    consts::RANK_ONE | consts::RANK_ONE << 8 | consts::A_FILE      // 2 down 1 left
                )
            ]
        );
    }

    pub fn generate_bishop_moves(
        &self,
        moves_list: &mut [super::Move; 16],
        offset: &mut usize,
        own_side: u64,
        other_side: u64,
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

    pub fn generate_rook_moves(
        &self,
        moves_list: &mut [super::Move; 16],
        offset: &mut usize,
        own_side: u64,
        other_side: u64,
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

    pub fn generate_queen_moves(
        &self,
        moves_list: &mut [super::Move; 16],
        offset: &mut usize,
        own_side: u64,
        other_side: u64,
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

    pub fn generate_king_moves(
        &self,
        moves_list: &mut [super::Move; 16],
        offset: &mut usize,
        own_side: u64,
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
