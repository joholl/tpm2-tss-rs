use core::panic;
use proc_macro::TokenStream;
use quote::quote;
use syn::{
    self, parse_macro_input, punctuated::Punctuated, Data, DataEnum, DeriveInput, Fields,
    FieldsUnnamed, Ident, Variant,
};

fn variant_to_tokenstream(
    variant: &Variant,
    source_type: &Ident,
    destination_type: &Ident,
    can_fail: bool,
    has_field: bool,
) -> proc_macro2::TokenStream {
    let variant_indent = &variant.ident;
    match (can_fail, has_field) {
        (false, false) => quote! {
            #source_type::#variant_indent => #destination_type::#variant_indent,
        },
        (true, false) => quote! {
            #source_type::#variant_indent => Ok(#destination_type::#variant_indent),
        },
        (false, true) => quote! {
            #source_type::#variant_indent(handle) => #destination_type::#variant_indent(handle),
        },
        (true, true) => quote! {
            #source_type::#variant_indent(handle) => Ok(#destination_type::#variant_indent(handle)),
        },
    }
}

fn match_arms_tokenstream(
    variants: &Punctuated<Variant, syn::token::Comma>,
    source_type: &Ident,
    destination_type: &Ident,
    can_fail: bool,
) -> Vec<proc_macro2::TokenStream> {
    let mut result: Vec<proc_macro2::TokenStream> = variants
        .iter()
        .map(|variant| match &variant.fields {
            Fields::Unit => {
                variant_to_tokenstream(variant, source_type, destination_type, can_fail, false)
            }
            Fields::Unnamed(FieldsUnnamed { unnamed, .. }) => {
                assert_eq!(unnamed.len(), 1);
                variant_to_tokenstream(variant, source_type, destination_type, can_fail, true)
            }
            _ => panic!("Expected unit or unnamed field for variant."),
        })
        .collect();
    if can_fail {
        result.push(quote! {_ => Err(()),})
    }

    result
}

pub fn handle_enum(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let variants = match input.data {
        Data::Enum(DataEnum { variants, .. }) => variants,
        _ => panic!("Expected enum."),
    };

    let enum_indent = input.ident;
    let handle_ident = Ident::new("Handle", enum_indent.span());
    let self_ident = syn::Ident::new("Self", enum_indent.span());

    let ident_to_self = match_arms_tokenstream(&variants, &enum_indent, &self_ident, false);
    let handle_to_self_can_fail =
        match_arms_tokenstream(&variants, &handle_ident, &self_ident, true);

    // TODO can we add doc strings?

    // Build the output, possibly using quasi-quotation
    let expanded = quote! {
        impl TryFrom<Handle> for #enum_indent {
            type Error = ();

            fn try_from(value: Handle) -> Result<Self, Self::Error> {
                match value {
                    #(#handle_to_self_can_fail)*
                }
            }
        }

        impl From<#enum_indent> for Handle {
            fn from(value: #enum_indent) -> Self {
                match value {
                    #(#ident_to_self)*
                }
            }
        }

        impl TryFrom<u32> for #enum_indent {
            type Error = ();

            fn try_from(value: u32) -> Result<Self, Self::Error> {
                #enum_indent::try_from(Handle::try_from(value)?)
            }
        }

        impl From<#enum_indent> for u32 {
            fn from(value: #enum_indent) -> Self {
                u32::from(Handle::from(value))
            }
        }

        impl<'de> Deserialize<'de> for #enum_indent {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: Deserializer<'de>,
            {
                struct HandleVisitor;
                impl<'de> Visitor<'de> for HandleVisitor {
                    type Value = #enum_indent;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                        formatter.write_str(&format!("u32 handle"))
                    }

                    fn visit_u32<E>(self, v: u32) -> Result<Self::Value, E>
                    where
                        E: serde::de::Error,
                    {
                        #enum_indent::try_from(v)
                            .map_err(|_| E::invalid_value(serde::de::Unexpected::Unsigned(v.into()), &self))
                    }
                }

                deserializer.deserialize_u32(HandleVisitor)
            }
        }

        impl Serialize for #enum_indent {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: Serializer,
            {
                serializer.serialize_u32(u32::from(*self))
            }
        }
    };

    // Hand the output tokens back to the compiler
    TokenStream::from(expanded)
}
