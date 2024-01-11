use crate::bitboard::consts;
use crate::bitboard::macros;

macros::implement_bitboard_functions!(Piece);

#[derive(Debug, PartialEq)]
pub enum PieceTypes {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}

macros::implement_from_for_corresponding_values!(
usize "Usize has many possible values, that one has no equivalent PieceType",
PieceTypes {
    {consts::pieces::PAWN => PieceTypes::Pawn,
    consts::pieces::KNIGHT => PieceTypes::Knight,
    consts::pieces::BISHOP => PieceTypes::Bishop,
    consts::pieces::ROOK => PieceTypes::Rook,
    consts::pieces::QUEEN => PieceTypes::Queen,
    consts::pieces::KING => PieceTypes::King
}});

#[derive(PartialEq, Eq, Debug, Clone)]
pub struct Piece(u64);

impl Piece {
    pub fn generate_piece_moves(
        &self,
        current_piece_type: &PieceTypes,
        moves_struct: &mut super::Moves,
    ) {
        match current_piece_type {
            PieceTypes::Pawn => {
                self.generate_pawn_moves(moves_struct);
            }
            PieceTypes::Knight => {
                self.generate_knight_moves(moves_struct);
            }
            PieceTypes::Bishop => {
                self.generate_bishop_moves(moves_struct);
            }
            PieceTypes::Rook => {
                self.generate_rook_moves(moves_struct);
            }
            PieceTypes::Queen => {
                self.generate_queen_moves(moves_struct);
            }
            PieceTypes::King => {
                self.generate_king_moves(moves_struct);
            }
        }
    }

    #[allow(clippy::if_not_else)]
    pub fn generate_pawn_moves(&self, moves_struct: &mut super::Moves) {
        let all_pieces = moves_struct.own_side | moves_struct.other_side;
        let is_white = crate::bitboard::Color::White == *moves_struct.color;

        let mut left_to_loop = self.0;
        let mut current_piece: u64;

        if left_to_loop != 0 {
            moves_struct.pieces_start[consts::pieces::PAWN] = Some(moves_struct.offset);
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
            let one_forward = one_square & !all_pieces;
            let forward = if one_forward != 0 {
                if is_white {
                    ((current_piece << 16 & consts::pawn_after_moving_two_forward::WHITE)
                        | one_forward)
                        & !all_pieces
                } else {
                    ((current_piece >> 16 & consts::pawn_after_moving_two_forward::BLACK)
                        | one_forward)
                        & !all_pieces
                }
            } else {
                0
            };

            // captures
            let capture_left = if is_white {
                (current_piece & !consts::file::A) << 7
            } else {
                (current_piece & !consts::file::H) >> 7
            };
            let capture_right = if is_white {
                (current_piece & !consts::file::H) << 9
            } else {
                (current_piece & !consts::file::A) >> 9
            };

            let possible_captures = capture_left | capture_right;
            let en_passant = possible_captures & {
                match moves_struct.en_passant_take {
                    Some(num) => num.get(),
                    None => 0,
                }
            };
            if en_passant != 0 {
                moves_struct.en_passant[moves_struct.en_passant_offset] =
                    Some(current_piece);
                moves_struct.en_passant_offset += 1;
            };
            let captures = possible_captures & moves_struct.other_side;

            let moves = captures | forward;

            moves_struct.pawn_attacks |= possible_captures;
            moves_struct.moves_list[moves_struct.offset] = Some(moves);
            moves_struct.pieces_list[moves_struct.offset] = current_piece;
            moves_struct.all_attacks |= captures;
            moves_struct.offset += 1;
            left_to_loop &= !current_piece;
        }
    }

    pub fn generate_knight_moves(&self, moves_struct: &mut super::Moves) {
        let piece = self.0;
        crate::bitboard::macros::jump_moves!(
            moves_struct,
            piece,
            consts::pieces::KNIGHT,
            [
                (
                    10,
                    consts::file::H | consts::file::H >> 1 | consts::rank::EIGHT, // 1 up 2 right
                    consts::file::A | consts::file::A << 1 | consts::rank::ONE    // 1 down 2 left
                ),
                (
                    6,
                    consts::file::A | consts::file::A << 1 | consts::rank::EIGHT, // 1 up 2 left
                    consts::file::H | consts::file::H >> 1 | consts::rank::ONE    // 1 down 2 right
                ),
                (
                    15,
                    consts::rank::EIGHT | consts::rank::EIGHT >> 8 | consts::file::A, // 2 up 1 left
                    consts::rank::ONE | consts::rank::ONE << 8 | consts::file::H // 2 down 1 right
                ),
                (
                    17,
                    consts::rank::EIGHT | consts::rank::EIGHT >> 8 | consts::file::H, // 2 up 1 right
                    consts::rank::ONE | consts::rank::ONE << 8 | consts::file::A // 2 down 1 left
                )
            ]
        );
    }

    pub fn generate_bishop_moves(&self, moves_struct: &mut super::Moves) {
        let piece = self.0;
        crate::bitboard::macros::move_in_line!(
            moves_struct,
            piece,
            consts::pieces::BISHOP,
            [
                (
                    7,
                    (consts::file::A | consts::rank::EIGHT),
                    (consts::file::H | consts::rank::ONE)
                ),
                (
                    9,
                    (consts::file::H | consts::rank::EIGHT),
                    (consts::file::A | consts::rank::ONE)
                )
            ]
        );
    }

    pub fn generate_rook_moves(&self, moves_struct: &mut super::Moves) {
        let piece = self.0;
        crate::bitboard::macros::move_in_line!(
            moves_struct,
            piece,
            consts::pieces::ROOK,
            [
                (1, consts::file::H, consts::file::A),
                (8, consts::rank::EIGHT, consts::rank::ONE)
            ]
        );
    }

    pub fn generate_queen_moves(&self, moves_struct: &mut super::Moves) {
        let piece = self.0;
        crate::bitboard::macros::move_in_line!(
            moves_struct,
            piece,
            consts::pieces::QUEEN,
            [
                (1, consts::file::H, consts::file::A),
                (8, consts::rank::EIGHT, consts::rank::ONE),
                (
                    7,
                    (consts::file::A | consts::rank::EIGHT),
                    (consts::file::H | consts::rank::ONE)
                ),
                (
                    9,
                    (consts::file::H | consts::rank::EIGHT),
                    (consts::file::A | consts::rank::ONE)
                )
            ]
        );
    }

    pub fn generate_king_moves(&self, moves_struct: &mut super::Moves) {
        let piece = self.0;
        let mut moves = 0;

        moves |= ((piece & !(consts::file::H)) << 1) | ((piece & !(consts::file::A)) >> 1); // Right and left
        moves |= ((piece & !(consts::rank::EIGHT)) << 8) | ((piece & !(consts::rank::ONE)) >> 8); // Up and Down
        moves |= ((piece & !(consts::file::A | consts::rank::EIGHT)) << 7)
            | ((piece & !(consts::file::H | consts::rank::ONE)) >> 7); // Left up and right down
        moves |= ((piece & !(consts::file::H | consts::rank::EIGHT)) << 9)
            | ((piece & !(consts::file::A | consts::rank::ONE)) >> 9); // Right up and left down

        moves &= !moves_struct.own_side;

        moves_struct.moves_list[moves_struct.offset] = Some(moves);
        moves_struct.pieces_list[moves_struct.offset] = piece;
        moves_struct.pieces_start[consts::pieces::KING] = Some(moves_struct.offset);
        moves_struct.offset += 1;
    }
}