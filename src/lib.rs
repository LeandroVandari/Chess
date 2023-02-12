use std::collections::HashMap;
use std::fmt;

#[derive(Debug)]
// Board struct, it's what the program is all about, contains a list of piece structs.
pub struct Board {
    board: [Option<Piece>; 64],
}

// Board methods
impl Board {
    // Return an empty board, that is, without any pieces in it.
    pub fn empty() -> Self {
        Board { board: [None; 64] }
    }

    // Return a board in the initial, default chess position. Could use from_fen, but this is ~2.5x faster.
    pub fn new() -> Self {
        let mut board = Self::empty();
        Self::add_piece(&mut board, Piece::new(PieceTypes::Rook, Color::White), 0);
        Self::add_piece(&mut board, Piece::new(PieceTypes::Knight, Color::White), 1);
        Self::add_piece(&mut board, Piece::new(PieceTypes::Bishop, Color::White), 2);
        Self::add_piece(&mut board, Piece::new(PieceTypes::Queen, Color::White), 3);
        Self::add_piece(&mut board, Piece::new(PieceTypes::King, Color::White), 4);
        Self::add_piece(&mut board, Piece::new(PieceTypes::Bishop, Color::White), 5);
        Self::add_piece(&mut board, Piece::new(PieceTypes::Knight, Color::White), 6);
        Self::add_piece(&mut board, Piece::new(PieceTypes::Rook, Color::White), 7);
        for i in 8..=15 {
            Self::add_piece(&mut board, Piece::new(PieceTypes::Pawn, Color::White), i);
        }

        Self::add_piece(&mut board, Piece::new(PieceTypes::Rook, Color::Black), 63);
        Self::add_piece(&mut board, Piece::new(PieceTypes::Knight, Color::Black), 62);
        Self::add_piece(&mut board, Piece::new(PieceTypes::Bishop, Color::Black), 61);
        Self::add_piece(&mut board, Piece::new(PieceTypes::King, Color::Black), 60);
        Self::add_piece(&mut board, Piece::new(PieceTypes::Queen, Color::Black), 59);
        Self::add_piece(&mut board, Piece::new(PieceTypes::Bishop, Color::Black), 58);
        Self::add_piece(&mut board, Piece::new(PieceTypes::Knight, Color::Black), 57);
        Self::add_piece(&mut board, Piece::new(PieceTypes::Rook, Color::Black), 56);
        for i in 48..=55 {
            Self::add_piece(&mut board, Piece::new(PieceTypes::Pawn, Color::Black), i);
        }

        board
    }

    // Get a FEN string and return a Board struct.
    pub fn from_fen(fen: &str) -> Self {
        // Create an empty Board to later mutate it.
        let mut board = Self::empty();
        // Index at which it will change the board (i. e. add the piece).
        let mut square = 0;

        // For each character in the string, convert it to a piece in the board, and add it.
        for ch in fen.chars() {
            match ch {
                // Empty squares
                '1'..='8' => square += ch.to_digit(10).unwrap() as u8,
                // White pawn
                'p' => {
                    let piece = Piece::new(PieceTypes::Pawn, Color::White);
                    board.add_piece(piece, square);
                    square += 1;
                }

                // White knight
                'n' => {
                    let piece = Piece::new(PieceTypes::Knight, Color::White);
                    board.add_piece(piece, square);
                    square += 1;
                }

                // White bishop
                'b' => {
                    let piece = Piece::new(PieceTypes::Bishop, Color::White);
                    board.add_piece(piece, square);
                    square += 1;
                }

                // White rook
                'r' => {
                    let piece = Piece::new(PieceTypes::Rook, Color::White);
                    board.add_piece(piece, square);
                    square += 1;
                }

                // White queen
                'q' => {
                    let piece = Piece::new(PieceTypes::Queen, Color::White);
                    board.add_piece(piece, square);
                    square += 1;
                }

                // White king
                'k' => {
                    let piece = Piece::new(PieceTypes::King, Color::White);
                    board.add_piece(piece, square);
                    square += 1;
                }

                // Black pawn
                'P' => {
                    let piece = Piece::new(PieceTypes::Pawn, Color::Black);
                    board.add_piece(piece, square);
                    square += 1;
                }

                // Black knight
                'N' => {
                    let piece = Piece::new(PieceTypes::Knight, Color::Black);
                    board.add_piece(piece, square);
                    square += 1;
                }

                // Black bishop
                'B' => {
                    let piece = Piece::new(PieceTypes::Bishop, Color::Black);
                    board.add_piece(piece, square);
                    square += 1;
                }

                // Black rook
                'R' => {
                    let piece = Piece::new(PieceTypes::Rook, Color::Black);
                    board.add_piece(piece, square);
                    square += 1;
                }

                // Black queen
                'Q' => {
                    let piece = Piece::new(PieceTypes::Queen, Color::Black);
                    board.add_piece(piece, square);
                    square += 1;
                }

                //Black king
                'K' => {
                    let piece = Piece::new(PieceTypes::King, Color::Black);
                    board.add_piece(piece, square);
                    square += 1;
                }

                _ => (),
            }
        }
        board
    }

