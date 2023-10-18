use self::pieces::PieceTypes;

/// Contains basic constants such as the game starting position, ranks and files etc.
pub mod consts;
/// Contains all macros, used for implementing traits etc.
pub mod macros;

/// Contains move generation, the [`Piece`](pieces::Piece) trait etc.
pub mod pieces;

pub type EnPassant = Option<u64>;

pub struct Square(u8);

impl From<&str> for Square {
    #[allow(clippy::cast_possible_truncation)]
    fn from(value: &str) -> Self {
        assert_eq!(value.len(), 2);
        let mut value_iter = value.chars();
        let column = match value_iter.next().unwrap() {
            'a' => 0,
            'b' => 1,
            'c' => 2,
            'd' => 3,
            'e' => 4,
            'f' => 5,
            'g' => 6,
            'h' => 7,
            _ => panic!("Invalid column"),
        };
        let row = value_iter.next().unwrap().to_digit(10).unwrap();
        Square((8 * (row - 1) + column) as u8)
    }
}

/// Represent a side (white or black).
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct Side(u64);

pub enum Move {
    Regular {
        piece_type: pieces::PieceTypes,
        start_square: Mask,
        end_square: Mask,
    },
    EnPassant {
        start_square: Mask,
        end_square: Mask,
    },
    Promotion {
        target_piece: pieces::PieceTypes,
        start_square: Mask,
        end_square: Mask,
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

        self.en_passant[0] = None;
        self.en_passant_offset = 0;
        self.castle_kingside = false;
        self.castle_queenside = false;
    }

