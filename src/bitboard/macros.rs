#[macro_export]
macro_rules! implement_bitboard_trait {
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


            }

            impl std::fmt::Display for $type {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                    let mut board = String::new();
                    for i in 0..64 {
                        let this_piece = self.0 >> i & 1;
                        if i % 8 == 0 && i != 0 {
                            board.push('\n');
                        }
                        if this_piece == 1 {
                            board.push('x');
                        } else {
                            board.push('.')
                        }
                        board.push(' ');
                    }
                    write!(f, "{}", board.as_str())
                }
            }
        )*
    };
}
pub use implement_bitboard_trait;

#[macro_export]
macro_rules! move_in_line {
    ($moves_list:ident, $offset:ident, $piece:ident, $own_side:ident, $opp_side:ident, [$(($direction:literal, $shl_collision:path, $shr_collision:path)), +] ) => {
        {
            let all_pieces = $own_side | $opp_side;
            let mut left_to_loop = $piece;
            let mut current_piece:u64;
            while left_to_loop != 0 {
                current_piece = 1 << left_to_loop.trailing_zeros();
                let mut moves = 0;
                $(
                    if current_piece & $shl_collision == 0 {
                        let mut current_move = current_piece << $direction;
                        while current_move & $shl_collision == 0{
                            moves |= current_move;

                            if current_move & all_pieces != 0 {
                                break
                            }
                            current_move <<= $direction;
                        }
                        moves |= current_move;
                    }

                    if current_piece & $shr_collision == 0 {
                        let mut current_move = current_piece >> $direction;
                        while current_move & $shr_collision == 0 {
                            moves |= current_move;

                            if current_move & all_pieces != 0 {
                                break
                            }
                            current_move >>= $direction;
                        }
                        moves |= current_move;
                    }
                )+

                moves &= (!$own_side);
                $moves_list[*$offset] = super::Move(moves);
                *$offset += 1;
                left_to_loop &= (!current_piece);
            }

        }
    };
}
pub use move_in_line;

#[macro_export]
macro_rules! jump_moves {
    ($moves_list:ident, $offset:ident, $piece:ident, $own_side:ident, [$(($shift_amount:literal, $cant_go_left:expr, $cant_go_right:expr)), +]) => {
        let mut left_to_loop = $piece;
        let mut current_piece:u64;
        while left_to_loop != 0 {
            current_piece = 1<<left_to_loop.trailing_zeros();
            let mut moves = 0;

            $(
                {
                    const ALLOWED_LEFT: u64 = !($cant_go_left);
                    const ALLOWED_RIGHT: u64 = !($cant_go_right);
                    moves |= ((current_piece & ALLOWED_LEFT) << $shift_amount) | ((current_piece & ALLOWED_RIGHT) >> $shift_amount);
                }

            )+

            moves &= (!$own_side);
            $moves_list[*$offset] = super::Move(moves);
            *$offset += 1;
            left_to_loop &= (!current_piece);
        }
    };
}
pub use jump_moves;
