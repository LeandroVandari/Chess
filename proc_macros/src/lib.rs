use proc_macro as pc;
use proc_macro2 as pc2;
use quote::quote;

#[proc_macro]
pub fn generate_perft_tests(item: pc::TokenStream) -> pc::TokenStream {
    /*     let token_vec = item.into_iter(); */
    let mut functions = quote! {};
    let perft_info = syn::parse_macro_input!(item as PerftAnswers);
    dbg!(perft_info.clone());
    let fen = perft_info.fen;
    for (i, depth_answer) in perft_info.depths_list.iter().enumerate() {
        let mut position_name = perft_info.position_name.clone();
        let i = pc2::Literal::usize_unsuffixed(i + 1);
        position_name.push_str(&format!("_depth_{}", i));
        let function_name = pc2::Ident::new(&position_name, pc2::Span::call_site());
        let depth_answer = pc2::Literal::u128_unsuffixed(*depth_answer);
        functions.extend( quote! {
            #[test]
            fn #function_name() {
                let pos = crate::bitboard::Position::from_fen(#fen);
                #[cfg(feature="concurrent_hashmap")]
                static MAP: once_cell::sync::Lazy<chashmap::CHashMap<crate::bitboard::Position, u32>> = once_cell::sync::Lazy::new(|| chashmap::CHashMap::new());
                let mut moves_list: [Option<super::PossiblePieceMoves>; 16] = [POSS_MOVE; 16];
                let mut pieces_list: [u64; 16] = [0; 16];
                let mut positions_list_list: [[Option<crate::bitboard::move_generation::Move>; 219]; #i] = [POSITIONS_LIST; #i];
                #[cfg(feature="hashmap")]
                let map = &mut ahash::AHashMap::new();
        
                assert_eq!(pos.perft(&mut positions_list_list, &mut moves_list, &mut pieces_list, #[cfg(feature="hashmap")] map), #depth_answer, "Regular fail");
                #[cfg(not(feature="concurrent_hashmap"))]
                assert_eq!(pos.multi_thread_perft::<{(#i-1)}>(), #depth_answer, "Multi-threaded fail");
                #[cfg(feature="concurrent_hashmap")]
                assert_eq!(pos.multi_thread_perft::<{(#i-1)}>(Some(&MAP)), #depth_answer, "Hashmap fail");
            }
        });
    }
    println!("{}", functions);

    functions.into()
}

#[derive(Debug, Clone)]
struct PerftAnswers {
    depths_list: Vec<u128>,
    position_name: String,
    fen: pc2::Literal,
}

impl syn::parse::Parse for PerftAnswers {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let mut depths_list = Vec::new();
        let position_name = input.parse::<pc2::Ident>()?.to_string();
        let punct = input.parse::<pc2::Punct>()?;
        if punct.as_char() != ',' {
            input.error("Position name and fen must be separated by a comma.");
        }
        let fen = input.parse::<pc2::Literal>()?;
        let punct = input.parse::<pc2::Punct>()?;
        if punct.as_char() != ',' {
            input.error("Fen and depths list must be separated by a comma.");
        }
        let group = input.parse::<pc2::Group>()?;
        if !matches!(group.delimiter(), pc2::Delimiter::Bracket) {
            panic!(
                "Last item must be a list contained by brackets, not {:?}",
                group.delimiter()
            )
        }
        let stream_iter = group.stream().into_iter();

        for tt in stream_iter {
            match tt {
                pc2::TokenTree::Literal(lit) => {
                    depths_list.push(syn::LitInt::from(lit).base10_parse::<u128>()?);
                }
                _ => panic!("fuck"),
            }
        }

        Ok(PerftAnswers {
            depths_list,
            position_name,
            fen,
        })
    }
}