    fn generate_castling(&mut self, position: &Position) {
        let castling = position.castling;
        let all_pieces = position.sides[0].inner() | position.sides[1].inner();
        let (kingside, queenside, kingside_pieces, queenside_pieces) = match position.to_move {
            Color::Black => (
                (castling & (1 << 2)) != 0,
                castling & (1 << 3) != 0,
                consts::MUST_BE_FREE_CASTLE_KINGSIDE_BLACK,
                consts::MUST_BE_FREE_CASTLE_QUEENSIDE_BLACK,
            ),
            Color::White => (
                (castling & 1) != 0,
                (castling & 0b10) != 0,
                consts::MUST_BE_FREE_CASTLE_KINGSIDE_WHITE,
                consts::MUST_BE_FREE_CASTLE_QUEENSIDE_WHITE,
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
        } else if let Some((consts::PAWN, pawn_start)) = next_piece {
            let pawn_start = *pawn_start;
            pieces_offsets.next();
            if self.en_passant_offset != 0 {
                for i in 0..self.en_passant_offset {
                    moves_list[current_position_index] = Some(Move::EnPassant {
                        start_square: Mask(
                            self.en_passant[i]
                                .as_ref()
                                .expect("As en_passant_take is not None, this should be set")
                                .0,
                        ),
                        end_square: Mask(
                            self.en_passant_take
                                .expect("I've already checked that this is None"),
                        ),
                    });
                    current_position_index += 1;
                }
            }

            for pawn in pawn_start..(pieces_offsets.peek().unwrap_or(&(0, self.offset)).1) {
                let start_square = self.pieces_list[pawn];
                if (start_square
                    & if self.color.is_white() {
                        consts::RANK_SEVEN
                    } else {
                        consts::RANK_TWO
                    })
                    == 0
                {
                    let mut left_to_loop = self.moves_list[pawn].as_ref().unwrap().0;
                    while left_to_loop != 0 {
                        let end_square = 1 << left_to_loop.trailing_zeros();
                        moves_list[current_position_index] = Some(Move::Regular {
                            piece_type: PieceTypes::Pawn,
                            start_square: Mask(start_square),
                            end_square: Mask(end_square),
                        });
                        current_position_index += 1;
                        left_to_loop &= !end_square;
                    }
                } else {
                    let mut left_to_loop = self.moves_list[pawn].as_ref().unwrap().0;
                    while left_to_loop != 0 {
                        let end_square = Mask(1 << left_to_loop.trailing_zeros());
                        for piece_type in [
                            PieceTypes::Knight,
                            PieceTypes::Bishop,
                            PieceTypes::Rook,
                            PieceTypes::Queen,
                        ] {
                            moves_list[current_position_index] = Some(Move::Promotion {
                                target_piece: piece_type,
                                start_square: Mask(start_square),
                                end_square: end_square.clone(),
                            });
                            current_position_index += 1;
                        }
                        left_to_loop &= !end_square.clone().0;
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
                        start_square: Mask(start_square),
                        end_square: Mask(end_square),
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

macros::implement_bitboard_functions!(Side, PossiblePieceMoves, EnPassantTaker, Mask);

/// Newtype on a [u64] to do basic operations and pass in functions.
#[derive(Clone)]
pub struct Mask(u64);

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

macros::implement_from_for_corresponding_values!(usize "Usize has many possible values, that one has no equivalent Color", Color {{consts::BLACK => Color::Black,
    consts::WHITE => Color::White}});

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

impl Mask {
    /// Function to generate a [Mask] from a given square position in the form of an [u8].
    /// # Examples
    /// ```
    /// use chess::bitboard::Mask;
    /// let mask = Mask::from_square(5);
    /// assert_eq!(mask.inner(), 0b100000u64);
    /// ```
    #[must_use]
    pub const fn from_square(square: u8) -> Self {
        Mask(1 << square)
    }

    #[must_use]
    pub fn reversed(&self) -> Self {
        Self(!self.0)
    }
}

impl Position {
    /// Returns a [Position] containing the starting position of chess.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            sides: [Side(consts::STARTPOS_BLACK), Side(consts::STARTPOS_WHITE)],
            pieces: [
                [
                    pieces::Piece::new(consts::STARTPOS_BLACK_PAWNS),
                    pieces::Piece::new(consts::STARTPOS_BLACK_KNIGHTS),
                    pieces::Piece::new(consts::STARTPOS_BLACK_BISHOPS),
                    pieces::Piece::new(consts::STARTPOS_BLACK_ROOKS),
                    pieces::Piece::new(consts::STARTPOS_BLACK_QUEEN),
                    pieces::Piece::new(consts::STARTPOS_BLACK_KING),
                ],
                [
                    pieces::Piece::new(consts::STARTPOS_WHITE_PAWNS),
                    pieces::Piece::new(consts::STARTPOS_WHITE_KNIGHTS),
                    pieces::Piece::new(consts::STARTPOS_WHITE_BISHOPS),
                    pieces::Piece::new(consts::STARTPOS_WHITE_ROOKS),
                    pieces::Piece::new(consts::STARTPOS_WHITE_QUEEN),
                    pieces::Piece::new(consts::STARTPOS_WHITE_KING),
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
            consts::BLACK
        } else {
            consts::WHITE
        };
        &self.pieces[side][usize::from(piece_type)]
    }

    /// Returns an empty [Position] that can be worked upon.
    #[must_use]
    pub fn empty() -> Self {
        Self {
            sides: [Side(0), Side(0)],
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
                    &Mask::from_square(Fen::index_to_fen_index(square)),
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
            _ => Some(1 << <&str as Into<Square>>::into(en_passant).0),
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
    /// assert_eq!(white_knights, consts::STARTPOS_WHITE_KNIGHTS, "Did not return the white knights");
    /// assert_eq!(black, consts::STARTPOS_BLACK, "Black pieces position is wrong");
    /// assert_eq!(white_king, consts::STARTPOS_WHITE_KING, "Did not return the white king");
    /// ```
    #[must_use]
    pub fn get_board(&self, color: &Color, piece_type: Option<pieces::PieceTypes>) -> u64 {
        let side = if let Color::Black = *color {
            consts::BLACK
        } else {
            consts::WHITE
        };
        match piece_type {
            None => self.sides[side].0,
            Some(ptype) => self.pieces[side][usize::from(ptype)].inner(),
        }
    }

    /// Gets a pieces' [`Color`] and type ([`PieceTypes`](pieces::PieceTypes)) given a [`Mask`] that contains the piece location. If piece type or color are already known, they can be specified with the [`Some`] variant.
    /// If the piece can't be located, it will return [`None`].
    /// # Examples
    /// ```
    /// use chess::bitboard::{Position, Mask, Color};
    /// use chess::bitboard::pieces::PieceTypes;
    ///
    /// let position = Position::new();
    ///
    /// let (color, piece_type) = position.locate_piece(None, None, &Mask::from_square(4)).unwrap();
    ///
    /// assert_eq!(color, Color::White);
    /// assert_eq!(piece_type, PieceTypes::King);
    /// ```
    #[must_use]
    pub fn locate_piece(
        &self,
        piece_type: Option<pieces::PieceTypes>,
        color: Option<Color>,
        mask: &Mask,
    ) -> Option<(Color, pieces::PieceTypes)> {
        let col = match color {
            Some(c) => c,
            None => {
                if self.sides[consts::BLACK].has_piece(mask) {
                    Color::Black
                } else if self.sides[consts::WHITE].has_piece(mask) {
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
                .position(|pc| pc.has_piece(mask))?
                .into()
        };
        Some((col, pc))
    }

    /// Places a piece in the board, replacing any piece that is already there.
    /// # Examples
    /// ```
    /// use chess::bitboard::{Position, Color, Mask};
    /// use chess::bitboard::pieces::PieceTypes;
    ///
    /// let mut position = Position::empty();
    ///
    /// position.place_piece(PieceTypes::Rook, Color::White, &Mask::from_square(6));
    ///
    /// assert_eq!(position.get_board(&Color::White, None), 0b1000000u64);
    /// assert_eq!(position.get_board(&Color::White, Some(PieceTypes::Rook)), 0b1000000u64);
    /// assert_ne!(position.get_board(&Color::Black, None), 0b1000000u64);
    /// ```
    pub fn place_piece(&mut self, piece_type: pieces::PieceTypes, color: Color, mask: &Mask) {
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
    /// use chess::bitboard::{Position, Color, Mask, consts};
    /// use chess::bitboard::pieces::PieceTypes;
    ///
    /// let mut position = Position::new();
    ///
    /// position.remove_piece(PieceTypes::Queen, Color::Black, &Mask::from_square(59));
    ///
    /// assert_eq!(position.get_board(&Color::Black, Some(PieceTypes::Queen)), 0);
    /// assert_eq!(position.get_board(&Color::Black, None), consts::STARTPOS_BLACK & !consts::STARTPOS_BLACK_QUEEN);
    /// ```
    pub fn remove_piece(&mut self, piece_type: pieces::PieceTypes, color: Color, mask: &Mask) {
        let color_index: usize = color.into();
        let piece_index: usize = piece_type.into();

        self.sides[color_index].delete_piece(mask);
        self.pieces[color_index][piece_index].delete_piece(mask);
    }

    fn add_piece(&mut self, piece_type: pieces::PieceTypes, color: Color, mask: &Mask) {
        let color_index: usize = color.into();
        let piece_index: usize = piece_type.into();

        self.sides[color_index].add_piece(mask);
        self.pieces[color_index][piece_index].add_piece(mask);
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
            self.sides[side].0,
            self.sides[usize::from(side == 0)].0,
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

    #[allow(clippy::too_many_lines, clippy::unreadable_literal)]
    #[must_use]
    pub fn new_with_move(&self, move_enum: &Move) -> Self {
        // TODO: try different aproach, only deleting the pieces, without checking.
        let mut new_board: Position = self.clone();
        let own_side_index: usize = new_board.to_move.clone().into();
        let other_side_index: usize = usize::from(own_side_index == 0);

        match move_enum {
            Move::Regular {
                piece_type,
                start_square,
                end_square,
            } => {
                new_board.en_passant = None;
                if new_board.sides[other_side_index].has_piece(end_square) {
                    new_board.sides[other_side_index].delete_piece(end_square);
                    for (i, piece) in new_board.pieces[other_side_index].iter().enumerate() {
                        if piece.has_piece(end_square) {
                            new_board.pieces[other_side_index][i].delete_piece(end_square);
                            break;
                        }
                    }
                }

                if let PieceTypes::King = piece_type {
                    new_board.castling = 0;
                } else if let PieceTypes::Pawn = piece_type {
                    if start_square.has_piece(&Mask(consts::RANK_SEVEN | consts::RANK_TWO))
                        && end_square
                            .has_piece(&Mask(consts::RANK_SEVEN >> 16 | consts::RANK_TWO << 16))
                    {
                        new_board.en_passant = Some(if new_board.to_move.is_white() {
                            start_square.inner() << 8
                        } else {
                            start_square.inner() >> 8
                        });
                    }
                } else if let PieceTypes::Rook = piece_type {
                    match new_board.to_move {
                        Color::White => match start_square.0 {
                            0b10000000 => new_board.castling &= !1,
                            0b1 => new_board.castling &= !0b10,
                            _ => (),
                        },
                        Color::Black => {
                            const STARTPOS_BLACK_ROOK_KINGSIDE: u64 = 0b10000000 << 56;
                            const STARTPOS_BLACK_ROOK_QUEENSIDE: u64 = 0b1 << 56;
                            match start_square.0 {
                                STARTPOS_BLACK_ROOK_KINGSIDE => new_board.castling &= !(1 << 2),
                                STARTPOS_BLACK_ROOK_QUEENSIDE => new_board.castling &= !(1 << 3),
                                _ => (),
                            }
                        }
                    }
                }

                let piece_index: usize = piece_type.into();
                new_board.pieces[own_side_index][piece_index].add_piece(end_square);
                new_board.sides[own_side_index].add_piece(end_square);
                new_board.sides[own_side_index].delete_piece(start_square);
                new_board.pieces[own_side_index][piece_index].delete_piece(start_square);
            }
            Move::EnPassant {
                start_square,
                end_square,
            } => {
                new_board.en_passant = None;
                let pawn_take = &Mask(if new_board.to_move.is_white() {
                    &end_square.inner() >> 8
                } else {
                    &end_square.inner() << 8
                });
                new_board.sides[other_side_index].delete_piece(pawn_take);
                new_board.pieces[other_side_index][consts::PAWN].delete_piece(pawn_take);

                new_board.pieces[own_side_index][consts::PAWN].add_piece(end_square);
                new_board.sides[own_side_index].add_piece(end_square);
                new_board.sides[own_side_index].delete_piece(start_square);
                new_board.pieces[own_side_index][consts::PAWN].delete_piece(start_square);
            }

            Move::Promotion {
                target_piece,
                start_square,
                end_square,
            } => {
                if new_board.sides[other_side_index].has_piece(end_square) {
                    new_board.sides[other_side_index].delete_piece(end_square);
                    for (i, piece) in new_board.pieces[other_side_index].iter().enumerate() {
                        if piece.has_piece(end_square) {
                            new_board.pieces[other_side_index][i].delete_piece(end_square);
                            break;
                        }
                    }
                }
                new_board.en_passant = None;
                new_board.sides[own_side_index].add_piece(end_square);
                new_board.pieces[own_side_index][usize::from(target_piece)].add_piece(end_square);
                new_board.sides[own_side_index].delete_piece(start_square);
                new_board.pieces[own_side_index][consts::PAWN].delete_piece(start_square);
            }
            Move::CastleKingside => {
                new_board.castling = 0;
                new_board.sides[own_side_index].add_piece(&Mask(if new_board.to_move.is_white() {
                    consts::CASTLE_KINGSIDE_WHITE
                } else {
                    consts::CASTLE_KINGSIDE_BLACK
                }));
                new_board.pieces[own_side_index][consts::KING].add_piece(&Mask(
                    if new_board.to_move.is_white() {
                        0b01000000
                    } else {
                        0b01000000 << 56
                    },
                ));
                new_board.pieces[own_side_index][consts::ROOK].delete_piece(&Mask(
                    if new_board.to_move.is_white() {
                        0b10000000u64
                    } else {
                        0b10000000u64 << 56
                    },
                ));
                new_board.pieces[own_side_index][consts::ROOK].add_piece(&Mask(
                    if new_board.to_move.is_white() {
                        0b00100000
                    } else {
                        0b01000000 << 56
                    },
                ));

                new_board.en_passant = None;
                new_board.sides[own_side_index].delete_piece(if new_board.to_move.is_white() {
                    &Mask(consts::STARTPOS_WHITE_KING | 0b10000000u64)
                } else {
                    &Mask(consts::STARTPOS_BLACK_KING | (0b10000000u64 << 56))
                });
                new_board.pieces[own_side_index][consts::KING].delete_piece(
                    if new_board.to_move.is_white() {
                        &Mask(consts::STARTPOS_WHITE_KING)
                    } else {
                        &Mask(consts::STARTPOS_BLACK_KING)
                    },
                );
            }
            Move::CastleQueenside => {
                new_board.castling = 0;
                new_board.sides[own_side_index].add_piece(&Mask(if new_board.to_move.is_white() {
                    consts::CASTLE_QUEENSIDE_WHITE
                } else {
                    consts::CASTLE_QUEENSIDE_BLACK
                }));
                new_board.pieces[own_side_index][consts::ROOK].delete_piece(&Mask(
                    if new_board.to_move.is_white() {
                        0b1u64
                    } else {
                        0b1u64 << 56
                    },
                ));
                new_board.pieces[own_side_index][consts::ROOK].add_piece(&Mask(
                    if new_board.to_move.is_white() {
                        0b00000010
                    } else {
                        0b00000010 << 56
                    },
                ));
                new_board.pieces[own_side_index][consts::KING].add_piece(&Mask(
                    if new_board.to_move.is_white() {
                        0b00000100
                    } else {
                        0b00000100 << 56
                    },
                ));

                new_board.en_passant = None;
                new_board.sides[own_side_index].delete_piece(if new_board.to_move.is_white() {
                    &Mask(consts::STARTPOS_WHITE_KING | 0b1u64)
                } else {
                    &Mask(consts::STARTPOS_BLACK_KING | (0b1u64 << 56))
                });
                new_board.pieces[own_side_index][consts::KING].delete_piece(
                    if new_board.to_move.is_white() {
                        &Mask(consts::STARTPOS_WHITE_KING)
                    } else {
                        &Mask(consts::STARTPOS_BLACK_KING)
                    },
                );
            }
        }

        new_board.halfmoves += 1;
        if let Color::Black = new_board.to_move {
            new_board.fullmoves += 1;
        }
        new_board.to_move = new_board.to_move.reversed();
        new_board
    }

    #[must_use]
    pub fn is_check(&self, attacks: u64, color: &Color) -> bool {
        self.pieces[usize::from(color)][consts::KING].has_piece(&Mask(attacks))
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
            if DEPTH == 1 {
                total_moves += 1;
                #[cfg(debug_assertions)]
                println!("{each_move}: 1");
            } else {
                let mut branch_moves = 0;
                let new_pos = self.new_with_move(each_move);

                let new_pos_moves = new_pos.generate_moves(
                    moves_list,
                    pieces_list,
                    new_pos.en_passant,
                    &new_pos.to_move,
                );
                if !new_pos.is_check(new_pos_moves.all_attacks, &self.to_move) {
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
            let mask = &Mask(1 << i);

            let col_index = self.sides.iter().position(|b| b.has_piece(mask));
            let piece_char = if let Some(index) = col_index {
                if let Some(i) = self.pieces[index].iter().position(|p| p.has_piece(mask)) {
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

impl From<&Mask> for String {
    fn from(value: &Mask) -> Self {
        let mut final_str = String::new();
        let mut u8val = 0;
        for i in 0..64u8 {
            if 1 << i == value.0 {
                u8val = i;
            }
        }
        final_str.push(match u8val % 8 {
            0 => 'a',
            1 => 'b',
            2 => 'c',
            3 => 'd',
            4 => 'e',
            5 => 'f',
            6 => 'g',
            7 => 'h',
            _ => 'z',
        });

        final_str.push(char::from_digit(u32::from(u8val / 8) + 1, 10).unwrap());
        final_str
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
            | Move::Promotion {
                target_piece: _,
                start_square,
                end_square,
            }
            | Move::Regular {
                piece_type: _,
                start_square,
                end_square,
            } => {
                s = String::from(start_square);
                s.push_str(String::from(end_square).as_str());
                s.as_str()
            }
        };
        write!(f, "{self_as_str}")
    }
}

#[cfg(test)]
#[allow(clippy::unreadable_literal)]
mod tests {
    const STARTPOS: super::Position = super::Position::new();
    const POSS_MOVE: Option<super::PossiblePieceMoves> = None;
    const POSITION: Option<super::Move> = None;
    const POSITIONS_LIST: [Option<super::Move>; 219] = [POSITION; 219];

    #[test]
    fn perft_1() {
        const DEPTH: usize = 1;
        let mut moves_list: [Option<super::PossiblePieceMoves>; 16] = [POSS_MOVE; 16];
        let mut pieces_list: [u64; 16] = [0; 16];
        let mut positions_list_list: [[Option<super::Move>; 219]; DEPTH] = [POSITIONS_LIST; DEPTH];

        assert_eq!(
            STARTPOS.perft(&mut positions_list_list, &mut moves_list, &mut pieces_list),
            20
        );
    }

    #[test]
    fn perft_2() {
        const DEPTH: usize = 2;
        let mut moves_list: [Option<super::PossiblePieceMoves>; 16] = [POSS_MOVE; 16];
        let mut pieces_list: [u64; 16] = [0; 16];
        let mut positions_list_list: [[Option<super::Move>; 219]; DEPTH] = [POSITIONS_LIST; DEPTH];

        assert_eq!(
            STARTPOS.perft(&mut positions_list_list, &mut moves_list, &mut pieces_list),
            400
        );
    }

    #[test]
    fn perft_3() {
        const DEPTH: usize = 3;
        let mut moves_list: [Option<super::PossiblePieceMoves>; 16] = [POSS_MOVE; 16];
        let mut pieces_list: [u64; 16] = [0; 16];
        let mut positions_list_list: [[Option<super::Move>; 219]; DEPTH] = [POSITIONS_LIST; DEPTH];

        assert_eq!(
            STARTPOS.perft(&mut positions_list_list, &mut moves_list, &mut pieces_list),
            8902
        );
    }

    #[test]
    fn perft_4() {
        const DEPTH: usize = 4;
        let mut moves_list: [Option<super::PossiblePieceMoves>; 16] = [POSS_MOVE; 16];
        let mut pieces_list: [u64; 16] = [0; 16];
        let mut positions_list_list: [[Option<super::Move>; 219]; DEPTH] = [POSITIONS_LIST; DEPTH];

        assert_eq!(
            STARTPOS.perft(&mut positions_list_list, &mut moves_list, &mut pieces_list),
            197281
        );
    }

    #[test]
    fn perft_5() {
        const DEPTH: usize = 5;
        let mut moves_list: [Option<super::PossiblePieceMoves>; 16] = [POSS_MOVE; 16];
        let mut pieces_list: [u64; 16] = [0; 16];
        let mut positions_list_list: [[Option<super::Move>; 219]; DEPTH] = [POSITIONS_LIST; DEPTH];

        assert_eq!(
            STARTPOS.perft(&mut positions_list_list, &mut moves_list, &mut pieces_list),
            4865609
        );
    }

    #[test]
    fn perft_6() {
        const DEPTH: usize = 6;
        let mut moves_list: [Option<super::PossiblePieceMoves>; 16] = [POSS_MOVE; 16];
        let mut pieces_list: [u64; 16] = [0; 16];
        let mut positions_list_list: [[Option<super::Move>; 219]; DEPTH] = [POSITIONS_LIST; DEPTH];

        assert_eq!(
            STARTPOS.perft(&mut positions_list_list, &mut moves_list, &mut pieces_list),
            119060324
        );
    }

    #[test]
    fn perft_7() {
        const DEPTH: usize = 7;
        let mut moves_list: [Option<super::PossiblePieceMoves>; 16] = [POSS_MOVE; 16];
        let mut pieces_list: [u64; 16] = [0; 16];
        let mut positions_list_list: [[Option<super::Move>; 219]; DEPTH] = [POSITIONS_LIST; DEPTH];

        assert_eq!(
            STARTPOS.perft(&mut positions_list_list, &mut moves_list, &mut pieces_list),
            3195901860
        );
    }
}
