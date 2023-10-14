#[macro_export]
macro_rules! move_in_line {
    ($moves_struct:ident, $piece:ident, $piece_type:path, [$(($direction:literal, $shl_collision:path, $shr_collision:path)), +] ) => {
        {
            let all_pieces = $moves_struct.own_side | $moves_struct.other_side;
            let mut left_to_loop = $piece;
            if left_to_loop != 0 {
                $moves_struct.pieces_start[$piece_type] = Some($moves_struct.offset);

            }
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


                moves &= (!$moves_struct.own_side);
                $moves_struct.moves_list[$moves_struct.offset] = Some(super::PossiblePieceMoves(moves));
                $moves_struct.pieces_list[$moves_struct.offset] = current_piece;
                $moves_struct.all_attacks |= moves;
                $moves_struct.offset += 1;
                left_to_loop &= (!current_piece);
            }

        }
    };
}
pub(crate) use move_in_line;

#[macro_export]
macro_rules! jump_moves {
    ($moves_struct:ident, $piece:ident, $piece_start:path, [$(($shift_amount:literal, $cant_go_left:expr, $cant_go_right:expr)), +]) => {
        let mut left_to_loop = $piece;
        if left_to_loop != 0 {
            $moves_struct.pieces_start[$piece_start] = Some($moves_struct.offset);
        }
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


            moves &= (!$moves_struct.own_side);
            $moves_struct.moves_list[$moves_struct.offset] = Some(super::PossiblePieceMoves(moves));
            $moves_struct.pieces_list[$moves_struct.offset] = current_piece;
            $moves_struct.all_attacks |= moves;
            $moves_struct.offset += 1;
            left_to_loop &= (!current_piece);
        }
    };
}
pub(crate) use jump_moves;

#[macro_export]
macro_rules! implement_from_for_corresponding_values {
    (@from_ref $t1:ty {$($infinite_pattern_matching:literal)?}, $t2:ty {$($t1_value:path => $t2_value:path),+}) => {
        impl From<&$t1> for $t2 {
            fn from(value: &$t1) -> Self {
                match value {
                    $(
                        &$t1_value => $t2_value,
                    )+
                    $(_ => panic!("Invalid value for conversion from {} to {}: {}", std::any::type_name::<$t1>(), std::any::type_name::<$t2>(), $infinite_pattern_matching))?
                }
            }
        }
    };
    (@normal $t1:ty {$($infinite_pattern_matching:literal)?}, $t2:ty {$($t1_value:path => $t2_value:path),+}) => {
        impl From<$t1> for $t2 {
            fn from(value: $t1) -> Self {
                match value {
                    $(
                        $t1_value => $t2_value,
                    )+
                    $(_ => panic!("Invalid value for conversion from {} to {}: {}", std::any::type_name::<$t1>(), std::any::type_name::<$t2>(), $infinite_pattern_matching))?
                }
            }
        }
    };
    (@rev_normal $t1:ty, $t2:ty {$($infinite_pattern_matching:literal)?} {$($t1_value:path => $t2_value:path),+}) => {
        impl From<$t1> for $t2 {
            fn from(value: $t1) -> Self {
                match value {
                    $(
                        $t2_value => $t1_value,
                    )+
                    $(_ => panic!("Invalid value for conversion from {} to {}: {}", std::any::type_name::<$t1>(), std::any::type_name::<$t2>(), $infinite_pattern_matching))?
                }
            }
        }
    };
    (@rev_from_ref $t1:ty, $t2:ty {$($infinite_pattern_matching:literal)?} {$($t1_value:path => $t2_value:path),+}) => {
        impl From<&$t1> for $t2 {
            fn from(value: &$t1) -> Self {
                match value {
                    $(
                        &$t2_value => $t1_value,
                    )+
                    $(_ => panic!("Invalid value for conversion from &{} to {}: {}", std::any::type_name::<$t1>(), std::any::type_name::<$t2>(), $infinite_pattern_matching))?
                }
            }
        }
    };

    ($t1:tt $($infinite_pattern_matching1:literal)?, $t2:tt $($infinite_pattern_matching2:literal)? {$correspondences:tt}) => {
        $crate::implement_from_for_corresponding_values!(@from_ref $t1 {$($infinite_pattern_matching1)?}, $t2 $correspondences);
        $crate::implement_from_for_corresponding_values!(@normal $t1 {$($infinite_pattern_matching1)?}, $t2 $correspondences);
        $crate::implement_from_for_corresponding_values!(@rev_normal $t2, $t1 {$($infinite_pattern_matching2)?} $correspondences);
        $crate::implement_from_for_corresponding_values!(@rev_from_ref $t2, $t1 {$($infinite_pattern_matching2)?} $correspondences);

    }
}

pub(crate) use implement_from_for_corresponding_values;

/// Implement some useful traits for a type that wraps a [u64] and functions as a bitboard. Also implements [Display](std::fmt::Display).
#[macro_export]
macro_rules! implement_bitboard_functions {
    ($($type:ty),+) => {
        $(
            impl $type {
                #[inline(always)]
                #[must_use]
                pub fn has_piece(&self, mask: &$crate::bitboard::Mask) -> bool {
                    (self.0 & mask.0) != 0
                }

                #[inline(always)]
                pub fn add_piece(&mut self, mask: &$crate::bitboard::Mask) {
                    self.0 |= mask.0
                }

                #[inline(always)]
                pub fn delete_piece(&mut self, mask: &$crate::bitboard::Mask) {
                    self.0 &= !(mask.0)
                }

                #[inline(always)]
                #[must_use]
                pub fn inner(&self) -> u64 {
                    self.0
                }

                #[inline(always)]
                #[must_use]
                pub const fn new(inner: u64) -> Self {
                    Self(inner)
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
pub use implement_bitboard_functions;
