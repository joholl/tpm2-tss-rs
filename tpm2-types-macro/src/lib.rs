mod alg_enums;
mod alg_variants;
mod handle_enums;
use proc_macro::TokenStream;

#[proc_macro]
pub fn alg_enum_all(input: TokenStream) -> TokenStream {
    alg_enums::alg_enum_all(input)
}

#[proc_macro]
pub fn alg_enum_for_at_least(input: TokenStream) -> TokenStream {
    alg_enums::alg_enum_for_at_least(input)
}

#[proc_macro]
pub fn alg_enum_for_exactly(input: TokenStream) -> TokenStream {
    alg_enums::alg_enum_for_exactly(input)
}

#[proc_macro_derive(HandleSubset)]
pub fn handle_enum(input: TokenStream) -> TokenStream {
    handle_enums::handle_enum(input)
}