    // Takes a mutable reference to self, a piece and a square, and adds the piece to the square in self.board
    pub fn add_piece(&mut self, piece: Piece, square: u8) {
        if square > self.board.len() as u8 - 1 {
            println!("Couldn't add piece {piece:?}: Square {square} not in board.");
            return;
        }
        self.board[square as usize] = Some(piece);
    }

    // Returns the row of a square
    fn row(square: u8) -> u8 {
        square / 8
    }

    // Returns the column of a square
    fn column(square: u8) -> u8 {
        square % 8
    }

    // Returns all possible moves from a given position
    pub fn possible_movements(&self) -> HashMap<u8, Moves> {
        const LIST_MAX_INDEX: u8 = 63;
        let mut movements = HashMap::new();
        for square in 0..=LIST_MAX_INDEX {
            if let Some(piece) = self.board[square as usize] {
                match piece {
                    Piece {
                        variant: PieceTypes::Pawn,
                        color: _,
                    } => {
                        let moves = Self::pawn_moves(self, piece, square);
                        movements.insert(square, moves);
                    }
                    Piece {
                        variant: PieceTypes::Knight,
                        color: _,
                    } => {
                        let moves = Self::knight_moves(self, piece, square);
                        movements.insert(square, moves);
                    }
                    Piece {
                        variant: PieceTypes::Bishop,
                        color: _,
                    } => {
                        let moves = Self::bishop_moves(self, piece, square);
                        movements.insert(square, moves);
                    }
                    Piece {
                        variant: PieceTypes::Rook,
                        color: _,
                    } => {
                        let moves = Self::rook_moves(self, piece, square);
                        movements.insert(square, moves);
                    }
                    Piece {
                        variant: PieceTypes::Queen,
                        color: _,
                    } => {
                        let moves = Self::queen_moves(self, piece, square);
                        movements.insert(square, moves);
                    }
                    Piece {
                        variant: PieceTypes::King,
                        color: _,
                    } => {
                        let moves = Self::king_moves(self, piece, square, &movements);
                        movements.insert(square, moves);
                    }
                }
            };
        }
        movements
    }

    fn pawn_moves(&self, piece: Piece, square: u8) -> Moves {
        let mut moves = Moves {
            can_move: Vec::new(),
            pieces_of_same_color: Vec::new(),
        };
        let is_white = piece.is_white();

        let can_add_move_again = Self::add_move(
            &mut moves,
            &self.board,
            if is_white { square + 8 } else { square - 8 },
            piece,
            false,
        );
        if Self::row(square) == (if is_white { 1 } else { 6 })
            && can_add_move_again.can_add_move_again()
        {
            Self::add_move(
                &mut moves,
                &self.board,
                if is_white { square + 16 } else { square - 16 },
                piece,
                false,
            );
        }
        if let Some(other_piece) =
            self.board[(if is_white { square + 9 } else { square - 9 }) as usize]
        {
            if other_piece.color != piece.color {
                Self::add_move(
                    &mut moves,
                    &self.board,
                    if is_white { square + 9 } else { square - 9 },
                    piece,
                    true,
                );
            }
        }
        if let Some(other_piece) =
            self.board[(if is_white { square + 7 } else { square - 7 }) as usize]
        {
            if other_piece.color != piece.color {
                Self::add_move(
                    &mut moves,
                    &self.board,
                    if is_white { square + 7 } else { square - 7 },
                    piece,
                    true,
                );
            }
        }

        moves
    }
    fn pawn_checks_king(square: u8, king: u8, self_color: Color) -> Checks {
        if let Color::Black = self_color {
            if square >= 7 {
                if square >= 9 && square - 9 == king {
                    return Checks::True(square - 9);
                }
                if square - 7 == king {
                    return Checks::True(square - 7);
                }
            }
        } else if square + 9 == king {
            return Checks::True(square + 9);
        } else if square + 7 == king {
            return Checks::True(square + 7);
        }
        Checks::False
    }

