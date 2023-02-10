use std::collections::HashMap;
use std::fmt;

#[derive(Debug)]
struct Board {
    board: [Option<Piece>; 64],
}

impl Board {
    fn empty() -> Board {
        Board { board: [None; 64] }
    }

    fn new() -> Board {
        Board::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR")
    }

    fn from_fen(fen: &str) -> Board {
        let mut board = Self::empty();
        let mut square = 0;
        for ch in fen.chars() {
            match ch {
                '1'..='8' => square += ch.to_digit(10).unwrap() as usize,
                '/' => (),
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

    fn add_piece(&mut self, piece: Piece, square: usize) {
        if square > self.board.len() - 1 {
            println!("Couldn't add piece {piece:?}: Square {square} not in board.");
            return;
        }
        self.board[square] = Some(piece);
    }

    fn row(square: usize) -> usize {
        square / 8
    }
    fn column(square: usize) -> usize {
        square % 8
    }

    fn possible_movements(&self) -> HashMap<usize, Vec<usize>> {
        let mut movements = HashMap::new();
        for square in 0..=63 {
            if let Some(piece) = self.board[square] {
                let mut moves = Vec::new();
                match piece {
                    Piece {
                        variant: PieceTypes::Pawn,
                        color: _,
                    } => {
                        if piece.color == Color::White {
                            let piece_in_square =
                                Self::add_move(&mut moves, &self.board, square + 8, piece.color);
                            if Self::row(square) == 1 && piece_in_square.is_ok() {
                                let _ = Self::add_move(
                                    &mut moves,
                                    &self.board,
                                    square + 16,
                                    piece.color,
                                );
                            }
                        } else if piece.color == Color::Black {
                            let piece_in_square =
                                Self::add_move(&mut moves, &self.board, square - 8, piece.color);
                            if Self::row(square) == 6 && piece_in_square.is_ok() {
                                let _ = Self::add_move(
                                    &mut moves,
                                    &self.board,
                                    square - 16,
                                    piece.color,
                                );
                            }
                        }
                        movements.insert(square, moves);
                    }
                    Piece {
                        variant: PieceTypes::Knight,
                        color: _,
                    } => {
                        let row = Self::row(square);
                        let column = Self::column(square);
                        if column != 7 {
                            if row < 6 {
                                let _ = Self::add_move(
                                    &mut moves,
                                    &self.board,
                                    square + 17,
                                    piece.color,
                                );
                            }
                            if row > 1 {
                                let _ = Self::add_move(
                                    &mut moves,
                                    &self.board,
                                    square - 17,
                                    piece.color,
                                );
                            }
                        }
                        if column != 0 {
                            if row < 6 {
                                let _ = Self::add_move(
                                    &mut moves,
                                    &self.board,
                                    square + 15,
                                    piece.color,
                                );
                            }
                            if row > 1 {
                                let _ = Self::add_move(
                                    &mut moves,
                                    &self.board,
                                    square - 15,
                                    piece.color,
                                );
                            }
                        }
                        if row != 7 {
                            if column > 1 {
                                let _ = Self::add_move(
                                    &mut moves,
                                    &self.board,
                                    square + 6,
                                    piece.color,
                                );
                            }
                            if column < 6 {
                                let _ = Self::add_move(
                                    &mut moves,
                                    &self.board,
                                    square + 10,
                                    piece.color,
                                );
                            }
                        }
                        if row != 0 {
                            if column > 1 {
                                let _ = Self::add_move(
                                    &mut moves,
                                    &self.board,
                                    square - 10,
                                    piece.color,
                                );
                            }
                            if column < 6 {
                                let _ = Self::add_move(
                                    &mut moves,
                                    &self.board,
                                    square - 6,
                                    piece.color,
                                );
                            }
                        }
                        movements.insert(square, moves);
                    }
                    Piece {
                        variant: PieceTypes::Bishop,
                        color: _,
                    } => {
                        let mut next_square = square + 9;
                        let mut piece_in_square = Ok(());
                        while Self::row(next_square) < 7
                            && Self::column(next_square) != 0
                            && piece_in_square.is_ok()
                        {
                            piece_in_square =
                                Self::add_move(&mut moves, &self.board, next_square, piece.color);
                            next_square += 9;
                        }
                        piece_in_square = Ok(());
                        next_square = square + 7;
                        while Self::row(next_square) < 7
                            && Self::column(next_square) != 7
                            && piece_in_square.is_ok()
                        {
                            piece_in_square =
                                Self::add_move(&mut moves, &self.board, next_square, piece.color);

                            next_square += 7;
                        }

                        if square >= 7 {
                            piece_in_square = Ok(());
                            next_square = square - 7;
                            while Self::column(next_square) != 0 && piece_in_square.is_ok() {
                                piece_in_square = Self::add_move(
                                    &mut moves,
                                    &self.board,
                                    next_square,
                                    piece.color,
                                );
                                if next_square < 7 {
                                    break;
                                }
                                next_square -= 7;
                            }

                            if square >= 9 {
                                piece_in_square = Ok(());
                                next_square = square - 9;
                                while Self::column(next_square) != 7 && piece_in_square.is_ok() {
                                    piece_in_square = Self::add_move(
                                        &mut moves,
                                        &self.board,
                                        next_square,
                                        piece.color,
                                    );
                                    if next_square < 9 {
                                        break;
                                    }
                                    next_square -= 9;
                                }
                            }
                        }

                        movements.insert(square, moves);
                    }
                    Piece {
                        variant: PieceTypes::Rook,
                        color: _,
                    } => {
                        movements.insert(square, moves);
                    }
                    Piece {
                        variant: PieceTypes::Queen,
                        color: _,
                    } => {
                        movements.insert(square, moves);
                    }
                    Piece {
                        variant: PieceTypes::King,
                        color: _,
                    } => {
                        movements.insert(square, moves);
                    }
                }
            };
        }
        movements
    }

    fn add_move(
        moves: &mut Vec<usize>,
        board: &[Option<Piece>],
        square: usize,
        color: Color,
    ) -> Result<(), ()> {
        if let Some(piece) = board[square] {
            if piece.color != color {
                moves.push(square);
            }
            return Err(());
        }
        else {
            moves.push(square);
            return Ok(());
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

#[derive(Clone, Copy, Debug, PartialEq)]
enum PieceTypes {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum Color {
    Black,
    White,
}

#[derive(Clone, Copy, Debug, PartialEq)]
struct Piece {
    variant: PieceTypes,
    color: Color,
}

impl Piece {
    fn new(variant: PieceTypes, color: Color) -> Piece {
        Piece { variant, color }
    }
}

fn main() {
    let board = Board::new();
    let movements = board.possible_movements();
    println!("{board}");
    println!("{movements:?}");
    
    /* let mut board = Board::empty();
    board.add_piece(Piece { variant: PieceTypes::Knight, color: Color::White }, 3);
    board.add_piece(Piece { variant: PieceTypes::Pawn, color: Color::White }, 9);

    println!("{board}");
    let movements = board.possible_movements();
    println!("{movements:?}"); */
}
