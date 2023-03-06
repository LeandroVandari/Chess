use super::{Bishop, Color, King, Knight, Pawn, Piece, Queen, Rook};
use std::cell::Cell;
use std::collections::HashMap;
use std::fmt;

// The board. Is wrapped in a struct in order to implement methods.
pub struct Board {
    pub board: [Option<Piece>; 64],
    pub can_en_passant: Cell<CanEnPassant>,
    pub can_castle: Cell<CanCastle>,
}

#[derive(Clone, Copy)]
pub enum CanEnPassant {
    Yes(u8),
    No,
}

#[derive(Clone, Copy)]
pub struct CanCastle {
    pub white_kingside: bool,
    pub white_queenside: bool,
    pub black_kingside: bool,
    pub black_queenside: bool,
}
impl CanCastle {
    fn new() -> Self {
        CanCastle {
            white_kingside: true,
            white_queenside: true,
            black_kingside: true,
            black_queenside: true,
        }
    }
}

// functions that affect the board
impl Board {
    // return an empty board
    fn empty() -> Self {
        Board {
            board: [None; 64],
            can_en_passant: Cell::new(CanEnPassant::No),
            can_castle: Cell::new(CanCastle::new()),
        }
    }

    // return a board in the starting chess position.
    pub fn new() -> Self {
        let mut board = Self::empty();
        board.add_piece(
            Piece::Rook(Rook {
                color: Color::White,
            }),
            0,
        );
        board.add_piece(
            Piece::Knight(Knight {
                color: Color::White,
            }),
            1,
        );
        board.add_piece(
            Piece::Bishop(Bishop {
                color: Color::White,
            }),
            2,
        );
        board.add_piece(
            Piece::Queen(Queen {
                color: Color::White,
            }),
            3,
        );
        board.add_piece(
            Piece::King(King {
                color: Color::White,
            }),
            4,
        );
        board.add_piece(
            Piece::Bishop(Bishop {
                color: Color::White,
            }),
            5,
        );
        board.add_piece(
            Piece::Knight(Knight {
                color: Color::White,
            }),
            6,
        );
        board.add_piece(
            Piece::Rook(Rook {
                color: Color::White,
            }),
            7,
        );
        for i in 8..=15 {
            board.add_piece(
                Piece::Pawn(Pawn {
                    color: Color::White,
                }),
                i,
            );
        }

        board.add_piece(
            Piece::Rook(Rook {
                color: Color::Black,
            }),
            63,
        );
        board.add_piece(
            Piece::Knight(Knight {
                color: Color::Black,
            }),
            62,
        );
        board.add_piece(
            Piece::Bishop(Bishop {
                color: Color::Black,
            }),
            61,
        );
        board.add_piece(
            Piece::Queen(Queen {
                color: Color::Black,
            }),
            60,
        );
        board.add_piece(
            Piece::King(King {
                color: Color::Black,
            }),
            59,
        );
        board.add_piece(
            Piece::Bishop(Bishop {
                color: Color::Black,
            }),
            58,
        );
        board.add_piece(
            Piece::Knight(Knight {
                color: Color::Black,
            }),
            57,
        );
        board.add_piece(
            Piece::Rook(Rook {
                color: Color::Black,
            }),
            56,
        );
        for i in 48..=55 {
            board.add_piece(
                Piece::Pawn(Pawn {
                    color: Color::Black,
                }),
                i,
            );
        }

        board
    }

    // example board with all piece types
    pub fn example() -> Self {
        let mut board = Self::empty();

        board.add_piece(
            Piece::Pawn(Pawn {
                color: Color::White,
            }),
            9,
        );
        board.add_piece(
            Piece::Rook(Rook {
                color: Color::Black,
            }),
            18,
        );
        board.add_piece(
            Piece::King(King {
                color: Color::Black,
            }),
            17,
        );
        board.add_piece(
            Piece::Knight(Knight {
                color: Color::White,
            }),
            19,
        );
        board.add_piece(
            Piece::King(King {
                color: Color::White,
            }),
            63,
        );
        board.add_piece(
            Piece::Queen(Queen {
                color: Color::White,
            }),
            11,
        );

        board.add_piece(
            Piece::Bishop(Bishop {
                color: Color::White,
            }),
            36,
        );
        board
    }

    // add a piece to a specific board location
    fn add_piece(&mut self, piece: Piece, square_to_add_piece: usize) {
        self.board[square_to_add_piece] = Some(piece);
    }

    pub fn get_row(square: u8) -> u8 {
        square / 8
    }
    pub fn get_column(square: u8) -> u8 {
        square % 8
    }

    pub fn generate_moves(&self) -> HashMap<u8, Vec<u8>> {
        let mut all_moves = HashMap::new();
        let mut kings: [(Piece, u8); 2] = [(
            Piece::King(King {
                color: Color::White,
            }),
            64,
        ); 2];
        for (index, item) in self
            .board
            .into_iter()
            .enumerate()
            .filter(|tuple| tuple.1.is_some())
            .map(|tuple| (tuple.0, tuple.1.unwrap()))
        {
            if let Piece::King(_) = item {
                kings[if item.get_color().is_white() { 0 } else { 1 }] = (item, index as u8);
            } else {
                all_moves.insert(index as u8, item.get_moves(self, index as u8));
            }
        }
        for item in &kings {
            all_moves.insert(item.1, item.0.get_moves(self, item.1));
        }
        all_moves
    }

    pub fn is_check_simple(king_pos: usize, all_moves: &[u8]) -> bool {
        all_moves.iter().any(|a| *a == king_pos as u8)
    }
}

// Print board to the terminal
impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut board = String::new();
        for (square_counter, square) in self.board.into_iter().enumerate() {
            if let Some(piece) = square {
                match piece {
                    Piece::Pawn(piece) => {
                        board.push(if piece.color.is_white() { '♙' } else { '♟' })
                    }
                    Piece::Knight(piece) => {
                        board.push(if piece.color.is_white() { '♘' } else { '♞' })
                    }
                    Piece::Bishop(piece) => {
                        board.push(if piece.color.is_white() { '♗' } else { '♝' })
                    }
                    Piece::Rook(piece) => {
                        board.push(if piece.color.is_white() { '♖' } else { '♜' })
                    }
                    Piece::Queen(piece) => {
                        board.push(if piece.color.is_white() { '♕' } else { '♛' })
                    }
                    Piece::King(piece) => {
                        board.push(if piece.color.is_white() { '♔' } else { '♚' })
                    }
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
        Self::new()
    }
}
