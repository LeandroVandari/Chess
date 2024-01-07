use self::pieces::PieceTypes;

mod bitboard_functions;
use bitboard_functions::*;
mod tests;

/// Contains basic constants such as the game starting position, ranks and files etc.
pub mod consts;
/// Contains all macros, used for implementing traits etc.
pub mod macros;

/// Contains move generation, the [`Piece`](pieces::Piece) trait etc.
pub mod pieces;

pub type EnPassant = Option<u64>;
pub type Side = u64;

pub enum Move {
    Regular {
        piece_type: pieces::PieceTypes,
        start_square: u64,
        end_square: u64,
    },
    EnPassant {
        start_square: u64,
        end_square: u64,
    },
    Promotion {
        target_piece: pieces::PieceTypes,
        start_square: u64,
        end_square: u64,
    },
    CastleKingside,
    CastleQueenside,
}

/// Represents all possiple moves by a piece, in a bitboard.
#[derive(Debug)]
pub struct PossiblePieceMoves(pub u64);

pub struct Moves<'a> {
    color: &'a Color,
    own_side: u64,
    other_side: u64,
    all_attacks: u64,
    pub offset: usize,

    moves_list: &'a mut [Option<PossiblePieceMoves>; 16],
    pieces_list: &'a mut [u64; 16],

    pieces_start: [Option<usize>; 6],

    pawn_attacks: u64,
    en_passant_take: Option<u64>,
    en_passant: [Option<EnPassantTaker>; 2],
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
        en_passant_take: EnPassant,
        color: &'a Color,
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
        color: Option<&'a Color>,
        own_side: Option<u64>,
        other_side: Option<u64>,
        en_passant_take: EnPassant,
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

    fn generate_castling(&mut self, position: &Position) {
        let castling = position.castling;
        let all_pieces = position.sides[0] | position.sides[1];
        let (kingside, queenside, kingside_pieces, queenside_pieces) = match position.to_move {
            Color::Black => (
                (castling & (1 << 2)) != 0,
                castling & (1 << 3) != 0,
                consts::boards::castling::kingside::black::MUST_BE_FREE,
                consts::boards::castling::queenside::black::MUST_BE_FREE,
            ),
            Color::White => (
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
                        start_square: self.en_passant[i]
                            .as_ref()
                            .expect("As en_passant_take is not None, this should be set")
                            .0,

                        end_square: self
                            .en_passant_take
                            .expect("I've already checked that this is None"),
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
                    let mut left_to_loop = self.moves_list[pawn].as_ref().unwrap().0;
                    while left_to_loop != 0 {
                        let end_square = 1 << left_to_loop.trailing_zeros();
                        moves_list[current_position_index] = Some(Move::Regular {
                            piece_type: PieceTypes::Pawn,
                            start_square,
                            end_square,
                        });
                        current_position_index += 1;
                        left_to_loop &= !end_square;
                    }
                } else {
                    let mut left_to_loop = self.moves_list[pawn].as_ref().unwrap().0;
                    while left_to_loop != 0 {
                        let end_square = 1 << left_to_loop.trailing_zeros();
                        for piece_type in [
                            PieceTypes::Knight,
                            PieceTypes::Bishop,
                            PieceTypes::Rook,
                            PieceTypes::Queen,
                        ] {
                            moves_list[current_position_index] = Some(Move::Promotion {
                                target_piece: piece_type,
                                start_square,
                                end_square,
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
                let start_square = self.pieces_list[piece];
                let mut left_to_loop = self.moves_list[piece].as_ref().unwrap().0;
                while left_to_loop != 0 {
                    let end_square = 1 << left_to_loop.trailing_zeros();
                    moves_list[current_position_index] = Some(Move::Regular {
                        piece_type: piece_type.into(),
                        start_square,
                        end_square,
                    });
                    current_position_index += 1;
                    left_to_loop &= !end_square;
                }
            }
        }
        moves_list[current_position_index] = None;
    }
}

pub struct EnPassantTaker(pub u64);

macros::implement_bitboard_functions!(PossiblePieceMoves, EnPassantTaker);

pub struct Fen(&'static str);

impl Fen {
    /// This function takes a [char] in standard FEN notation and returns the corresponding piece type and color.
    ///
    /// # Examples
    /// ```
    /// use chess::bitboard::{Fen, pieces, Color};
    ///
    /// assert_eq!((pieces::PieceTypes::Pawn, Color::White), Fen::char_to_piece('P'));
    /// ```
    ///
    /// # Panics
    /// This function will panic if the provided [char] does not have a corresponding piece type and color in FEN notation.
    #[must_use]
    pub const fn char_to_piece(ch: char) -> (pieces::PieceTypes, Color) {
        let col: Color = if ch.is_ascii_lowercase() {
            Color::Black
        } else {
            Color::White
        };
        let tp = match ch.to_ascii_lowercase() {
            'p' => pieces::PieceTypes::Pawn,
            'n' => pieces::PieceTypes::Knight,
            'b' => pieces::PieceTypes::Bishop,
            'r' => pieces::PieceTypes::Rook,
            'q' => pieces::PieceTypes::Queen,
            'k' => pieces::PieceTypes::King,
            _ => panic!("Char is not a valid chess piece"),
        };

        (tp, col)
    }
    #[must_use]
    pub fn inner(&self) -> &'static str {
        self.0
    }

    #[must_use]
    pub const fn new(inner: &'static str) -> Self {
        Self(inner)
    }

    fn index_to_fen_index(square: u8) -> u8 {
        70 - square - 2 * ((63 - square) % 8)
    }
}

/// Deal with game order, piece side etc.
#[derive(PartialEq, Debug, Clone)]
pub enum Color {
    White,
    Black,
}

impl Color {
    #[must_use]
    pub fn reversed(&self) -> Self {
        if let Color::Black = self {
            Color::White
        } else {
            Color::Black
        }
    }

    #[must_use]
    pub fn is_white(&self) -> bool {
        matches!(self, Color::White)
    }
}

macros::implement_from_for_corresponding_values!(usize "Usize has many possible values, that one has no equivalent Color", Color {{consts::sides::BLACK => Color::Black,
    consts::sides::WHITE => Color::White}});

/// Contains all bitboards fundamental to a position.
#[derive(PartialEq, Debug, Clone)]
pub struct Position {
    pub(crate) sides: [Side; 2],

    pub(crate) pieces: [[pieces::Piece; 6]; 2],

    pub(crate) to_move: Color,
    pub(crate) en_passant: EnPassant,
    pub(crate) castling: u8,
    pub(crate) halfmoves: u8,
    pub(crate) fullmoves: u8,
}

impl Position {
    /// Returns a [Position] containing the starting position of chess.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            sides: [
                consts::boards::startpos::black::ALL,
                consts::boards::startpos::white::ALL,
            ],
            pieces: [
                [
                    pieces::Piece::new(consts::boards::startpos::black::PAWN),
                    pieces::Piece::new(consts::boards::startpos::black::KNIGHT),
                    pieces::Piece::new(consts::boards::startpos::black::BISHOP),
                    pieces::Piece::new(consts::boards::startpos::black::ROOK),
                    pieces::Piece::new(consts::boards::startpos::black::QUEEN),
                    pieces::Piece::new(consts::boards::startpos::black::KING),
                ],
                [
                    pieces::Piece::new(consts::boards::startpos::white::PAWN),
                    pieces::Piece::new(consts::boards::startpos::white::KNIGHT),
                    pieces::Piece::new(consts::boards::startpos::white::BISHOP),
                    pieces::Piece::new(consts::boards::startpos::white::ROOK),
                    pieces::Piece::new(consts::boards::startpos::white::QUEEN),
                    pieces::Piece::new(consts::boards::startpos::white::KING),
                ],
            ],

            to_move: Color::White,
            en_passant: None,
            castling: 0b1111,
            halfmoves: 0,
            fullmoves: 1,
        }
    }

    #[must_use]
    pub fn get_piece(&self, color: &Color, piece_type: pieces::PieceTypes) -> &pieces::Piece {
        let side = if let Color::Black = *color {
            consts::sides::BLACK
        } else {
            consts::sides::WHITE
        };
        &self.pieces[side][usize::from(piece_type)]
    }

    /// Returns an empty [Position] that can be worked upon.
    #[must_use]
    pub fn empty() -> Self {
        Self {
            sides: [0, 0],
            pieces: [
                [
                    pieces::Piece::new(0),
                    pieces::Piece::new(0),
                    pieces::Piece::new(0),
                    pieces::Piece::new(0),
                    pieces::Piece::new(0),
                    pieces::Piece::new(0),
                ],
                [
                    pieces::Piece::new(0),
                    pieces::Piece::new(0),
                    pieces::Piece::new(0),
                    pieces::Piece::new(0),
                    pieces::Piece::new(0),
                    pieces::Piece::new(0),
                ],
            ],
            to_move: Color::White,
            en_passant: None,
            castling: 0,
            halfmoves: 0,
            fullmoves: 0,
        }
    }

    /// Return a new [Position] from a string in the FEN format.
    ///
    /// # Examples
    /// ```
    /// use chess::bitboard::{Fen, Position};
    ///
    /// assert_eq!(Position::new(), Position::from_fen(&Fen::new("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1")))
    /// ```
    /// # Panics
    ///    This function will panic if the FEN string provided is not in the standard FEN format, which you can learn more about [here](https://en.wikipedia.org/wiki/Forsyth%E2%80%93Edwards_Notation).
    #[must_use]
    #[allow(clippy::cast_possible_truncation)]
    pub fn from_fen(fen: &Fen) -> Self {
        let mut pos = Self::empty();
        let mut square = 0;
        let mut fen_iter = fen.inner().split_ascii_whitespace();
        for ch in fen_iter.next().unwrap().chars() {
            if let '1'..='8' = ch {
                square += ch
                    .to_digit(10)
                    .expect("As ch is only 1 to 8, conversion to usize should not fail.")
                    as u8;
            } else if ch != '/' {
                let (pc_type, pc_color) = Fen::char_to_piece(ch);
                pos.add_piece(
                    pc_type,
                    pc_color,
                    crate::convert::from::square_index::to_bitboard(Fen::index_to_fen_index(
                        square,
                    )), // TODO: create utility function to remove FEN
                );
                square += 1;
            }
        }

        match fen_iter.next().unwrap() {
            "w" => pos.to_move = Color::White,
            "b" => pos.to_move = Color::Black,
            _ => panic!("Invalid fen notation: second field should be color to move"),
        };

        let castling = fen_iter.next().unwrap();
        if castling != "-" {
            let castling_chars = ['K', 'Q', 'k', 'q'];
            for (i, ch) in castling_chars.iter().enumerate() {
                if castling.find(*ch).is_some() {
                    pos.castling |= 1 << i;
                }
            }
        }

        let en_passant = fen_iter.next().unwrap();
        pos.en_passant = match en_passant {
            "-" => None,
            _ => Some(crate::convert::from::algebraic_square::to_bitboard(
                en_passant,
            )),
        };

        pos.halfmoves = fen_iter.next().unwrap().parse().unwrap();
        pos.fullmoves = fen_iter.next().unwrap().parse().unwrap();

        pos
    }

    #[must_use]
    pub fn example() -> Self {
        Self::from_fen(&Fen::new("8/1P1Q4/1krN4/8/4B3/8/8/7K w - - 0 1"))
    }
    /// Get a specific bitboard in the position. If both a [`Color`] and a [`PieceTypes`](pieces::PieceTypes) are passed, it will return the board of that specific piece. If only a [`Color`] is passed, it will return that color's board.
    ///
    /// # Examples
    /// ```
    /// use chess::bitboard::{Position, Color, consts};
    /// use chess::bitboard::pieces::PieceTypes;
    ///
    /// let position = Position::new();
    ///
    /// let white_knights = position.get_board(&Color::White, Some(PieceTypes::Knight));
    /// let black = position.get_board(&Color::Black, None);
    /// let white_king = position.get_board(&Color::White, Some(PieceTypes::King));
    ///
    /// assert_eq!(white_knights, consts::boards::startpos::white::KNIGHT, "Did not return the white knights");
    /// assert_eq!(black, consts::boards::startpos::black::ALL, "Black pieces position is wrong");
    /// assert_eq!(white_king, consts::boards::startpos::white::KING, "Did not return the white king");
    /// ```
    #[must_use]
    pub fn get_board(&self, color: &Color, piece_type: Option<pieces::PieceTypes>) -> u64 {
        let side = if let Color::Black = *color {
            consts::sides::BLACK
        } else {
            consts::sides::WHITE
        };
        match piece_type {
            None => self.sides[side],
            Some(ptype) => self.pieces[side][usize::from(ptype)].inner(),
        }
    }

    /// Gets a pieces' [`Color`] and type ([`PieceTypes`](pieces::PieceTypes)) given a [`u64`] that contains the piece location. If piece type or color are already known, they can be specified with the [`Some`] variant.
    /// If the piece can't be located, it will return [`None`].
    /// # Examples
    /// ```
    /// use chess::bitboard::{Position, Color};
    /// use chess::bitboard::pieces::PieceTypes;
    /// use chess::convert;
    ///
    /// let position = Position::new();
    ///
    /// let (color, piece_type) = position.locate_piece(None, None, convert::from::square_index::to_bitboard(4)).unwrap();
    ///
    /// assert_eq!(color, Color::White);
    /// assert_eq!(piece_type, PieceTypes::King);
    /// ```
    #[must_use]
    pub fn locate_piece(
        &self,
        piece_type: Option<pieces::PieceTypes>,
        color: Option<Color>,
        mask: u64,
    ) -> Option<(Color, pieces::PieceTypes)> {
        let col = match color {
            Some(c) => c,
            None => {
                if has_piece(self.sides[consts::sides::BLACK], mask) {
                    Color::Black
                } else if has_piece(self.sides[consts::sides::WHITE], mask) {
                    Color::White
                } else {
                    return None;
                }
            }
        };

        let pc = if let Some(p) = piece_type {
            p
        } else {
            let color_index: usize = (&col).into();

            self.pieces[color_index]
                .iter()
                .position(|pc| has_piece(pc.inner(), mask))?
                .into()
        };
        Some((col, pc))
    }

    /// Places a piece in the board, replacing any piece that is already there.
    /// # Examples
    /// ```
    /// use chess::bitboard::{Position, Color};
    /// use chess::bitboard::pieces::PieceTypes;
    /// use chess::convert;
    ///
    /// let mut position = Position::empty();
    ///
    /// position.place_piece(PieceTypes::Rook, Color::White, convert::from::square_index::to_bitboard(6));
    ///
    /// assert_eq!(position.get_board(&Color::White, None), 0b1000000u64);
    /// assert_eq!(position.get_board(&Color::White, Some(PieceTypes::Rook)), 0b1000000u64);
    /// assert_ne!(position.get_board(&Color::Black, None), 0b1000000u64);
    /// ```
    pub fn place_piece(&mut self, piece_type: pieces::PieceTypes, color: Color, mask: u64) {
        let piece_in_board = self.locate_piece(None, None, mask);
        match piece_in_board {
            None => self.add_piece(piece_type, color, mask),
            Some((col, ptype)) => {
                self.remove_piece(ptype, col, mask);
                self.add_piece(piece_type, color, mask);
            }
        }
    }

    /// Takes a piece out of the board, updating the [Position] state if needed.
    /// # Examples
    /// ```
    /// use chess::bitboard::{Position, Color, consts};
    /// use chess::bitboard::pieces::PieceTypes;
    /// use chess::convert;
    ///
    /// let mut position = Position::new();
    ///
    /// position.remove_piece(PieceTypes::Queen, Color::Black, convert::from::square_index::to_bitboard(59));
    ///
    /// assert_eq!(position.get_board(&Color::Black, Some(PieceTypes::Queen)), 0);
    /// assert_eq!(position.get_board(&Color::Black, None), consts::boards::startpos::black::ALL & !consts::boards::startpos::black::QUEEN);
    /// ```
    pub fn remove_piece(&mut self, piece_type: pieces::PieceTypes, color: Color, mask: u64) {
        let color_index: usize = color.into();
        let piece_index: usize = piece_type.into();

        delete_piece(&mut self.sides[color_index], mask);
        delete_piece(self.pieces[color_index][piece_index].inner_mut(), mask);
    }

    fn add_piece(&mut self, piece_type: pieces::PieceTypes, color: Color, mask: u64) {
        let color_index: usize = color.into();
        let piece_index: usize = piece_type.into();

        add_piece(&mut self.sides[color_index], mask);
        add_piece(self.pieces[color_index][piece_index].inner_mut(), mask);
    }

    /// Generates all possible moves for the given [Color] and returns a [Moves] struct, containing all possible moves.
    pub fn generate_moves<'b>(
        &self,
        moves_list: &'b mut [Option<PossiblePieceMoves>; 16],
        pieces_list: &'b mut [u64; 16],
        en_passant: EnPassant,
        color: &'b Color,
    ) -> Moves<'b> {
        let side = usize::from(color);
        let mut moves = Moves::<'b>::new(
            self.sides[side],
            self.sides[usize::from(side == 0)],
            moves_list,
            pieces_list,
            en_passant,
            color,
        );

        self.pieces[side]
            .iter()
            .enumerate()
            .for_each(|(index, piece)| {
                piece.generate_piece_moves(&index.into(), &mut moves);
            });

        moves.generate_castling(self);
        moves
    }

    #[must_use]
    pub fn new_with_move(&self, move_enum: &Move) -> Self {
        let mut new_board: Position = self.clone();
        new_board.make_move(move_enum);
        new_board
    }

    #[allow(clippy::too_many_lines, clippy::unreadable_literal)]
    pub fn make_move(&mut self, move_enum: &Move) -> &mut Self {
        // TODO: try different aproach, only deleting the pieces, without checking.
        let own_side_index: usize = usize::from(&self.to_move);
        let other_side_index: usize = usize::from(own_side_index == 0);

        match move_enum {
            Move::Regular {
                piece_type,
                start_square,
                end_square,
            } => {
                self.en_passant = None;
                if has_piece(self.sides[other_side_index], *end_square) {
                    delete_piece(&mut self.sides[other_side_index], *end_square);
                    for (i, piece) in self.pieces[other_side_index].iter().enumerate() {
                        if has_piece(piece.inner(), *end_square) {
                            delete_piece(self.pieces[other_side_index][i].inner_mut(), *end_square);
                            if let PieceTypes::Rook = i.into() {
                                match self.to_move.reversed() {
                                    Color::White => {
                                        if *end_square == 0b1 {
                                            self.castling &= !0b10;
                                        } else if *end_square == 0b10000000 {
                                            self.castling &= !0b1;
                                        }
                                    }
                                    Color::Black => {
                                        if *end_square == 0b1 << 56 {
                                            self.castling &= !0b1000;
                                        } else if *end_square == 0b10000000 << 56 {
                                            self.castling &= !0b100;
                                        }
                                    }
                                }
                            }
                            break;
                        }
                    }
                }

                if let PieceTypes::King = piece_type {
                    self.castling &= if self.to_move.is_white() {
                        0b1100u8
                    } else {
                        0b11u8
                    };
                } else if let PieceTypes::Pawn = piece_type {
                    if has_piece(*start_square, consts::rank::SEVEN | consts::rank::TWO)
                        && has_piece(
                            *end_square,
                            consts::pawn_after_moving_two_forward::BLACK
                                | consts::pawn_after_moving_two_forward::WHITE,
                        )
                    {
                        self.en_passant = Some(if self.to_move.is_white() {
                            start_square << 8
                        } else {
                            start_square >> 8
                        });
                    }
                } else if let PieceTypes::Rook = piece_type {
                    match self.to_move {
                        Color::White => match *start_square {
                            0b10000000 => self.castling &= !1,
                            0b1 => self.castling &= !0b10,
                            _ => (),
                        },
                        Color::Black => {
                            const STARTPOS_BLACK_ROOK_KINGSIDE: u64 = 0b10000000 << 56;
                            const STARTPOS_BLACK_ROOK_QUEENSIDE: u64 = 0b1 << 56;
                            match *start_square {
                                STARTPOS_BLACK_ROOK_KINGSIDE => self.castling &= !0b0100,
                                STARTPOS_BLACK_ROOK_QUEENSIDE => self.castling &= !0b1000,
                                _ => (),
                            }
                        }
                    }
                }

                let piece_index: usize = piece_type.into();
                add_piece(
                    self.pieces[own_side_index][piece_index].inner_mut(),
                    *end_square,
                );
                add_piece(&mut self.sides[own_side_index], *end_square);
                delete_piece(&mut self.sides[own_side_index], *start_square);
                delete_piece(
                    self.pieces[own_side_index][piece_index].inner_mut(),
                    *start_square,
                );
            }
            Move::EnPassant {
                start_square,
                end_square,
            } => {
                self.en_passant = None;
                let pawn_take = if self.to_move.is_white() {
                    end_square >> 8
                } else {
                    end_square << 8
                };
                delete_piece(&mut self.sides[other_side_index], pawn_take);
                delete_piece(
                    self.pieces[other_side_index][consts::pieces::PAWN].inner_mut(),
                    pawn_take,
                );

                add_piece(
                    self.pieces[own_side_index][consts::pieces::PAWN].inner_mut(),
                    *end_square,
                );
                add_piece(&mut self.sides[own_side_index], *end_square);
                delete_piece(&mut self.sides[own_side_index], *start_square);
                delete_piece(
                    self.pieces[own_side_index][consts::pieces::PAWN].inner_mut(),
                    *start_square,
                );
            }

            Move::Promotion {
                target_piece,
                start_square,
                end_square,
            } => {
                if has_piece(self.sides[other_side_index], *end_square) {
                    delete_piece(&mut self.sides[other_side_index], *end_square);
                    for (i, piece) in self.pieces[other_side_index].iter().enumerate() {
                        if has_piece(piece.inner(), *end_square) {
                            delete_piece(self.pieces[other_side_index][i].inner_mut(), *end_square);
                            if let PieceTypes::Rook = i.into() {
                                match self.to_move.reversed() {
                                    Color::White => {
                                        if *end_square == 0b1 {
                                            self.castling &= !0b10;
                                        } else if *end_square == 0b10000000 {
                                            self.castling &= !0b1;
                                        }
                                    }
                                    Color::Black => {
                                        if *end_square == 0b1 << 56 {
                                            self.castling &= !0b1000;
                                        } else if *end_square == 0b10000000 << 56 {
                                            self.castling &= !0b100;
                                        }
                                    }
                                }
                            }
                            break;
                        }
                    }
                }
                self.en_passant = None;
                add_piece(&mut self.sides[own_side_index], *end_square);
                add_piece(
                    self.pieces[own_side_index][usize::from(target_piece)].inner_mut(),
                    *end_square,
                );
                delete_piece(&mut self.sides[own_side_index], *start_square);
                delete_piece(
                    self.pieces[own_side_index][consts::pieces::PAWN].inner_mut(),
                    *start_square,
                );
            }
            Move::CastleKingside => {
                self.castling &= !if self.to_move.is_white() {
                    0b11
                } else {
                    0b11 << 2
                };
                add_piece(
                    &mut self.sides[own_side_index],
                    if self.to_move.is_white() {
                        consts::boards::castling::kingside::white::KING_AND_ROOK_POS
                    } else {
                        consts::boards::castling::kingside::black::KING_AND_ROOK_POS
                    },
                );
                add_piece(
                    self.pieces[own_side_index][consts::pieces::KING].inner_mut(),
                    if self.to_move.is_white() {
                        0b01000000
                    } else {
                        0b01000000 << 56
                    },
                );
                delete_piece(
                    self.pieces[own_side_index][consts::pieces::ROOK].inner_mut(),
                    if self.to_move.is_white() {
                        0b10000000u64
                    } else {
                        0b10000000u64 << 56
                    },
                );
                add_piece(
                    self.pieces[own_side_index][consts::pieces::ROOK].inner_mut(),
                    if self.to_move.is_white() {
                        0b00100000
                    } else {
                        0b00100000 << 56
                    },
                );

                self.en_passant = None;
                delete_piece(
                    &mut self.sides[own_side_index],
                    if self.to_move.is_white() {
                        consts::boards::startpos::white::KING | 0b10000000u64
                    } else {
                        consts::boards::startpos::black::KING | (0b10000000u64 << 56)
                    },
                );
                delete_piece(
                    self.pieces[own_side_index][consts::pieces::KING].inner_mut(),
                    if self.to_move.is_white() {
                        consts::boards::startpos::white::KING
                    } else {
                        consts::boards::startpos::black::KING
                    },
                );
            }
            Move::CastleQueenside => {
                self.castling &= !if self.to_move.is_white() {
                    0b11
                } else {
                    0b11 << 2
                };
                add_piece(
                    &mut self.sides[own_side_index],
                    if self.to_move.is_white() {
                        consts::boards::castling::queenside::white::KING_AND_ROOK_POS
                    } else {
                        consts::boards::castling::queenside::black::KING_AND_ROOK_POS
                    },
                );
                delete_piece(
                    self.pieces[own_side_index][consts::pieces::ROOK].inner_mut(),
                    if self.to_move.is_white() {
                        0b1u64
                    } else {
                        0b1u64 << 56
                    },
                );
                add_piece(
                    self.pieces[own_side_index][consts::pieces::ROOK].inner_mut(),
                    if self.to_move.is_white() {
                        0b00001000
                    } else {
                        0b00001000 << 56
                    },
                );
                add_piece(
                    self.pieces[own_side_index][consts::pieces::KING].inner_mut(),
                    if self.to_move.is_white() {
                        0b00000100
                    } else {
                        0b00000100 << 56
                    },
                );

                self.en_passant = None;
                delete_piece(
                    &mut self.sides[own_side_index],
                    if self.to_move.is_white() {
                        consts::boards::startpos::white::KING | 0b1u64
                    } else {
                        consts::boards::startpos::black::KING | (0b1u64 << 56)
                    },
                );
                delete_piece(
                    self.pieces[own_side_index][consts::pieces::KING].inner_mut(),
                    if self.to_move.is_white() {
                        consts::boards::startpos::white::KING
                    } else {
                        consts::boards::startpos::black::KING
                    },
                );
            }
        }

        self.halfmoves += 1;
        if let Color::Black = self.to_move {
            self.fullmoves += 1;
        }
        self.to_move = self.to_move.reversed();
        self
    }

    #[must_use]
    pub fn is_check(&self, attacks: u64, color: &Color) -> bool {
        has_piece(
            self.pieces[usize::from(color)][consts::pieces::KING].inner(),
            attacks,
        )
    }

    pub fn perft<const DEPTH: usize>(
        &self,
        moves_list_list: &mut [[Option<Move>; 219]; DEPTH],
        moves_list: &mut [Option<PossiblePieceMoves>; 16],
        pieces_list: &mut [u64; 16],
    ) -> u32 {
        if DEPTH == 0 {
            return 0;
        }
        let mut total_moves = 0;
        let ptr_positions_list_list = moves_list_list as *mut [[Option<Move>; 219]; DEPTH];

        let current_list = &mut (*moves_list_list)[0];

        let moves_struct =
            self.generate_moves(moves_list, pieces_list, self.en_passant, &self.to_move);
        moves_struct.to_list_of_moves(current_list);
        let positions_iter =
            current_list
                .iter()
                .map_while(|pos| if let Some(p) = pos { Some(p) } else { None });
        for each_move in positions_iter {
            let mut branch_moves = 0;
            let new_pos = self.new_with_move(each_move);

            let new_pos_moves = new_pos.generate_moves(
                moves_list,
                pieces_list,
                new_pos.en_passant,
                &new_pos.to_move,
            );
            match each_move {
                Move::CastleKingside => {
                    if (new_pos_moves.all_attacks | new_pos_moves.pawn_attacks)
                        & if self.to_move.is_white() {
                            consts::boards::castling::kingside::white::KING_AND_ROOK_POS
                                | consts::boards::startpos::white::KING
                        } else {
                            consts::boards::castling::kingside::black::KING_AND_ROOK_POS
                                | consts::boards::startpos::black::KING
                        }
                        != 0
                    {
                        continue;
                    }
                }
                Move::CastleQueenside => {
                    if (new_pos_moves.all_attacks | new_pos_moves.pawn_attacks)
                        & if self.to_move.is_white() {
                            consts::boards::castling::queenside::white::KING_AND_ROOK_POS
                                | consts::boards::startpos::white::KING
                        } else {
                            consts::boards::castling::queenside::black::KING_AND_ROOK_POS
                                | consts::boards::startpos::black::KING
                        }
                        != 0
                    {
                        continue;
                    }
                }
                _ => (),
            }
            if !new_pos.is_check(new_pos_moves.all_attacks, &self.to_move) {
                if DEPTH == 1 {
                    total_moves += 1;
                    // #[cfg(debug_assertions)]
                    // println!("{each_move}: 1");
                    continue;
                }
                new_pos.perft_internal(
                    1,
                    ptr_positions_list_list,
                    &mut branch_moves,
                    new_pos_moves,
                );
                #[cfg(debug_assertions)]
                println!("{each_move}: {branch_moves}");
                total_moves += branch_moves;
            }
        }
        total_moves
    }

    #[allow(clippy::if_not_else, clippy::needless_pass_by_value)]
    fn perft_internal<const DEPTH: usize>(
        &self,
        curr_depth: usize,
        positions_list_list: *mut [[Option<Move>; 219]; DEPTH],
        total_moves: &mut u32,
        moves_struct: Moves,
    ) {
        let current_list = unsafe { &mut (*positions_list_list)[curr_depth] };

        moves_struct.to_list_of_moves(current_list);

        let positions_iter =
            current_list
                .iter()
                .map_while(|pos| if let Some(p) = pos { Some(p) } else { None });

        for each_move in positions_iter {
            let new_pos = self.new_with_move(each_move);
            let new_pos_moves = new_pos.generate_moves(
                moves_struct.moves_list,
                moves_struct.pieces_list,
                new_pos.en_passant,
                &new_pos.to_move,
            );
            match each_move {
                Move::CastleKingside => {
                    if (new_pos_moves.all_attacks | new_pos_moves.pawn_attacks)
                        & if self.to_move.is_white() {
                            consts::boards::castling::kingside::white::KING_AND_ROOK_POS
                                | consts::boards::startpos::white::KING
                        } else {
                            consts::boards::castling::kingside::black::KING_AND_ROOK_POS
                                | consts::boards::startpos::black::KING
                        }
                        != 0
                    {
                        continue;
                    }
                }
                Move::CastleQueenside => {
                    if (new_pos_moves.all_attacks | new_pos_moves.pawn_attacks)
                        & if self.to_move.is_white() {
                            consts::boards::castling::queenside::white::KING_AND_ROOK_POS
                                | consts::boards::startpos::white::KING
                        } else {
                            consts::boards::castling::queenside::black::KING_AND_ROOK_POS
                                | consts::boards::startpos::black::KING
                        }
                        != 0
                    {
                        continue;
                    }
                }
                _ => (),
            }
            if !new_pos.is_check(new_pos_moves.all_attacks, &self.to_move) {
                let new_depth = curr_depth + 1;
                if new_depth != DEPTH {
                    new_pos.perft_internal(
                        new_depth,
                        positions_list_list,
                        total_moves,
                        new_pos_moves,
                    );
                } else {
                    *total_moves += 1;
                }
            } else {
                // println!("{each_move}: Check!");
            }
        }
    }

    #[must_use]
    pub fn multi_thread_perft<const DEPTH: usize>(&self) -> u128 {
        const POSS_MOVE: Option<PossiblePieceMoves> = None;
        const POSITION: Option<Move> = None;
        const POSITIONS_LIST: [Option<Move>; 219] = [POSITION; 219];

        let (tx, rx) = std::sync::mpsc::channel();

        let mut moves_list: [Option<PossiblePieceMoves>; 16] = [POSS_MOVE; 16];
        let mut pieces_list: [u64; 16] = [0; 16];

        let moves_struct = self.generate_moves(
            &mut moves_list,
            &mut pieces_list,
            self.en_passant,
            &self.to_move,
        );
        let mut moves_list: [Option<Move>; 219] = POSITIONS_LIST;
        moves_struct.to_list_of_moves(&mut moves_list);
        let positions_iter =
            moves_list
                .iter()
                .map_while(|pos| if let Some(p) = pos { Some(p) } else { None });

        let total_moves = std::thread::scope(|s| {
            let mut handles = Vec::new();

            for each_move in positions_iter {
                let tx = tx.clone();

                let new_pos = self.new_with_move(each_move);
                let mut moves_list: [Option<PossiblePieceMoves>; 16] = [POSS_MOVE; 16];
                let mut pieces_list: [u64; 16] = [0; 16];
                let mut positions_list_list: [[Option<Move>; 219]; DEPTH] = [POSITIONS_LIST; DEPTH];
                let handle = s.spawn(move || {
                    tx.send((
                        u128::from(new_pos.perft(
                            &mut positions_list_list,
                            &mut moves_list,
                            &mut pieces_list,
                        )),
                        each_move,
                    ))
                    .unwrap();
                });
                handles.push(handle);
            }
            drop(tx);
            let counter_thread = s.spawn(move || {
                let mut total_moves = 0;
                #[allow(clippy::used_underscore_binding)]
                for (branch_moves, _each_move) in rx {
                    #[cfg(debug_assertions)]
                    println!("{_each_move}: {branch_moves}");
                    total_moves += branch_moves;
                }
                total_moves
            });
            for handle in handles {
                handle.join().unwrap();
            }
            counter_thread.join().unwrap()
        });
        total_moves
    }
}