    fn knight_moves(&self, piece: Piece, square: u8) -> Moves {
        let mut moves = Moves {
            can_move: Vec::new(),
            pieces_of_same_color: Vec::new(),
        };
        let row = Self::row(square);
        let column = Self::column(square);
        if column != 7 {
            if row < 6 {
                let _ = Self::add_move(&mut moves, &self.board, square + 17, piece, false);
            }
            if row > 1 {
                let _ = Self::add_move(&mut moves, &self.board, square - 17, piece, false);
            }
        }
        if column != 0 {
            if row < 6 {
                let _ = Self::add_move(&mut moves, &self.board, square + 15, piece, false);
            }
            if row > 1 {
                let _ = Self::add_move(&mut moves, &self.board, square - 15, piece, false);
            }
        }
        if row != 7 {
            if column > 1 {
                let _ = Self::add_move(&mut moves, &self.board, square + 6, piece, false);
            }
            if column < 6 {
                let _ = Self::add_move(&mut moves, &self.board, square + 10, piece, false);
            }
        }
        if row != 0 {
            if column > 1 {
                let _ = Self::add_move(&mut moves, &self.board, square - 10, piece, false);
            }
            if column < 6 {
                let _ = Self::add_move(&mut moves, &self.board, square - 6, piece, false);
            }
        }
        moves
    }

    fn bishop_moves(&self, piece: Piece, square: u8) -> Moves {
        let mut moves = Moves {
            can_move: Vec::new(),
            pieces_of_same_color: Vec::new(),
        };
        let mut next_square = square + 9;
        let mut can_add_move_again = AddMoveResult::Yes(CanAddMoveAgain::Yes(0));
        while Self::row(next_square) < 7
            && Self::column(next_square) != 0
            && can_add_move_again.can_add_move_again()
        {
            can_add_move_again = Self::add_move(&mut moves, &self.board, next_square, piece, false);
            next_square += 9;
        }
        can_add_move_again = AddMoveResult::Yes(CanAddMoveAgain::Yes(0));
        next_square = square + 7;
        while Self::row(next_square) < 7
            && Self::column(next_square) != 7
            && can_add_move_again.can_add_move_again()
        {
            can_add_move_again = Self::add_move(&mut moves, &self.board, next_square, piece, false);

            next_square += 7;
        }

        if square >= 7 {
            can_add_move_again = AddMoveResult::Yes(CanAddMoveAgain::Yes(0));
            next_square = square - 7;
            while Self::column(next_square) != 0 && can_add_move_again.can_add_move_again() {
                can_add_move_again =
                    Self::add_move(&mut moves, &self.board, next_square, piece, false);
                if next_square < 7 {
                    break;
                }
                next_square -= 7;
            }

            if square >= 9 {
                can_add_move_again = AddMoveResult::Yes(CanAddMoveAgain::Yes(0));
                next_square = square - 9;
                while Self::column(next_square) != 7 && can_add_move_again.can_add_move_again() {
                    can_add_move_again =
                        Self::add_move(&mut moves, &self.board, next_square, piece, false);
                    if next_square < 9 {
                        break;
                    }
                    next_square -= 9;
                }
            }
        }
        moves
    }

