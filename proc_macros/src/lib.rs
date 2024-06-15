
use proc_macro as pc;
use proc_macro2 as pc2;
use quote::quote;
use syn::token;

#[proc_macro]
pub fn generate_perft_tests(item: pc::TokenStream) -> pc::TokenStream {
/*     let token_vec = item.into_iter(); */
    let mut stream = pc::TokenStream::new();
    let perft_info = syn::parse_macro_input!(item as PerftAnswers);
    dbg!(perft_info.clone());
    let fen = perft_info.fen.as_str();
    for (i,depth) in perft_info.depths_list.iter().enumerate() {
        let position_name = perft_info.position_name.clone();
        let i = i + 1;
        
        stream.extend(quote! {
            #[test]
            fn #position_name _ #i {
                let pos = crate::bitboard::Position::from_fen(#fen);
                #[cfg(feature="concurrent_hashmap")]
                static MAP: once_cell::sync::Lazy<chashmap::CHashMap<crate::bitboard::Position, u32>> = once_cell::sync::Lazy::new(|| chashmap::CHashMap::new());
                let mut moves_list: [Option<super::PossiblePieceMoves>; 16] = [POSS_MOVE; 16];
                let mut pieces_list: [u64; 16] = [0; 16];
                let mut positions_list_list: [[Option<crate::bitboard::move_generation::Move>; 219]; #i] = [POSITIONS_LIST; #i];
                #[cfg(feature="hashmap")]
                let map = &mut ahash::AHashMap::new();
        
                assert_eq!(pos.perft(&mut positions_list_list, &mut moves_list, &mut pieces_list, #[cfg(feature="hashmap")] map), $first, "Regular fail");
                #[cfg(not(feature="concurrent_hashmap"))]
                assert_eq!(pos.multi_thread_perft::<{(#i-1)}>(), $first, "Multi-threaded fail");
                #[cfg(feature="concurrent_hashmap")]
                assert_eq!(pos.multi_thread_perft::<{(#i-1)}>(Some(&MAP)), $first, "Hashmap fail");
            }
        });
    }
    
    stream

}

#[derive(Debug, Clone)]
struct PerftAnswers {
    depths_list: Vec<u32>,
    position_name: String,
    fen: String
}


impl syn::parse::Parse for PerftAnswers {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let mut depths_list = Vec::new();
        let position_name = input.parse::<pc2::Ident>()?.to_string();
        let punct = input.parse::<pc2::Punct>()?;
        if punct.as_char() != ',' {
            input.error("Position name and fen must be separated by a comma.");
        }
        let fen = input.parse::<pc2::Literal>()?.to_string();
        let punct = input.parse::<pc2::Punct>()?;
        if punct.as_char() != ',' {
            input.error("Fen and depths list must be separated by a comma.");
        }

        dbg!(input);
        let group = input.parse::<pc2::Group>()?;
        if !matches!(group.delimiter(), pc2::Delimiter::Bracket) {
            panic!("Last item must be a list contained by brackets, not {:?}", group.delimiter())
        }
        let stream_iter = group.stream().into_iter();


        for tt in stream_iter {
            match tt {
                pc2::TokenTree::Literal(lit) => {depths_list.push(syn::LitInt::from(lit).base10_parse::<u32>()?);},
                _ => panic!("fuck")
            }
            
        }

        Ok(PerftAnswers { depths_list, position_name, fen})

    }
}

