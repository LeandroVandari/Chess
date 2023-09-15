/// Contains basic constants such as the game starting position, ranks and files etc.
pub mod consts;
/// Contains all macros, used for implementing traits etc.
pub mod macros;

/// Contains move generation, the [`Piece`](pieces::Piece) trait etc.
pub mod pieces;


pub type EnPassant = Option<u64>;

/// The trait implemented by a struct containing a [u64], representing a bitboard. Should be implemented using the [`implement_bitboard_trait`](macros::implement_bitboard_trait) macro.
pub trait BitBoard {
    /// Check if the bitboard has a piece in a given position.
    fn has_piece(&self, mask: &Mask) -> bool;

    /// Add a piece at a given position.
    fn add_piece(&mut self, mask: &Mask);

    /// Remove a piece at a given position.
    fn delete_piece(&mut self, mask: &Mask);

    /// Return the inner [u64].
    fn inner(&self) -> u64;

    fn new(inner: u64) -> Self;
}

fn index_to_fen_index(square: u8) -> u8 {
    70 - square - 2 * ((63 - square) % 8)
}

/// Represent a side (white or black).
#[derive(PartialEq, Eq, Debug)]
pub struct Side(u64);

/// Represents all possiple moves by a piece, in a bitboard.
pub struct Move(pub u64);

pub struct Moves<'a> {
    color: &'a Color,
    own_side: u64,
    other_side: u64,
    offset: usize,

    moves_list: &'a mut [Move; 16],
    pieces_list: &'a mut [u64; 16],

    pawn_start: Option<usize>,
    knight_start: Option<usize>,
    bishop_start: Option<usize>,
    rook_start: Option<usize>,
    queen_start: Option<usize>,

    en_passant_take: Option<u64>,
    en_passant: [Option<EnPassantTaker>; 2],
    en_passant_offset: usize,
    castle_kingside: bool,
    castle_queenside: bool,
}

impl<'a> Moves<'a> {
    fn new(
        own_side: u64,
        other_side: u64,
        moves_list: &'a mut [Move; 16],
        pieces_list: &'a mut [u64; 16],
        en_passant_take: EnPassant,
        color: &'a Color,
    ) -> Self {
        Moves {
            color,
            own_side,
            other_side,
            offset: 0,
            moves_list,
            pieces_list,
            pawn_start: None,
            knight_start: None,
            bishop_start: None,
            rook_start: None,
            queen_start: None,
            en_passant_take,
            en_passant: [None, None],
            en_passant_offset: 0,
            castle_kingside: false,
            castle_queenside: false,
        }
    }
}

pub struct EnPassantTaker(pub u64);

macros::implement_bitboard_trait!(Side, Move, EnPassantTaker);

/// Newtype on a [u64] to do basic operations and pass in functions.
pub struct Mask(u64);

pub struct Fen(&'static str);

impl Fen {
    #[must_use]
    pub fn char_to_piece(ch: char) -> (pieces::PieceTypes, Color) {
        let col: Color = if ch.is_ascii_lowercase() {
            Color::Black
        } else {
            Color::White
        };
        println!("{ch}");
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
    pub fn new(inner: &'static str) -> Self {
        Self(inner)
    }
}

/// Deal with game order, piece side etc.
#[derive(PartialEq, Debug)]
pub enum Color {
    White,
    Black,
}

macros::implement_from_for_corresponding_values!(usize "Usize has many possible values, that one has no equivalent Color", Color {{consts::BLACK => Color::Black,
    consts::WHITE => Color::White}});

/// Contains all bitboards fundamental to a position.
#[derive(PartialEq, Eq, Debug)]
pub struct Position {
    pub(crate) sides: [Side; 2],

    pub(crate) pieces: [[pieces::Piece; 6]; 2],
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
    pub fn from_square(square: u8) -> Self {
        Mask(1 << square)
    }

    #[must_use]
    fn reverse(&self) -> Self {
        Self(!self.0)
    }

    #[must_use]
    pub fn inner(&self) -> u64 {
        self.0
    }
}

impl Position {
    /// Returns a [Position] containing the starting position of chess.
    #[must_use]
    pub fn new() -> Self {
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
        }
    }

    /// Return a new [Position] from a string in the FEN format.
    ///
    /// # Examples:
    /// ```
    /// use chess::bitboard::{Fen, Position};
    ///
    /// assert_eq!(Position::new(), Position::from_fen(Fen::new("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR")))
    /// ```
    ///
    #[must_use]
    #[allow(clippy::cast_possible_truncation)]
    pub fn from_fen(fen: &Fen) -> Self {
        let mut pos = Self::empty();
        let mut square = 0;

        for ch in fen.inner().chars() {
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
                    &Mask::from_square(index_to_fen_index(square)),
                );
                square += 1;
            }
        }

        pos
    }

    #[must_use]
    pub fn example() -> Self {
        Self::from_fen(&Fen::new("8/1P1Q4/1krN4/8/4B3/8/8/7K"))
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
    /// assert_eq!(white_knights, consts::STARTPOS_WHITE_KNIGHTS);
    /// assert_eq!(black, consts::STARTPOS_BLACK);
    /// assert_eq!(white_king, consts::STARTPOS_WHITE_KING);
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

    /// Puts all moves possible for the position for the given [Color] in the `moves_list` parameter.
    pub fn generate_moves<'b>(
        &self,
        moves_list: &'b mut [Move; 16],
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

        /*         self.pieces[side]
        .iter()
        .enumerate()
        .for_each(|(index, piece)| {
            piece.generate_piece_moves(
                &index.into(),
                moves_list,
                &mut offset,
                self.sides[side].0,
                self.sides[usize::from(side == 0)].0,
                color,
                en_passant,
            );
        }); */
        moves
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