    fn rook_moves(&self, piece: Piece, square: u8) -> Moves {
        let mut moves = Moves {
            can_move: Vec::new(),
            pieces_of_same_color: Vec::new(),
        };
        let mut next_square = square + 8;
        let mut can_add_move_again = AddMoveResult::Yes(CanAddMoveAgain::Yes(0));

        while next_square < 64 && can_add_move_again.can_add_move_again() {
            can_add_move_again = Self::add_move(&mut moves, &self.board, next_square, piece, false);
            if Self::row(next_square) == 7 {
                break;
            }
            next_square += 8;
        }

        can_add_move_again = AddMoveResult::Yes(CanAddMoveAgain::Yes(0));
        if square >= 8 {
            next_square = square - 8;
            while can_add_move_again.can_add_move_again() {
                can_add_move_again =
                    Self::add_move(&mut moves, &self.board, next_square, piece, false);
                if Self::row(next_square) == 0 {
                    break;
                }
                next_square -= 8;
            }
        }

        can_add_move_again = AddMoveResult::Yes(CanAddMoveAgain::Yes(0));
        next_square = square + 1;
        while next_square < 64
            && Self::column(next_square) != 0
            && can_add_move_again.can_add_move_again()
        {
            can_add_move_again = Self::add_move(&mut moves, &self.board, next_square, piece, false);
            next_square += 1;
        }
        if square > 0 {
            can_add_move_again = AddMoveResult::Yes(CanAddMoveAgain::Yes(0));
            next_square = square - 1;
            while Self::column(next_square) != 7 && can_add_move_again.can_add_move_again() {
                can_add_move_again =
                    Self::add_move(&mut moves, &self.board, next_square, piece, false);
                if next_square == 0 {
                    break;
                }
                next_square -= 1;
            }
        }
        moves
    }

    fn queen_moves(&self, piece: Piece, square: u8) -> Moves {
        let mut moves = Self::bishop_moves(self, piece, square);
        let rook = Self::rook_moves(self, piece, square);
        moves.can_move.extend(rook.can_move);
        moves.pieces_of_same_color.extend(rook.pieces_of_same_color);
        moves
    }

    fn king_moves(&self, piece: Piece, square: u8, other_moves: &HashMap<u8, Moves>) -> Moves {
        let mut moves = Moves {
            can_move: Vec::new(),
            pieces_of_same_color: Vec::new(),
        };
        let _ = Self::get_adjacent_squares(square)
            .into_iter()
            .flatten()
            .filter(|square| !(Self::is_check(self, other_moves, piece, *square)))
            .filter_map(|valid_square| {
                if let AddMoveResult::Yes(CanAddMoveAgain::Yes(square)) =
                    Self::add_move(&mut moves, &self.board, valid_square, piece, false)
                {
                    Some(square)
                } else {
                    None
                }
            })
            .collect::<Vec<u8>>();
        moves
    }


    fn is_check(&self, possible_moves: &HashMap<u8, Moves>, king: Piece, king_pos: u8) -> bool {
        println!("{possible_moves:?}");
        let answer = possible_moves
            .iter()
            .filter(|values| self.board[*values.0 as usize].unwrap().color != king.color)
            .map(|tuple| {
                match self.board[*tuple.0 as usize].unwrap().variant{
                    PieceTypes::Pawn => {
                        if let Checks::True(_) = Self::pawn_checks_king(
                            *tuple.0,
                            king_pos,
                            self.board[*tuple.0 as usize].unwrap().color,
                        ) { return true; }
                        else { return false;}
                    }
                    _ => {
                        let moves = tuple.1.can_move.iter().filter(|value| {println!("{value}, {king_pos}"); **value == king_pos}).next().is_some();
                        let moves2 = tuple.1.pieces_of_same_color.iter().filter(|value| {println!("{value}, {king_pos}"); **value == king_pos}).next().is_some();
                        moves || moves2
                    }
                } 
            }).any(|value| value == true);
        println!("{king_pos}, {answer}");
        answer
    }

    fn get_adjacent_squares(square: u8) -> [Option<u8>; 8] {
        let square_column = Self::column(square);
        let square_row = Self::row(square);
        let mut adjacent_squares: [Option<u8>; 8] = [None; 8];
        if square > 0 && Self::column(square - 1) == square_column - 1 {
            adjacent_squares[0] = Some(square - 1);
            if square_row != 7 {
                adjacent_squares[1] = Some(square + 7);
            }
            if square >= 9 {
                adjacent_squares[2] = Some(square - 9);
            }
        }
        if Self::column(square + 1) == square_column + 1 {
            adjacent_squares[3] = Some(square + 1);
            if square_row != 7 {
                adjacent_squares[4] = Some(square + 9);
            }
            if square >= 7 {
                adjacent_squares[5] = Some(square - 7);
            }
        }
        if square_row != 7 {
            adjacent_squares[6] = Some(square + 8);
        }
        if square_row != 0 {
            adjacent_squares[7] = Some(square - 8);
        }
        adjacent_squares
    }

