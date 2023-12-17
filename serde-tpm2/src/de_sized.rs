use proc_macro2::TokenStream;
use quote::{quote, quote_spanned};
use syn::{Data, DeriveInput, Fields, Ident};

#[proc_macro_derive(Deserialize_sized)]
pub fn derive_deserialize_sized(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    // Parse the input tokens into a syntax tree
    let input = syn::parse_macro_input!(input as DeriveInput);

    // Ensure the input is a struct
    if let Data::Struct(data) = &input.data {
        // Get the struct name
        let struct_name = &input.ident;

        // Generate the struct visitor code
        let visitor_code = generate_visitor_code(struct_name, &data.fields);

        // Generate the final code
        let expanded = quote! {
            #[derive(Debug, Clone, Default, PartialEq)]
            pub struct #struct_name {
                #visitor_code
            }

            impl<'de> serde::de::Deserialize<'de> for #struct_name {
                fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
                where
                    D: serde::Deserializer<'de>,
                {
                    deserializer.deserialize_tuple(1, #struct_name::deserialize_tuple_visitor)
                }
            }
        };

        // Return the generated code as a token stream
        expanded.into()
    } else {
        // If the input is not a struct, return a compilation error
        let error_msg = "Deserialize_sized can only be used on structs";
        proc_macro::TokenStream::from(quote_spanned! {input.span()=> compile_error!(#error_msg); })
    }
}

fn generate_visitor_code(struct_name: &Ident, fields: &Fields) -> TokenStream {
    match fields {
        Fields::Named(named_fields) => {
            let field_name = named_fields.named.iter().map(|f| &f.ident);
            quote! {
                #(
                    pub #field_name: <#field_name as serde::de::Deserialize<'de>>::Owned,
                )*
            }
        }
        Fields::Unnamed(unnamed_fields) => {
            let field_type = unnamed_fields.unnamed.iter().map(|f| &f.ty);
            quote! {
                #(
                    pub #field_type,
                )*
            }
        }
        Fields::Unit => TokenStream::new(),
    }
}

mod tests {
    use super::*;

    #[test]
    fn test_derive_deserialize_sized() {
        let input: TokenStream = quote! {
            #[derive(Deserialize_sized, Debug, Clone, Default, PartialEq)]
            pub struct TPM2B_PUBLIC {
                pub publicArea: TPMT_PUBLIC,
            }
        };

        let expected: TokenStream = quote! {
            #[derive(Debug, Clone, Default, PartialEq)]
            pub struct TPM2B_PUBLIC {
                pub publicArea: <TPMT_PUBLIC as serde::de::Deserialize<'de>>::Owned,
            }

            impl<'de> serde::de::Deserialize<'de> for TPM2B_PUBLIC {
                fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
                where
                    D: serde::Deserializer<'de>,
                {
                    deserializer.deserialize_tuple(1, TPM2B_PUBLIC::deserialize_tuple_visitor)
                }
            }
        };

        assert_eq!(
            expected.to_string(),
            derive_deserialize_sized(input).to_string()
        );
    }
}
