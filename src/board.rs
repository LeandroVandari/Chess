use crate::{down, up, Move};

use super::{Bishop, Color, King, Knight, Pawn, Piece, Queen, Rook};
use std::collections::HashMap;
use std::fmt;
use std::hash::Hash;

// The board. Is wrapped in a struct in order to implement methods.
#[derive(Debug, Clone, PartialEq, Hash, Eq)]
pub struct Board {
    pub board: [Option<Piece>; 64],
    pub can_en_passant: CanEnPassant,
    pub can_castle: CanCastle,
    pub white_king_pos: u8,
    pub black_king_pos: u8,
    pub is_check: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Hash, Eq)]
pub enum CanEnPassant {
    Yes(u8),
    No,
}

#[derive(Clone, Copy, Debug, PartialEq, Hash, Eq)]
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
    /* fn all_to_false(&mut self) {
        self.black_kingside = false;
        self.black_queenside = false;
        self.white_kingside = false;
        self.white_queenside = false;
    } */
    fn all_to_false_with_color(&mut self, color: Color) {
        if let Color::White = color {
            self.white_kingside = false;
            self.white_queenside = false;
        } else {
            self.black_kingside = false;
            self.black_queenside = false;
        }
    }
}

// functions that affect the board
impl Board {
    // return an empty board
    fn empty() -> Self {
        Board {
            board: [None; 64],
            can_en_passant: CanEnPassant::No,
            can_castle: CanCastle::new(),
            white_king_pos: 4,
            black_king_pos: 60,
            is_check: false,
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
            59,
        );
        board.add_piece(
            Piece::King(King {
                color: Color::Black,
            }),
            60,
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

    pub fn for_castle() -> Self {
        let mut board = Self::new();
        board.board[5] = None;
        board.board[6] = None;
        board
    }

    // example board with all piece types
    pub fn example() -> Self {
        let mut board = Self::empty();
        board.black_king_pos = 17;
        board.white_king_pos = 63;

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

    // Get a FEN string and return a Board struct.
    pub fn from_fen(fen: &str) -> Self {
        // Create an empty Board to later mutate it.
        let mut board = Self::empty();
        // Index at which it will change the board (i. e. add the piece).
        let mut square: usize = 0;

        // For each character in the string, convert it to a piece in the board, and add it.
        for ch in fen.chars() {
            match ch {
                // Empty squares
                '1'..='8' => square += ch.to_digit(10).unwrap() as usize,
                // White pawn
                'p' => {
                    board.add_piece(
                        Piece::Pawn(Pawn {
                            color: Color::Black,
                        }),
                        ((8 * (7 - Self::get_row(square as u8))) + Self::get_column(square as u8))
                            as usize,
                    );
                    square += 1;
                }

                // White knight
                'n' => {
                    board.add_piece(
                        Piece::Knight(Knight {
                            color: Color::Black,
                        }),
                        ((8 * (7 - Self::get_row(square as u8))) + Self::get_column(square as u8))
                            as usize,
                    );
                    square += 1;
                }

                // White bishop
                'b' => {
                    board.add_piece(
                        Piece::Bishop(Bishop {
                            color: Color::Black,
                        }),
                        ((8 * (7 - Self::get_row(square as u8))) + Self::get_column(square as u8))
                            as usize,
                    );
                    square += 1;
                }

                // White rook
                'r' => {
                    board.add_piece(
                        Piece::Rook(Rook {
                            color: Color::Black,
                        }),
                        ((8 * (7 - Self::get_row(square as u8))) + Self::get_column(square as u8))
                            as usize,
                    );
                    square += 1;
                }

                // White queen
                'q' => {
                    board.add_piece(
                        Piece::Queen(Queen {
                            color: Color::Black,
                        }),
                        ((8 * (7 - Self::get_row(square as u8))) + Self::get_column(square as u8))
                            as usize,
                    );
                    square += 1;
                }

                // White king
                'k' => {
                    board.add_piece(
                        Piece::King(King {
                            color: Color::Black,
                        }),
                        ((8 * (7 - Self::get_row(square as u8))) + Self::get_column(square as u8))
                            as usize,
                    );
                    board.black_king_pos =
                        (8 * (7 - Self::get_row(square as u8))) + Self::get_column(square as u8);
                    square += 1;
                }

                // Black pawn
                'P' => {
                    board.add_piece(
                        Piece::Pawn(Pawn {
                            color: Color::White,
                        }),
                        ((8 * (7 - Self::get_row(square as u8))) + Self::get_column(square as u8))
                            as usize,
                    );
                    square += 1;
                }

                // Black knight
                'N' => {
                    board.add_piece(
                        Piece::Knight(Knight {
                            color: Color::White,
                        }),
                        ((8 * (7 - Self::get_row(square as u8))) + Self::get_column(square as u8))
                            as usize,
                    );
                    square += 1;
                }

                // Black bishop
                'B' => {
                    board.add_piece(
                        Piece::Bishop(Bishop {
                            color: Color::White,
                        }),
                        ((8 * (7 - Self::get_row(square as u8))) + Self::get_column(square as u8))
                            as usize,
                    );
                    square += 1;
                }

                // Black rook
                'R' => {
                    board.add_piece(
                        Piece::Rook(Rook {
                            color: Color::White,
                        }),
                        ((8 * (7 - Self::get_row(square as u8))) + Self::get_column(square as u8))
                            as usize,
                    );
                    square += 1;
                }

                // Black queen
                'Q' => {
                    board.add_piece(
                        Piece::Queen(Queen {
                            color: Color::White,
                        }),
                        ((8 * (7 - Self::get_row(square as u8))) + Self::get_column(square as u8))
                            as usize,
                    );
                    square += 1;
                }

                //Black king
                'K' => {
                    board.add_piece(
                        Piece::King(King {
                            color: Color::White,
                        }),
                        ((8 * (7 - Self::get_row(square as u8))) + Self::get_column(square as u8))
                            as usize,
                    );
                    board.white_king_pos =
                        (8 * (7 - Self::get_row(square as u8))) + Self::get_column(square as u8);
                    square += 1;
                }

                _ => (),
            }
        }
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

    pub fn generate_moves(&self, color: Color) -> HashMap<u8, Vec<Move>> {
        let mut all_moves = HashMap::new();

        for (index, item) in self
            .board
            .into_iter()
            .enumerate()
            .filter(|tuple| is_some_and_same_color(tuple.1, color))
        {
            all_moves.insert(index as u8, item.unwrap().get_moves(self, index as u8));
        }
        all_moves
    }

    pub fn make_move(&self, start_square: usize, end_square: Move, color: Color) -> Self {
        let mut clone: Board = self.clone();
        match end_square {
            Move::RegularMove(sqr) => {
                if let Some(Piece::King(_)) = self.board[start_square] {
                    match color {
                        Color::White => {
                            clone.white_king_pos = sqr;
                            clone.can_castle.white_kingside = false;
                            clone.can_castle.white_queenside = false;
                        }
                        Color::Black => {
                            clone.black_king_pos = sqr;
                            clone.can_castle.black_kingside = false;
                            clone.can_castle.black_queenside = false;
                        }
                    }
                } else if let Some(Piece::Rook(_)) = self.board[start_square] {
                    match color {
                        Color::White => match start_square {
                            0 => clone.can_castle.white_queenside = false,
                            7 => clone.can_castle.white_kingside = false,
                            _ => (),
                        },
                        Color::Black => match start_square {
                            63 => clone.can_castle.black_kingside = false,
                            56 => clone.can_castle.black_queenside = false,
                            _ => (),
                        },
                    }
                } else {
                    match sqr {
                        0 => clone.can_castle.white_queenside = false,
                        7 => clone.can_castle.white_kingside = false,
                        63 => clone.can_castle.black_kingside = false,
                        56 => clone.can_castle.black_queenside = false,
                        _ => (),
                    }
                }
                clone.board[sqr as usize] = clone.board[start_square];
                clone.board[start_square] = None;
                clone.can_en_passant = CanEnPassant::No;
            }
            Move::PawnAdvanceTwoSquares(sqr) => {
                clone.board[sqr as usize] = clone.board[start_square];
                clone.board[start_square] = None;
                clone.can_en_passant = CanEnPassant::Yes(sqr);
            }
            Move::CastleKingside => {
                if color.is_white() {
                    clone.board.swap(4, 6);
                    clone.board.swap(5, 7);
                    clone.white_king_pos = 6;
                } else {
                    clone.board.swap(60, 62);
                    clone.board.swap(63, 61);
                    clone.black_king_pos = 62;
                }
                clone.can_castle.all_to_false_with_color(color);
                clone.can_en_passant = CanEnPassant::No;
            }
            Move::CastleQueenside => {
                if color.is_white() {
                    clone.white_king_pos = 2;
                    clone.board.swap(4, 2);
                    clone.board.swap(0, 3);
                } else {
                    clone.black_king_pos = 58;
                    clone.board.swap(60, 58);
                    clone.board.swap(56, 59);
                }
                clone.can_castle.all_to_false_with_color(color);
                clone.can_en_passant = CanEnPassant::No;
            }

            Move::PawnPromotion(sqr, new_piece) => {
                clone.board[start_square] = None;
                clone.board[sqr as usize] = Some(new_piece);
                clone.can_en_passant = CanEnPassant::No;
            }

            Move::EnPassant(sqr) => {
                clone.board[sqr as usize] = clone.board[start_square];
                clone.board[if let Color::White = color {
                    down(sqr as usize).unwrap() as usize
                } else {
                    up(sqr as usize).unwrap() as usize
                }] = None;
                clone.board[start_square] = None;
                clone.can_en_passant = CanEnPassant::No;
            }
        }

        clone
    }
}

fn is_some_and_same_color(possible_piece: Option<Piece>, color: Color) -> bool {
    if possible_piece.is_none() {
        return false;
    }
    if possible_piece.unwrap().get_color() == color {
        return true;
    }
    false
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