    pub fn add_move(
        moves: &mut Moves,
        board: &[Option<Piece>],
        square: u8,
        piece: Piece,
        is_pawn_diagonal_move: bool,
    ) -> AddMoveResult {
        if square < 64 {
            if let Some(other_piece) = board[square as usize] {
                if let PieceTypes::Pawn = piece.variant {
                    if !(is_pawn_diagonal_move) {
                        return AddMoveResult::No;
                    }
                }
                if other_piece.color != piece.color {
                    moves.can_move.push(square);
                    return AddMoveResult::Yes(CanAddMoveAgain::No);
                } else {
                    moves.pieces_of_same_color.push(square);
                }
                return AddMoveResult::No;
            } else {
                moves.can_move.push(square);
                return AddMoveResult::Yes(CanAddMoveAgain::Yes(square));
            }
        }
        AddMoveResult::No
    }
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut board = String::new();
        for (square_counter, square) in self.board.into_iter().enumerate() {
            if let Some(piece) = square {
                match piece {
                    Piece {
                        variant: PieceTypes::Pawn,
                        color: Color::White,
                    } => board.push('♙'),
                    Piece {
                        variant: PieceTypes::Knight,
                        color: Color::White,
                    } => board.push('♘'),
                    Piece {
                        variant: PieceTypes::Bishop,
                        color: Color::White,
                    } => board.push('♗'),
                    Piece {
                        variant: PieceTypes::Rook,
                        color: Color::White,
                    } => board.push('♖'),
                    Piece {
                        variant: PieceTypes::Queen,
                        color: Color::White,
                    } => board.push('♕'),
                    Piece {
                        variant: PieceTypes::King,
                        color: Color::White,
                    } => board.push('♔'),

                    Piece {
                        variant: PieceTypes::Pawn,
                        color: Color::Black,
                    } => board.push('♟'),
                    Piece {
                        variant: PieceTypes::Knight,
                        color: Color::Black,
                    } => board.push('♞'),
                    Piece {
                        variant: PieceTypes::Bishop,
                        color: Color::Black,
                    } => board.push('♝'),
                    Piece {
                        variant: PieceTypes::Rook,
                        color: Color::Black,
                    } => board.push('♜'),
                    Piece {
                        variant: PieceTypes::Queen,
                        color: Color::Black,
                    } => board.push('♛'),
                    Piece {
                        variant: PieceTypes::King,
                        color: Color::Black,
                    } => board.push('♚'),
                }
            } else {
                board.push('.');
            }
            board.push(' ');
            if square_counter % 8 == 7 {
                board.push('\n');
            }
        }
        write!(f, "{}", board.as_str())
    }
}

impl Default for Board {
    fn default() -> Self {
        Board::new()
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PieceTypes {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}

enum Checks {
    True(u8),
    False,
}

pub enum CanAddMoveAgain {
    Yes(u8),
    No,
}

pub enum AddMoveResult {
    Yes(CanAddMoveAgain),
    No,
}

impl AddMoveResult {
    fn can_add_move_again(&self) -> bool {
        matches!(self, AddMoveResult::Yes(CanAddMoveAgain::Yes(_)))
    }

    fn added_move(&self) -> bool {
        matches!(self, AddMoveResult::Yes(_))
    }
}

#[derive(Debug)]
pub struct Moves {
    can_move: Vec<u8>,
    pieces_of_same_color: Vec<u8>,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Color {
    Black,
    White,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Piece {
    pub variant: PieceTypes,
    pub color: Color,
}

impl Piece {
    pub fn new(variant: PieceTypes, color: Color) -> Piece {
        Piece { variant, color }
    }

    fn is_white(&self) -> bool {
        self.color == Color::White
    }
}
