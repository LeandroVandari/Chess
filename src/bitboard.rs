
type Piece = u8;
type Color = u8;
const PAWN: Piece = 0b001;
const KNIGHT: Piece = 0b010;
const BISHOP: Piece = 0b011;
const ROOK: Piece = 0b100;
const QUEEN: Piece = 0b101;
const KING: Piece = 0b110;
const PIECE_SIZE: u8 = 4;

struct Board {
    squares: u128
}

enum CanEnPassant {
    Yes(u8),
    No,
}
struct State {
    board: Board,
    en_passant: CanEnPassant,
    can_castle: u8,
    white_king_pos: u8,
    black_king_pos: u8,
    is_check: bool
}
impl State {
    fn new() -> State {
        
    }
}



impl Board {
    fn new() -> Self {
        Self {squares: 0b0}
    }

    fn add_piece(&mut self, piece: Piece, pos: u8, color: Color) {
        self.unset_piece(pos);
        
        self.squares |= ((color << 4 | piece) << (pos * PIECE_SIZE));
    }
    fn unset_piece(&mut self, pos: u8) {
        self.squares &= !(0b1111 << (pos * PIECE_SIZE))
    }
}