impl Default for Position {
    fn default() -> Self {
        Position::new()
    }
}

impl std::fmt::Display for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut board = String::new();
        let piece_characters = [
            ['♟', '♞', '♝', '♜', '♛', '♚'],
            ['♙', '♘', '♗', '♖', '♕', '♔'],
        ];

        for i in 0..64 {
            if i % 8 == 0 && i != 0 {
                board.push('\n');
            }
            let mask = crate::convert::from::square_index::to_bitboard(i);

            let col_index = self.sides.iter().position(|b| has_piece(*b, mask));
            let piece_char = if let Some(index) = col_index {
                if let Some(i) = self.pieces[index]
                    .iter()
                    .position(|p| has_piece(p.inner(), mask))
                {
                    piece_characters[index][i]
                } else {
                    '.'
                }
            } else {
                '.'
            };

            board.push(piece_char);
            board.push(' ');
        }
        write!(f, "{}", board.as_str())
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
                s = crate::convert::from::bitboard::to_algebraic_square(*start_square);
                s.push_str(
                    crate::convert::from::bitboard::to_algebraic_square(*end_square).as_str(),
                );
                s.as_str()
            }
            Move::Promotion {
                target_piece,
                start_square,
                end_square,
            } => {
                s = crate::convert::from::bitboard::to_algebraic_square(*start_square);
                s.push_str(
                    crate::convert::from::bitboard::to_algebraic_square(*end_square).as_str(),
                );
                s.push(match target_piece {
                    PieceTypes::Bishop => 'b',
                    PieceTypes::Knight => 'n',
                    PieceTypes::Queen => 'q',
                    PieceTypes::Rook => 'r',
                    _ => 'e',
                });
                s.as_str()
            }
        };
        write!(f, "{self_as_str}")
    }
}
