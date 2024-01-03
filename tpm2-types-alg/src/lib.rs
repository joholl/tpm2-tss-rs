mod alg_variants;

use alg_variants::ALGS;
use core::panic;
use proc_macro::TokenStream;
use quote::quote;
use std::collections::HashSet;
use std::str::FromStr;
use syn::parse::ParseStream;
use syn::{self, Expr, ExprArray, Token};
use syn::{parse::Parse, parse_macro_input, Ident};

#[derive(Debug)]
struct IndentAndArray {
    enum_ident: Ident,
    types: ExprArray,
    additional_variants: ExprArray,
}

impl Parse for IndentAndArray {
    fn parse(input: ParseStream) -> Result<IndentAndArray, syn::Error> {
        let enum_ident = input.parse()?;
        let _: Token![,] = input.parse()?;
        let types = input.parse()?;
        let _: Token![,] = input.parse()?;
        let additional_variants = input.parse()?;
        Ok(Self {
            enum_ident,
            types,
            additional_variants,
        })
    }
}

enum Mode {
    Exactly,
    AtLeast,
}

#[proc_macro]
pub fn alg_enum_for_exactly(input: TokenStream) -> TokenStream {
    alg_enum(input, Mode::Exactly)
}

#[proc_macro]
pub fn alg_enum_for_at_least(input: TokenStream) -> TokenStream {
    alg_enum(input, Mode::AtLeast)
}

/// Input tokens are enum name, arrays of type arrays, array of additional variants
/// e.g. AlgSymMode, [[sym, enc], [sym, sign]], [Null]
fn alg_enum(input: TokenStream, mode: Mode) -> TokenStream {
    let IndentAndArray {
        enum_ident,
        types,
        additional_variants,
    } = parse_macro_input!(input as IndentAndArray);

    // e.g. Vec(HashSet("asym", "sign"), HashSet("asym", "enc"))
    let sets_of_types: Vec<HashSet<String>> = types
        .elems
        .iter()
        .map(|nested_array| match nested_array {
            Expr::Array(expr_array) => expr_array
                .elems
                .iter()
                .map(|type_name| match type_name {
                    Expr::Path(expr_path) => {
                        assert_eq!(expr_path.path.segments.len(), 1);
                        expr_path.path.segments[0].ident.to_string()
                    }
                    _ => panic!(
                        "Unexpected expression for array element: {:#?}. Expected e.g. asym.",
                        type_name
                    ),
                })
                .collect(),
            _ => panic!(
                "Unexpected expression for array element: {:#?}. Expected nested array.",
                nested_array
            ),
        })
        .collect();

    // e.g. Vec("Null", "HMAC", "XOR")
    let additional_variants: Vec<String> = additional_variants
        .elems
        .iter()
        .map(|variant_name| match variant_name {
            Expr::Path(expr_path) => {
                assert_eq!(expr_path.path.segments.len(), 1);
                let name = expr_path.path.segments[0].ident.to_string();
                assert!(ALGS.iter().any(|alg| alg.name == name));
                name
            }
            _ => panic!(
                "Unexpected expression for variant: {:#?}. Expected e.g. HMAC.",
                variant_name
            ),
        })
        .collect();

    let variants = ALGS.iter().filter_map(|alg| {
        // filter_map maps for Some(...) and filters away for None

        let is_selected_by_type = match mode {
            Mode::Exactly => sets_of_types
                .iter()
                .any(|set_of_types| *set_of_types == alg.types()),
            Mode::AtLeast => sets_of_types
                .iter()
                .any(|set_of_types| set_of_types.is_subset(&alg.types())),
        };
        let is_selected_as_additional_variant =
            additional_variants.contains(&String::from_str(alg.name).unwrap());

        if is_selected_by_type || is_selected_as_additional_variant {
            let name = Ident::new(alg.name, enum_ident.span());
            let value = alg.value;
            Some(quote! {#name = #value,})
        } else {
            None
        }
    });

    let new_enum = quote! {
        #[derive(Deserialize_repr, Serialize_repr, Debug, PartialEq, Clone)]
        #[repr(u16)]
        pub enum #enum_ident {
            #(#variants)*
        }
    };

    new_enum.into()
}
