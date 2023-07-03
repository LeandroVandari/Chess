#[macro_export]
macro_rules! implement_bitboard {
    ($($type:ty),+) => {
    $(
        impl $crate::bitboard::BitBoard for $type {
            #[inline(always)]
    fn has_piece(&self, mask: &$crate::bitboard::Mask) -> bool {
        (self.0 & mask.0) != 0
    }

    #[inline(always)]
    fn add_piece(&mut self, mask: &$crate::bitboard::Mask) {
        self.0 |= mask.0
    }

    #[inline(always)]
    fn delete_piece(&mut self, mask: &$crate::bitboard::Mask) {
        self.0 &= mask.reverse().0
    }

    #[inline(always)]
    fn get_board(&self) -> u64 {
        self.0
    }
        })*
    };
}
pub use implement_bitboard;

#[macro_export]
macro_rules! move_in_line {
    (piece bitboard: $piece:ident, own side: $own_side:ident, opponent side: $opp_side:ident, directions and conditions: [$(($direction:literal, $shl_collision:path, $shr_collision:path)), +] ) => {
        {
            let all_pieces = $own_side | $opp_side;
            let mut left_to_loop = $piece;
            let mut current_piece = 1 << $piece.trailing_zeros();
            let mut moves: u64 = 0;
            while left_to_loop != 0 {
                $(
                    let mut current_move = current_piece << $direction;
                    while current_move & $shl_collision == 0{
                        moves |= current_move;
                        current_move <<= $direction;
                        if current_move & all_pieces != 0 {
                            break
                        }
                    }

                    let mut current_move = current_piece >> $direction;
                    while current_move & $shr_collision == 0 {
                        moves |= current_move;
                        current_move >>= $direction;
                        if current_move & all_pieces != 0 {
                            break
                        }
                    }
                )+

                moves &= (!$own_side);
                left_to_loop &= (!current_piece);
                current_piece = 1 << left_to_loop.trailing_zeros();
            }
            
            moves
        }
    };
}
pub use move_in_line;


macro_rules! move_in_directions {
    () => {
        
    };
}
