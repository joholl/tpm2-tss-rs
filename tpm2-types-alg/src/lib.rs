use core::panic;
use proc_macro::TokenStream;
use quote::quote;
use std::collections::HashSet;
use std::str::FromStr;
use syn::parse::ParseStream;
use syn::{self, Expr, ExprArray, Token};
use syn::{parse::Parse, parse_macro_input, Ident};

#[derive(Debug, Clone, Copy, PartialEq)]
struct AlgVariant {
    name: &'static str,
    value: u16,
    asym: bool,
    sym: bool,
    hash: bool,
    sign: bool,
    anon_sign: bool,
    enc: bool,
    meth: bool,
    obj: bool,
}

const ALGS: [AlgVariant; 39] = [
    AlgVariant {
        name: "Error",
        value: 0x0000,
        asym: false,
        sym: false,
        hash: false,
        sign: false,
        anon_sign: false,
        enc: false,
        meth: false,
        obj: false,
    },
    AlgVariant {
        name: "RSA",
        value: 0x0001,
        asym: true,
        sym: false,
        hash: false,
        sign: false,
        anon_sign: false,
        enc: false,
        meth: false,
        obj: true,
    },
    AlgVariant {
        name: "TDES",
        value: 0x0003,
        asym: false,
        sym: true,
        hash: false,
        sign: false,
        anon_sign: false,
        enc: false,
        meth: false,
        obj: false,
    },
    AlgVariant {
        name: "SHA1",
        value: 0x0004,
        asym: false,
        sym: false,
        hash: true,
        sign: false,
        anon_sign: false,
        enc: false,
        meth: false,
        obj: false,
    },
    AlgVariant {
        name: "HMAC",
        value: 0x0005,
        asym: false,
        sym: false,
        hash: true,
        sign: true,
        anon_sign: false,
        enc: false,
        meth: false,
        obj: false,
    },
    AlgVariant {
        name: "AES",
        value: 0x0006,
        asym: false,
        sym: true,
        hash: false,
        sign: false,
        anon_sign: false,
        enc: false,
        meth: false,
        obj: false,
    },
    AlgVariant {
        name: "MGF1",
        value: 0x0007,
        asym: false,
        sym: false,
        hash: true,
        sign: false,
        anon_sign: false,
        enc: false,
        meth: true,
        obj: false,
    },
    AlgVariant {
        name: "KeyedHash",
        value: 0x0008,
        asym: false,
        sym: false,
        hash: true,
        sign: false,
        anon_sign: false,
        enc: false,
        meth: false,
        obj: true,
    },
    AlgVariant {
        name: "XOR",
        value: 0x000A,
        asym: false,
        sym: true,
        hash: true,
        sign: false,
        anon_sign: false,
        enc: false,
        meth: false,
        obj: false,
    },
    AlgVariant {
        name: "SHA256",
        value: 0x000B,
        asym: false,
        sym: false,
        hash: true,
        sign: false,
        anon_sign: false,
        enc: false,
        meth: false,
        obj: false,
    },
    AlgVariant {
        name: "SHA384",
        value: 0x000C,
        asym: false,
        sym: false,
        hash: true,
        sign: false,
        anon_sign: false,
        enc: false,
        meth: false,
        obj: false,
    },
    AlgVariant {
        name: "SHA512",
        value: 0x000D,
        asym: false,
        sym: false,
        hash: true,
        sign: false,
        anon_sign: false,
        enc: false,
        meth: false,
        obj: false,
    },
    AlgVariant {
        name: "Null",
        value: 0x0010,
        asym: true,
        sym: true,
        hash: true,
        sign: true,
        anon_sign: true,
        enc: true,
        meth: true,
        obj: true,
    },
    AlgVariant {
        name: "SM3_256",
        value: 0x0012,
        asym: false,
        sym: false,
        hash: true,
        sign: false,
        anon_sign: false,
        enc: false,
        meth: false,
        obj: false,
    },
    AlgVariant {
        name: "SM4",
        value: 0x0013,
        asym: false,
        sym: true,
        hash: false,
        sign: false,
        anon_sign: false,
        enc: false,
        meth: false,
        obj: false,
    },
    AlgVariant {
        name: "RSASSA",
        value: 0x0014,
        asym: true,
        sym: false,
        hash: false,
        sign: true,
        anon_sign: false,
        enc: false,
        meth: false,
        obj: false,
    },
    AlgVariant {
        name: "RSAES",
        value: 0x0015,
        asym: true,
        sym: false,
        hash: false,
        sign: false,
        anon_sign: false,
        enc: true,
        meth: false,
        obj: false,
    },
    AlgVariant {
        name: "RSAPSS",
        value: 0x0016,
        asym: true,
        sym: false,
        hash: false,
        sign: true,
        anon_sign: false,
        enc: false,
        meth: false,
        obj: false,
    },
    AlgVariant {
        name: "OAEP",
        value: 0x0017,
        asym: true,
        sym: false,
        hash: true,
        sign: false,
        anon_sign: false,
        enc: true,
        meth: false,
        obj: false,
    },
    AlgVariant {
        name: "ECDSA",
        value: 0x0018,
        asym: true,
        sym: false,
        hash: false,
        sign: true,
        anon_sign: false,
        enc: false,
        meth: false,
        obj: false,
    },
    AlgVariant {
        name: "ECDH",
        value: 0x0019,
        asym: true,
        sym: false,
        hash: false,
        sign: false,
        anon_sign: false,
        enc: false,
        meth: true,
        obj: false,
    },
    AlgVariant {
        name: "ECDAA",
        value: 0x001A,
        asym: true,
        sym: false,
        hash: false,
        sign: true,
        anon_sign: true,
        enc: false,
        meth: false,
        obj: false,
    },
    AlgVariant {
        name: "SM2",
        value: 0x001B,
        asym: true,
        sym: false,
        hash: false,
        sign: true,
        anon_sign: false,
        enc: false,
        meth: false,
        obj: false,
    },
    AlgVariant {
        name: "ECSCHNORR",
        value: 0x001C,
        asym: true,
        sym: false,
        hash: false,
        sign: true,
        anon_sign: false,
        enc: false,
        meth: false,
        obj: false,
    },
    AlgVariant {
        name: "ECMQV",
        value: 0x001D,
        asym: true,
        sym: false,
        hash: false,
        sign: false,
        anon_sign: false,
        enc: false,
        meth: true,
        obj: false,
    },
    AlgVariant {
        name: "Kdf1Sp800_56a",
        value: 0x0020,
        asym: false,
        sym: false,
        hash: true,
        sign: false,
        anon_sign: false,
        enc: false,
        meth: true,
        obj: false,
    },
    AlgVariant {
        name: "KDF2",
        value: 0x0021,
        asym: false,
        sym: false,
        hash: true,
        sign: false,
        anon_sign: false,
        enc: false,
        meth: true,
        obj: false,
    },
    AlgVariant {
        name: "Kdf1Sp800_108",
        value: 0x0022,
        asym: false,
        sym: false,
        hash: true,
        sign: false,
        anon_sign: false,
        enc: false,
        meth: true,
        obj: false,
    },
    AlgVariant {
        name: "ECC",
        value: 0x0023,
        asym: true,
        sym: false,
        hash: false,
        sign: false,
        anon_sign: false,
        enc: false,
        meth: false,
        obj: true,
    },
    AlgVariant {
        name: "SYMCIPHER",
        value: 0x0025,
        asym: false,
        sym: true,
        hash: false,
        sign: false,
        anon_sign: false,
        enc: false,
        meth: false,
        obj: true,
    },
    AlgVariant {
        name: "CAMELLIA",
        value: 0x0026,
        asym: false,
        sym: true,
        hash: false,
        sign: false,
        anon_sign: false,
        enc: false,
        meth: false,
        obj: false,
    },
    AlgVariant {
        name: "SHA3_256",
        value: 0x0027,
        asym: false,
        sym: false,
        hash: true,
        sign: false,
        anon_sign: false,
        enc: false,
        meth: false,
        obj: false,
    },
    AlgVariant {
        name: "SHA3_384",
        value: 0x0028,
        asym: false,
        sym: false,
        hash: true,
        sign: false,
        anon_sign: false,
        enc: false,
        meth: false,
        obj: false,
    },
    AlgVariant {
        name: "SHA3_512",
        value: 0x0029,
        asym: false,
        sym: false,
        hash: true,
        sign: false,
        anon_sign: false,
        enc: false,
        meth: false,
        obj: false,
    },
    AlgVariant {
        name: "CTR",
        value: 0x0040,
        asym: false,
        sym: true,
        hash: false,
        sign: false,
        anon_sign: false,
        enc: true,
        meth: false,
        obj: false,
    },
    AlgVariant {
        name: "OFB",
        value: 0x0041,
        asym: false,
        sym: true,
        hash: false,
        sign: false,
        anon_sign: false,
        enc: true,
        meth: false,
        obj: false,
    },
    AlgVariant {
        name: "CBC",
        value: 0x0042,
        asym: false,
        sym: true,
        hash: false,
        sign: false,
        anon_sign: false,
        enc: true,
        meth: false,
        obj: false,
    },
    AlgVariant {
        name: "CFB",
        value: 0x0043,
        asym: false,
        sym: true,
        hash: false,
        sign: false,
        anon_sign: false,
        enc: true,
        meth: false,
        obj: false,
    },
    AlgVariant {
        name: "ECB",
        value: 0x0044,
        asym: false,
        sym: true,
        hash: false,
        sign: false,
        anon_sign: false,
        enc: true,
        meth: false,
        obj: false,
    },
];

impl AlgVariant {
    fn is_type(&self, name: &str) -> bool {
        match name {
            "asym" => self.asym,
            "sym" => self.sym,
            "hash" => self.hash,
            "sign" => self.sign,
            "anon_sign" => self.anon_sign,
            "enc" => self.enc,
            "meth" => self.meth,
            "obj" => self.obj,
            _ => panic!(),
        }
    }

    fn types(&self) -> HashSet<String> {
        let r = vec![
            "asym",
            "sym",
            "hash",
            "sign",
            "anon_sign",
            "enc",
            "meth",
            "obj",
        ]
        .iter()
        .filter_map(|t| match self.is_type(t) {
            true => Some(t.to_string()),
            false => None,
        })
        .collect();

        r
    }
}

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
