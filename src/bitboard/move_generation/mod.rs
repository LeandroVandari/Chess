pub mod pieces;
use super::consts;

type PossiblePieceMoves = u64;

pub enum Move {
    Regular {
        piece_type: pieces::PieceTypes,
        start_square: std::num::NonZeroU64,
        end_square: std::num::NonZeroU64,
    },
    EnPassant {
        start_square: std::num::NonZeroU64,
        end_square: std::num::NonZeroU64,
    },
    Promotion {
        target_piece: pieces::PieceTypes,
        start_square: std::num::NonZeroU64,
        end_square: std::num::NonZeroU64,
    },
    CastleKingside,
    CastleQueenside,
}

pub struct Moves<'a> {
    color: &'a super::Color,
    own_side: u64,
    other_side: u64,
    pub(super) all_attacks: u64,
    pub offset: usize,

    pub(super) moves_list: &'a mut [Option<PossiblePieceMoves>; 16],
    pub(super) pieces_list: &'a mut [u64; 16],

    pieces_start: [Option<usize>; 6],

    pub(super) pawn_attacks: u64,
    en_passant_take: super::EnPassant,
    en_passant: [Option<super::EnPassantTaker>; 2],
    en_passant_offset: usize,

    castle_kingside: bool,
    castle_queenside: bool,
}

impl<'a> Moves<'a> {
    pub fn new(
        own_side: u64,
        other_side: u64,
        moves_list: &'a mut [Option<PossiblePieceMoves>; 16],
        pieces_list: &'a mut [u64; 16],
        en_passant_take: super::EnPassant,
        color: &'a super::Color,
    ) -> Self {
        Moves {
            color,
            own_side,
            other_side,
            all_attacks: 0,
            offset: 0,
            moves_list,
            pieces_list,

            pieces_start: [None; 6],

            pawn_attacks: 0,
            en_passant_take,
            en_passant: [None, None],
            en_passant_offset: 0,

            castle_kingside: false,
            castle_queenside: false,
        }
    }

    pub fn clear(
        &mut self,
        color: Option<&'a super::Color>,
        own_side: Option<u64>,
        other_side: Option<u64>,
        en_passant_take: super::EnPassant,
    ) {
        self.color = color.unwrap_or(self.color);
        self.own_side = own_side.unwrap_or(self.own_side);
        self.other_side = other_side.unwrap_or(self.other_side);
        self.en_passant_take = en_passant_take;

        self.offset = 0;
        self.moves_list[0] = None;
        self.pieces_list[0] = 0;

        self.pieces_start = [None; 6];

        self.pawn_attacks = 0;
        self.en_passant[0] = None;
        self.en_passant_offset = 0;
        self.castle_kingside = false;
        self.castle_queenside = false;
    }

    pub(super) fn generate_castling(&mut self, position: &super::Position) {
        let castling = position.castling;
        let all_pieces = position.sides[0] | position.sides[1];
        let (kingside, queenside, kingside_pieces, queenside_pieces) = match position.to_move {
            super::Color::Black => (
                (castling & (1 << 2)) != 0,
                castling & (1 << 3) != 0,
                consts::boards::castling::kingside::black::MUST_BE_FREE,
                consts::boards::castling::queenside::black::MUST_BE_FREE,
            ),
            super::Color::White => (
                (castling & 1) != 0,
                (castling & 0b10) != 0,
                consts::boards::castling::kingside::white::MUST_BE_FREE,
                consts::boards::castling::queenside::white::MUST_BE_FREE,
            ),
        };

        if kingside && (all_pieces & kingside_pieces == 0) {
            self.castle_kingside = true;
        }
        if queenside && (all_pieces & queenside_pieces == 0) {
            self.castle_queenside = true;
        }
    }

