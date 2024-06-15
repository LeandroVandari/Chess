#[macro_export]
macro_rules! move_in_line {
    ($moves_struct:ident, $piece:ident, $piece_type:path, [$(($direction:literal, $shl_collision:expr, $shr_collision:expr)), +] ) => {
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
                    if current_piece & ($shl_collision) == 0 {
                        let mut current_move = current_piece << $direction;
                        while current_move & ($shl_collision) == 0{
                            moves |= current_move;

                            if current_move & all_pieces != 0 {
                                break
                            }
                            current_move <<= $direction;
                        }
                        moves |= current_move;
                    }

                    if current_piece & ($shr_collision) == 0 {
                        let mut current_move = current_piece >> $direction;
                        while current_move & ($shr_collision) == 0 {
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
                $moves_struct.moves_list[$moves_struct.offset] = Some(moves);
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
            $moves_struct.moves_list[$moves_struct.offset] = Some(moves);
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
                pub fn inner(&self) -> u64 {
                    self.0
                }

                #[inline(always)]
                #[must_use]
                pub fn inner_mut(&mut self) -> &mut u64 {
                    &mut self.0
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

/* #[macro_export]

macro_rules! perft_for_position {
    ($fen:literal, [$($expected_result:literal),+]) => {
        let pos = $crate::bitboard::Position::from_fen(&$crate::bitboard::Fen($fen));

        $(
            let mut moves_list: [Option<super::PossiblePieceMoves>; 16] = [POSS_MOVE; 16];
            let mut pieces_list: [u64; 16] = [0; 16];
            let mut positions_list_list: [[Option<$crate::bitboard::move_generation::Move>; 219]; ${index()} + 1] = [POSITIONS_LIST; ${index()} + 1];

            assert_eq!(pos.perft(&mut positions_list_list, &mut moves_list, &mut pieces_list), $expected_result);
        )+
    };
}
pub use perft_for_position; */

#[macro_export]
macro_rules! test_position_perft {
    (@internal $pos_name:ident, $fen:literal, $curr_depth:expr, [$last:literal]) => {
        paste::item! {
            #[test]
            fn [<$pos_name _depth_ $curr_depth:lower>]() {
                let pos = $crate::bitboard::Position::from_fen(&$fen);
                #[cfg(feature="concurrent_hashmap")]
                static MAP: once_cell::sync::Lazy<chashmap::CHashMap<$crate::bitboard::Position, u32>> = once_cell::sync::Lazy::new(|| chashmap::CHashMap::new());
                let mut moves_list: [Option<super::PossiblePieceMoves>; 16] = [POSS_MOVE; 16];
                let mut pieces_list: [u64; 16] = [0; 16];
                let mut positions_list_list: [[Option<$crate::bitboard::move_generation::Move>; 219]; $curr_depth] = [POSITIONS_LIST; $curr_depth];
                #[cfg(feature="hashmap")]
                let map = &mut ahash::AHashMap::new();

                assert_eq!(pos.perft(&mut positions_list_list, &mut moves_list, &mut pieces_list, #[cfg(feature="hashmap")] map), $last, "Regular fail");
                #[cfg(not(feature="concurrent_hashmap"))]
                assert_eq!(pos.multi_thread_perft::<{($curr_depth-1)}>(), $last, "Multi-threaded fail");
                #[cfg(feature="concurrent_hashmap")]
                assert_eq!(pos.multi_thread_perft::<{($curr_depth-1)}>(Some(&MAP)), $last, "Hashmap fail");
            }
        }
    };

    (@internal $pos_name:ident, $fen:literal, $curr_depth:expr, [$first:literal $($other_results:tt)*]) => {
        paste::item! {
            #[test]
            fn [<$pos_name _depth_ $curr_depth>]() {
                let pos = $crate::bitboard::Position::from_fen(&$fen);
                #[cfg(feature="concurrent_hashmap")]
                static MAP: once_cell::sync::Lazy<chashmap::CHashMap<$crate::bitboard::Position, u32>> = once_cell::sync::Lazy::new(|| chashmap::CHashMap::new());
                let mut moves_list: [Option<super::PossiblePieceMoves>; 16] = [POSS_MOVE; 16];
                let mut pieces_list: [u64; 16] = [0; 16];
                let mut positions_list_list: [[Option<$crate::bitboard::move_generation::Move>; 219]; $curr_depth] = [POSITIONS_LIST; $curr_depth];
                #[cfg(feature="hashmap")]
                let map = &mut ahash::AHashMap::new();

                assert_eq!(pos.perft(&mut positions_list_list, &mut moves_list, &mut pieces_list, #[cfg(feature="hashmap")] map), $first, "Regular fail");
                #[cfg(not(feature="concurrent_hashmap"))]
                assert_eq!(pos.multi_thread_perft::<{($curr_depth-1)}>(), $first, "Multi-threaded fail");
                #[cfg(feature="concurrent_hashmap")]
                assert_eq!(pos.multi_thread_perft::<{($curr_depth-1)}>(Some(&MAP)), $first, "Hashmap fail");
            }
        }




        $crate::test_position_perft!(@internal $pos_name, $fen, {($curr_depth+1)}, [$($other_results)*]);

    };
    ($pos_name:ident, $fen:literal, [$first:tt $($other_results:tt)*]) => {
        paste::item! {
            #[test]
            fn [<$pos_name _depth_1>]() {
                const CURR_DEPTH: usize = 1;
                //static MAP: once_cell::sync::Lazy<chashmap::CHashMap<$crate::bitboard::Position, u32>> = once_cell::sync::Lazy::new(|| chashmap::CHashMap::new());
                let pos = $crate::bitboard::Position::from_fen(&$fen);

                let mut moves_list: [Option<super::PossiblePieceMoves>; 16] = [POSS_MOVE; 16];
                let mut pieces_list: [u64; 16] = [0; 16];
                let mut positions_list_list: [[Option<$crate::bitboard::move_generation::Move>; 219]; CURR_DEPTH] = [POSITIONS_LIST; CURR_DEPTH];
                #[cfg(feature="hashmap")]
                let map = &mut ahash::AHashMap::new();
                assert_eq!(pos.perft(&mut positions_list_list, &mut moves_list, &mut pieces_list, #[cfg(feature="hashmap")] map), $first, "Regular fail");
                //assert_eq!(pos.multi_thread_perft::<0>(None), $first, "Multi-threaded fail");
                //assert_eq!(pos.multi_thread_perft::<0>(Some(&MAP)), $first, "Hashmap fail");
            }
        }
        $crate::test_position_perft!(@internal $pos_name, $fen, 2, [$($other_results)*]);
    };
}
pub use test_position_perft;

#[macro_export]
macro_rules! benchmark_position {
    ($c:ident, $position_fen:literal, $position_number:literal, [$($depth:literal),+]) => {
        {
            type PositionList = [Option<Move>; 219];

            const POSS_MOVE: Option<PossiblePieceMoves> = None;
            const POSITION: Option<Move> = None;
            const POSITIONS_LIST: PositionList = [POSITION; 219];

            let mut moves_list: [Option<PossiblePieceMoves>; 16] = [POSS_MOVE; 16];
            let mut pieces_list: [u64; 16] = [0; 16];
            let board = chess::bitboard::Position::from_fen($position_fen);
            #[cfg(feature="hashmap")]
            let map = &mut ahash::AHashMap::new();

            $(

                let mut positions_list_list = [POSITIONS_LIST;$depth];

                $c.bench_function(&format!("{}_move_ahead_position_{}", $depth, $position_number), |b| {
                    b.iter(|| {
                        let _ =  board.perft(&mut positions_list_list, &mut moves_list, &mut pieces_list, #[cfg(feature="hashmap")] map);
                        #[cfg(feature="hashmap")]
                        map.clear();
                    })
                });

                $c.bench_function(&format!("multi_threaded_{}_move_ahead_position_{}", $depth, $position_number), |b| {
                    b.iter(|| {
                        let _ =  board.multi_thread_perft::<{$depth-1}>();
                    })
                });
                #[cfg(feature="concurrent_hashmap")]
                $c.bench_function(&format!("hashmapped_multi_threaded_{}_move_ahead_position_{}", $depth, $position_number), |b| {
                    b.iter(|| {
                        static MAP: once_cell::sync::Lazy<chashmap::CHashMap<$crate::bitboard::Position, u32>> = once_cell::sync::Lazy::new(|| chashmap::CHashMap::new());
                        let _ =  board.multi_thread_perft::<{$depth-1}>(&MAP);
                    })
                });
            )+
        }
    };


}
pub use benchmark_position;

#[macro_export]
macro_rules! make_variables_for_perft {
    ($depth:literal, $moves_list_list:ident, $moves_list:ident, $pieces_list:ident) => {
        const DEPTH: usize = $depth;
        const OTHER_DEPTH: usize = DEPTH - 1;
        const POSS_MOVE: Option<bb::PossiblePieceMoves> = None;
        const POSITION: Option<bb::move_generation::Move> = None;
        const POSITIONS_LIST: [Option<bb::move_generation::Move>; 219] = [POSITION; 219];

        let mut $moves_list: [Option<bb::PossiblePieceMoves>; 16] = [POSS_MOVE; 16];
        let mut $pieces_list: [u64; 16] = [0; 16];
        let mut $moves_list_list: [[Option<bb::move_generation::Move>; 219]; DEPTH] =
            [POSITIONS_LIST; DEPTH];
    };
}
pub use make_variables_for_perft;
