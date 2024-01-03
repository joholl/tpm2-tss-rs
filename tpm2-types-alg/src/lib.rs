mod alg_variants;

use alg_variants::{AlgVariant, ALGS};
use core::panic;
use proc_macro::TokenStream;
use quote::quote;
use std::collections::HashSet;
use std::str::FromStr;
use syn::parse::ParseStream;
use syn::{self, Expr, ExprArray, Token};
use syn::{parse::Parse, parse_macro_input, Ident};

#[derive(Debug)]
struct InputSyntax {
    spec_name: Ident,
    enum_ident: Ident,
    types: ExprArray,
    additional_variants: ExprArray,
}

impl Parse for InputSyntax {
    fn parse(input: ParseStream) -> Result<InputSyntax, syn::Error> {
        let spec_name = input.parse()?;
        let _: Token![,] = input.parse()?;
        let enum_ident = input.parse()?;
        let _: Token![,] = input.parse()?;
        let types = input.parse()?;
        let _: Token![,] = input.parse()?;
        let additional_variants = input.parse()?;
        Ok(Self {
            spec_name,
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
    let InputSyntax {
        spec_name,
        enum_ident,
        types,
        additional_variants,
    } = parse_macro_input!(input as InputSyntax);

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

    let variants: Vec<AlgVariant> = ALGS
        .iter()
        .filter_map(|alg| {
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
                Some(*alg)
            } else {
                None
            }
        })
        .collect();

    let spec_name = spec_name.to_string();

    let variant_tokenstream = variants.iter().map(|alg| {
        let name = Ident::new(alg.name, enum_ident.span());
        let value = alg.value;
        quote! {#name = #value,}
    });
    let u16_to_enum_tokenstream = variants.iter().map(|alg| {
        let name = Ident::new(alg.name, enum_ident.span());
        let value = alg.value;
        quote! {#value  => Ok(#enum_ident::#name),}
    });
    let enum_to_u16_tokenstream = variants.iter().map(|alg| {
        let name = Ident::new(alg.name, enum_ident.span());
        let value = alg.value;
        quote! {#enum_ident::#name => #value,}
    });
    let alg_to_enum_tokenstream = variants.iter().map(|alg| {
        let name = Ident::new(alg.name, enum_ident.span());
        quote! {Alg::#name  => Ok(#enum_ident::#name),}
    });
    let enum_to_alg_tokenstream = variants.iter().map(|alg| {
        let name = Ident::new(alg.name, enum_ident.span());
        quote! {#enum_ident::#name => Alg::#name,}
    });

    let test_conversion = syn::Ident::new(
        &format!("test_conversion_{}", enum_ident.to_string().to_lowercase()),
        enum_ident.span(),
    );

    let output = quote! {
        #[doc=#spec_name]
        #[derive(Deserialize_repr, Serialize_repr, Debug, PartialEq, Clone)]
        #[repr(u16)]
        pub enum #enum_ident {
            #(#variant_tokenstream)*
        }

        impl TryFrom<u16> for #enum_ident {
            type Error = ();

            fn try_from(value: u16) -> Result<Self, Self::Error> {
                match value {
                    #(#u16_to_enum_tokenstream)*
                    _ => Err(()),
                }
            }
        }

        impl From<#enum_ident> for u16 {
            fn from(value: #enum_ident) -> Self {
                match value {
                    #(#enum_to_u16_tokenstream)*
                }
            }
        }

        impl TryFrom<Alg> for #enum_ident {
            type Error = ();

            fn try_from(value: Alg) -> Result<Self, Self::Error> {
                match value {
                    #(#alg_to_enum_tokenstream)*
                    _ => Err(()),
                }
            }
        }

        impl From<#enum_ident> for Alg {
            fn from(value: #enum_ident) -> Self {
                match value {
                    #(#enum_to_alg_tokenstream)*
                }
            }
        }

        #[test]
        fn #test_conversion() {
            // enum <-> u16
            // every Alg has 0x0010/Null
            let variant: Result<#enum_ident, _> = 0x0010u16.try_into();
            assert_eq!(variant, Ok(#enum_ident::Null));
            let number: u16 = #enum_ident::Null.into();
            assert_eq!(number, 0x0010);

            // no Alg has 0x0000/Error
            let variant: Result<#enum_ident, _> = 0x0000u16.try_into();
            assert_eq!(variant, Err(()));

            // enum <-> Alg
            // every Alg has Null
            let variant: Result<#enum_ident, _> = Alg::Null.try_into();
            assert_eq!(variant, Ok(#enum_ident::Null));
            let variant: Alg = #enum_ident::Null.into();
            assert_eq!(variant, Alg::Null);

            // no Alg has 0x0000/Error
            let variant: Result<#enum_ident, _> = Alg::Error.try_into();
            assert_eq!(variant, Err(()));
        }
    };

    output.into()
}

#[derive(Debug)]
struct InputSyntax2 {
    spec_name: Ident,
    enum_ident: Ident,
}

impl Parse for InputSyntax2 {
    fn parse(input: ParseStream) -> Result<InputSyntax2, syn::Error> {
        let spec_name = input.parse()?;
        let _: Token![,] = input.parse()?;
        let enum_ident = input.parse()?;
        Ok(Self {
            spec_name,
            enum_ident,
        })
    }
}

#[proc_macro]
pub fn alg_enum_all(input: TokenStream) -> TokenStream {
    let InputSyntax2 {
        spec_name,
        enum_ident,
    } = parse_macro_input!(input as InputSyntax2);

    assert_eq!(enum_ident.to_string(), "Alg");

    let variants: Vec<AlgVariant> = ALGS.to_vec();
    let spec_name = spec_name.to_string();

    let variant_tokenstream = variants.iter().map(|alg| {
        let name = Ident::new(alg.name, enum_ident.span());
        let value = alg.value;
        quote! {#name = #value,}
    });
    let u16_to_enum_tokenstream = variants.iter().map(|alg| {
        let name = Ident::new(alg.name, enum_ident.span());
        let value = alg.value;
        quote! {#value  => Ok(#enum_ident::#name),}
    });
    let enum_to_u16_tokenstream = variants.iter().map(|alg| {
        let name = Ident::new(alg.name, enum_ident.span());
        let value = alg.value;
        quote! {#enum_ident::#name => #value,}
    });

    let test_conversion = syn::Ident::new(
        &format!("test_conversion_{}", enum_ident.to_string().to_lowercase()),
        enum_ident.span(),
    );

    let output = quote! {
        #[doc=#spec_name]
        #[derive(Deserialize_repr, Serialize_repr, Debug, PartialEq, Clone)]
        #[repr(u16)]
        pub enum #enum_ident {
            #(#variant_tokenstream)*
        }

        impl TryFrom<u16> for #enum_ident {
            type Error = ();

            fn try_from(value: u16) -> Result<Self, <Self as TryFrom::<u16>>::Error> {
                match value {
                    #(#u16_to_enum_tokenstream)*
                    _ => Err(()),
                }
            }
        }

        impl From<#enum_ident> for u16 {
            fn from(value: #enum_ident) -> Self {
                match value {
                    #(#enum_to_u16_tokenstream)*
                }
            }
        }

        #[test]
        fn #test_conversion() {
            // enum <-> u16
            // Alg has 0x0010/Null
            let variant: Result<#enum_ident, _> = 0x0010u16.try_into();
            assert_eq!(variant, Ok(#enum_ident::Null));
            let number: u16 = #enum_ident::Null.into();
            assert_eq!(number, 0x0010);

            // Alg has no 0xffff
            let variant: Result<#enum_ident, _> = 0xffffu16.try_into();
            assert_eq!(variant, Err(()));
        }
    };

    output.into()
}