    #[allow(clippy::too_many_lines)]
    /// Creates a list of possible moves from the current position based on self.
    ///
    /// # Panics
    /// This might panic if the Moves struct is created incorrectly.
    pub fn to_list_of_moves(&self, moves_list: &mut [Option<Move>]) {
        let mut current_position_index = 0;
        if self.castle_kingside {
            moves_list[0] = Some(Move::CastleKingside);
            current_position_index = 1;
        }
        if self.castle_queenside {
            moves_list[current_position_index] = Some(Move::CastleQueenside);
            current_position_index += 1;
        }
        let mut pieces_offsets = self
            .pieces_start
            .iter()
            .enumerate()
            .filter_map(|(i, p)| p.as_ref().map(|piece| (i, *piece)))
            .peekable();

        let next_piece = pieces_offsets.peek();
        if next_piece.is_none() {
            return;
        } else if let Some((consts::pieces::PAWN, pawn_start)) = next_piece {
            let pawn_start = *pawn_start;
            pieces_offsets.next();
            if self.en_passant_offset != 0 {
                for i in 0..self.en_passant_offset {
                    moves_list[current_position_index] = Some(Move::EnPassant {
                        start_square: unsafe {std::num::NonZeroU64::new_unchecked(*self.en_passant[i]
                            .as_ref()
                            .expect("As en_passant_take is not None, this should be set"))}
                            ,
                        end_square: self
                            .en_passant_take
                            .expect("I've already checked that this is None")
                            ,
                    });
                    current_position_index += 1;
                }
            }

            for pawn in pawn_start..(pieces_offsets.peek().unwrap_or(&(0, self.offset)).1) {
                let start_square = self.pieces_list[pawn];
                if (start_square
                    & if self.color.is_white() {
                        consts::rank::SEVEN
                    } else {
                        consts::rank::TWO
                    })
                    == 0
                {
                    let start_square = unsafe{std::num::NonZeroU64::new_unchecked(start_square)};
                    let mut left_to_loop = *self.moves_list[pawn].as_ref().unwrap();
                    while left_to_loop != 0 {
                        let end_square = 1 << left_to_loop.trailing_zeros();
                        moves_list[current_position_index] = Some(Move::Regular {
                            piece_type: pieces::PieceTypes::Pawn,
                            start_square,
                            end_square: unsafe{std::num::NonZeroU64::new_unchecked(end_square)},
                        });
                        current_position_index += 1;
                        left_to_loop &= !end_square;
                    }
                } else {
                    let start_square =  unsafe{std::num::NonZeroU64::new_unchecked(start_square)};
                    let mut left_to_loop = *self.moves_list[pawn].as_ref().unwrap();
                    while left_to_loop != 0 {
                        let end_square = 1 << left_to_loop.trailing_zeros();
                        for piece_type in [
                            pieces::PieceTypes::Knight,
                            pieces::PieceTypes::Bishop,
                            pieces::PieceTypes::Rook,
                            pieces::PieceTypes::Queen,
                        ] {
                            moves_list[current_position_index] = Some(Move::Promotion {
                                target_piece: piece_type,
                                start_square,
                            end_square: unsafe{std::num::NonZeroU64::new_unchecked(end_square)},
                            });
                            current_position_index += 1;
                        }
                        left_to_loop &= !end_square;
                    }
                }
            }
        }
        while let Some((piece_type, piece_start)) = pieces_offsets.next() {
            for piece in piece_start..pieces_offsets.peek().unwrap_or(&(0, self.offset)).1 {
                let start_square = unsafe{std::num::NonZeroU64::new_unchecked(self.pieces_list[piece])};
                let mut left_to_loop = *self.moves_list[piece].as_ref().unwrap();
                while left_to_loop != 0 {
                    let end_square = 1 << left_to_loop.trailing_zeros();
                    moves_list[current_position_index] = Some(Move::Regular {
                        piece_type: piece_type.into(),
                        start_square,
                            end_square: unsafe{std::num::NonZeroU64::new_unchecked(end_square)},
                    });
                    current_position_index += 1;
                    left_to_loop &= !end_square;
                }
            }
        }
        moves_list[current_position_index] = None;
    }
}

impl std::fmt::Display for Move {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut s: String;
        let self_as_str = match self {
            Move::CastleKingside => "O-O",
            Move::CastleQueenside => "O-O-O",
            Move::EnPassant {
                start_square,
                end_square,
            }
            | Move::Regular {
                piece_type: _,
                start_square,
                end_square,
            } => {
                s = crate::convert::from::bitboard::to_algebraic_square(start_square.get());
                s.push_str(
                    crate::convert::from::bitboard::to_algebraic_square(end_square.get()).as_str(),
                );
                s.as_str()
            }
            Move::Promotion {
                target_piece,
                start_square,
                end_square,
            } => {
                s = crate::convert::from::bitboard::to_algebraic_square(start_square.get());
                s.push_str(
                    crate::convert::from::bitboard::to_algebraic_square(end_square.get()).as_str(),
                );
                s.push(match target_piece {
                    pieces::PieceTypes::Bishop => 'b',
                    pieces::PieceTypes::Knight => 'n',
                    pieces::PieceTypes::Queen => 'q',
                    pieces::PieceTypes::Rook => 'r',
                    _ => 'e',
                });
                s.as_str()
            }
        };
        write!(f, "{self_as_str}")
    }
}
