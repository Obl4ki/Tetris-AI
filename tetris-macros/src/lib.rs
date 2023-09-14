use proc_macro::TokenStream;

use quote::quote;
use syn::parse::{Parse, ParseStream};
use syn::{parse_macro_input, Ident, Token};

#[derive(Debug)]
struct GenBoardInput {
    pub cols: Vec<Col>,
}

#[derive(Debug)]
struct Col {
    pub values: Vec<Ident>,
}

impl Parse for GenBoardInput {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut rows: Vec<Col> = vec![];

        loop {
            let mut values = vec![];

            if input.parse::<Token![|]>().is_err() {
                break;
            }

            for _ in 0..10 {
                let id: Ident = input.parse()?;
                values.push(id);
            }

            input.parse::<Token![|]>()?;

            rows.push(Col { values });
        }

        Ok(Self {
            cols: rows.into_iter().rev().collect(),
        })
    }
}

#[proc_macro]
pub fn gen_game(input: TokenStream) -> TokenStream {
    let GenBoardInput { cols } = parse_macro_input!(input as GenBoardInput);
    let mut ast = quote! {
            use tetris_core::entities::{PieceType, Coord};
            use tetris_core::game_builder::GameBuilder;

            fn str_into_piece_type(piece_str: &str) -> PieceType {
                match piece_str {
                    "o" => PieceType::O,
                    "i" => PieceType::I,
                    "z" => PieceType::Z,
                    "j" => PieceType::J,
                    "t" => PieceType::T,
                    "s" => PieceType::S,
                    "l" => PieceType::L,
                    _ => panic!("Only i|u|z|j|t|s|l|x|o are allowed."),
                }
            }
            let mut game_builder = GameBuilder::new();
    };

    for (y, col) in cols.into_iter().enumerate() {
        for (x, id) in col.values.into_iter().enumerate() {
            let cell_value = id.to_string();
            let val = cell_value.as_str();
            match val {
                "i" | "u" | "z" | "j" | "t" | "s" | "l" | "o" => {
                    ast = quote!(
                        #ast
                        game_builder = game_builder.add_piece(str_into_piece_type(#val), Coord::new(#x, #y));
                    )
                }
                "x" => continue,
                e => panic!("Only i|u|z|j|t|s|l|o|x are allowed, found {e} in ({x}, {y})."),
            };
        }
    }

    ast = quote! {
        #ast
        let game = game_builder.build();
        game
    };

    ast = quote! {
        {
            #ast
        }
    };

    ast.into()
}
