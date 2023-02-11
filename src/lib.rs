use std::collections::HashMap;
use std::fmt;

#[derive(Debug)]
pub struct Board {
    board: [Option<Piece>; 64],
}

impl Board {
    pub fn empty() -> Self {
        Board { board: [None; 64] }
    }

    pub fn new() -> Self {
        Board::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR")
    }

    pub fn from_fen(fen: &str) -> Self {
        let mut board = Self::empty();
        let mut square = 0;
        for ch in fen.chars() {
            match ch {
                '1'..='8' => square += ch.to_digit(10).unwrap() as u8,
                'p' => {
                    let piece = Piece::new(PieceTypes::Pawn, Color::White);
                    board.add_piece(piece, square);
                    square += 1;
                }
                'n' => {
                    let piece = Piece::new(PieceTypes::Knight, Color::White);
                    board.add_piece(piece, square);
                    square += 1;
                }
                'b' => {
                    let piece = Piece::new(PieceTypes::Bishop, Color::White);
                    board.add_piece(piece, square);
                    square += 1;
                }
                'r' => {
                    let piece = Piece::new(PieceTypes::Rook, Color::White);
                    board.add_piece(piece, square);
                    square += 1;
                }
                'q' => {
                    let piece = Piece::new(PieceTypes::Queen, Color::White);
                    board.add_piece(piece, square);
                    square += 1;
                }
                'k' => {
                    let piece = Piece::new(PieceTypes::King, Color::White);
                    board.add_piece(piece, square);
                    square += 1;
                }

                'P' => {
                    let piece = Piece::new(PieceTypes::Pawn, Color::Black);
                    board.add_piece(piece, square);
                    square += 1;
                }
                'N' => {
                    let piece = Piece::new(PieceTypes::Knight, Color::Black);
                    board.add_piece(piece, square);
                    square += 1;
                }
                'B' => {
                    let piece = Piece::new(PieceTypes::Bishop, Color::Black);
                    board.add_piece(piece, square);
                    square += 1;
                }
                'R' => {
                    let piece = Piece::new(PieceTypes::Rook, Color::Black);
                    board.add_piece(piece, square);
                    square += 1;
                }
                'Q' => {
                    let piece = Piece::new(PieceTypes::Queen, Color::Black);
                    board.add_piece(piece, square);
                    square += 1;
                }
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

    pub fn add_piece(&mut self, piece: Piece, square: u8) {
        if square > self.board.len() as u8 - 1 {
            println!("Couldn't add piece {piece:?}: Square {square} not in board.");
            return;
        }
        self.board[square as usize] = Some(piece);
    }

    fn row(square: u8) -> u8 {
        square / 8
    }
    fn column(square: u8) -> u8 {
        square % 8
    }

    pub fn possible_movements(&self) -> HashMap<u8, Vec<u8>> {
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

    fn pawn_moves(&self, piece: Piece, square: u8) -> Vec<u8> {
        let mut moves = Vec::new();
        if piece.color == Color::White {
            let piece_in_square = Self::add_move(&mut moves, &self.board, square + 8, piece.color);
            if Self::row(square) == 1 && piece_in_square.is_yes() {
                let _ = Self::add_move(&mut moves, &self.board, square + 16, piece.color);
            }
            if let Some(other_piece) = self.board[(square + 9) as usize] {
                if other_piece.color != piece.color {
                    let _ = Self::add_move(&mut moves, &self.board, square + 9, piece.color);
                }
            }
            if let Some(other_piece) = self.board[(square + 7) as usize] {
                if other_piece.color != piece.color {
                    let _ = Self::add_move(&mut moves, &self.board, square + 7, piece.color);
                }
            }
        } else if piece.color == Color::Black {
            let piece_in_square = Self::add_move(&mut moves, &self.board, square - 8, piece.color);
            if Self::row(square) == 6 && piece_in_square.is_yes() {
                let _ = Self::add_move(&mut moves, &self.board, square - 16, piece.color);
            }
            if square >= 7 {
                if let Some(other_piece) = self.board[(square - 7) as usize] {
                    if other_piece.color != piece.color {
                        let _ = Self::add_move(&mut moves, &self.board, square - 7, piece.color);
                    }
                }
                if square >= 9 {
                    if let Some(other_piece) = self.board[(square - 9) as usize] {
                        if other_piece.color != piece.color {
                            let _ =
                                Self::add_move(&mut moves, &self.board, square - 9, piece.color);
                        }
                    }
                }
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

    fn knight_moves(&self, piece: Piece, square: u8) -> Vec<u8> {
        let mut moves = Vec::new();
        let row = Self::row(square);
        let column = Self::column(square);
        if column != 7 {
            if row < 6 {
                let _ = Self::add_move(&mut moves, &self.board, square + 17, piece.color);
            }
            if row > 1 {
                let _ = Self::add_move(&mut moves, &self.board, square - 17, piece.color);
            }
        }
        if column != 0 {
            if row < 6 {
                let _ = Self::add_move(&mut moves, &self.board, square + 15, piece.color);
            }
            if row > 1 {
                let _ = Self::add_move(&mut moves, &self.board, square - 15, piece.color);
            }
        }
        if row != 7 {
            if column > 1 {
                let _ = Self::add_move(&mut moves, &self.board, square + 6, piece.color);
            }
            if column < 6 {
                let _ = Self::add_move(&mut moves, &self.board, square + 10, piece.color);
            }
        }
        if row != 0 {
            if column > 1 {
                let _ = Self::add_move(&mut moves, &self.board, square - 10, piece.color);
            }
            if column < 6 {
                let _ = Self::add_move(&mut moves, &self.board, square - 6, piece.color);
            }
        }
        moves
    }

    fn bishop_moves(&self, piece: Piece, square: u8) -> Vec<u8> {
        let mut moves = Vec::new();
        let mut next_square = square + 9;
        let mut piece_in_square = CanAddMoveAgain::Yes(0);
        while Self::row(next_square) < 7
            && Self::column(next_square) != 0
            && piece_in_square.is_yes()
        {
            piece_in_square = Self::add_move(&mut moves, &self.board, next_square, piece.color);
            next_square += 9;
        }
        piece_in_square = CanAddMoveAgain::Yes(0);
        next_square = square + 7;
        while Self::row(next_square) < 7
            && Self::column(next_square) != 7
            && piece_in_square.is_yes()
        {
            piece_in_square = Self::add_move(&mut moves, &self.board, next_square, piece.color);

            next_square += 7;
        }

        if square >= 7 {
            piece_in_square = CanAddMoveAgain::Yes(0);
            next_square = square - 7;
            while Self::column(next_square) != 0 && piece_in_square.is_yes() {
                piece_in_square = Self::add_move(&mut moves, &self.board, next_square, piece.color);
                if next_square < 7 {
                    break;
                }
                next_square -= 7;
            }

            if square >= 9 {
                piece_in_square = CanAddMoveAgain::Yes(0);
                next_square = square - 9;
                while Self::column(next_square) != 7 && piece_in_square.is_yes() {
                    piece_in_square =
                        Self::add_move(&mut moves, &self.board, next_square, piece.color);
                    if next_square < 9 {
                        break;
                    }
                    next_square -= 9;
                }
            }
        }
        moves
    }

    fn rook_moves(&self, piece: Piece, square: u8) -> Vec<u8> {
        let mut moves = Vec::new();
        let mut next_square = square + 8;
        let mut piece_in_square = CanAddMoveAgain::Yes(0);

        while next_square < 64 && piece_in_square.is_yes() {
            piece_in_square = Self::add_move(&mut moves, &self.board, next_square, piece.color);
            if Self::row(next_square) == 7 {
                break;
            }
            next_square += 8;
        }

        piece_in_square = CanAddMoveAgain::Yes(0);
        if square >= 8 {
            next_square = square - 8;
            while piece_in_square.is_yes() {
                piece_in_square = Self::add_move(&mut moves, &self.board, next_square, piece.color);
                if Self::row(next_square) == 0 {
                    break;
                }
                next_square -= 8;
            }
        }

        piece_in_square = CanAddMoveAgain::Yes(0);
        next_square = square + 1;
        while next_square < 64 && Self::column(next_square) != 0 && piece_in_square.is_yes() {
            piece_in_square = Self::add_move(&mut moves, &self.board, next_square, piece.color);
            next_square += 1;
        }
        if square > 0 {
            piece_in_square = CanAddMoveAgain::Yes(0);
            next_square = square - 1;
            while Self::column(next_square) != 7 && piece_in_square.is_yes() {
                piece_in_square = Self::add_move(&mut moves, &self.board, next_square, piece.color);
                if next_square == 0 {
                    break;
                }
                next_square -= 1;
            }
        }
        moves
    }

    fn queen_moves(&self, piece: Piece, square: u8) -> Vec<u8> {
        let mut moves = Self::bishop_moves(self, piece, square);
        moves.extend(Self::rook_moves(self, piece, square));
        moves
    }

    fn king_moves(&self, piece: Piece, square: u8, other_moves: &HashMap<u8, Vec<u8>>) -> Vec<u8> {
        let mut moves = Vec::new();
        let _ = Self::get_adjacent_squares(square)
            .into_iter()
            .flatten()
            .filter(|square| !(Self::is_check(self, other_moves, piece, *square)))
            .filter_map(|valid_square| {
                if let CanAddMoveAgain::Yes(square) =
                    Self::add_move(&mut moves, &self.board, valid_square, piece.color)
                {
                    Some(square)
                } else {
                    None
                }
            })
            .collect::<Vec<u8>>();
        //TODO: implement other king moves, exclude pawn moves
        moves
    }

    fn is_check(&self, possible_moves: &HashMap<u8, Vec<u8>>, king: Piece, king_pos: u8) -> bool {
        possible_moves
            .iter()
            .filter(|values| self.board[*values.0 as usize].unwrap().color != king.color)
            .map(|values| {
                let mut moves = Vec::new();
                if let PieceTypes::Pawn = self.board[*values.0 as usize].unwrap().variant {
                    if let Checks::True(check) = Self::pawn_checks_king(
                        *values.0,
                        king_pos,
                        self.board[*values.0 as usize].unwrap().color,
                    ) {
                        moves.push(check)
                    }
                } else {
                    moves = values.1.clone();
                }
                (values.0, moves)
            })
            .any(|values| {
                for value in &values.1 {
                    if *value == king_pos {
                        return true;
                    }
                }
                false
            })
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
        moves: &mut Vec<u8>,
        board: &[Option<Piece>],
        square: u8,
        color: Color,
    ) -> CanAddMoveAgain {
        if let Some(piece) = board[square as usize] {
            if piece.color != color {
                moves.push(square);
            }
            CanAddMoveAgain::No
        } else {
            moves.push(square);
            CanAddMoveAgain::Yes(square)
        }
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

impl CanAddMoveAgain {
    fn is_yes(&self) -> bool {
        if let CanAddMoveAgain::Yes(_) = self {
            return true;
        } else {
            return false;
        }
    }
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
}
