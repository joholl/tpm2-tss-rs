#![feature(exclusive_range_pattern)]
#![feature(const_trait_impl)]

pub mod alg;
pub mod bitfields;
pub mod constants;
pub mod de;
pub mod enums;
pub mod handles;
pub mod selectables;
pub mod structs;
pub mod util;

pub mod types {

    use crate::alg::{
        AlgAsym, AlgAsymScheme, AlgCipherMode, AlgEccKeyEchange, AlgHash, AlgKdf, AlgMacScheme,
        AlgPublic, AlgSigScheme, AlgSym, AlgSymMode, AlgSymObj, EccCurve,
    };
    use crate::constants::CommandCode;
    use crate::de::{deserialize_u16_sized_vec, deserialize_u8_sized_vec};
    use crate::enums::{AESKeyBits, CAMELLIAKeyBits, RSAKeyBits, SM4KeyBits, TDESKeyBits};
    use crate::handles::Hierarchy;
    use crate::selectables::SymDefObject;
    use crate::structs::{ACTData, PCRSelection, TaggedPolicy};
    use serde::{
        de::{self, IgnoredAny, MapAccess, SeqAccess, Visitor},
        Deserialize, Deserializer, Serialize,
    };
    use serde_repr::{Deserialize_repr, Serialize_repr};
    use std::{
        any::{self, type_name},
        collections::HashMap,
        default, fmt, marker,
    };

    #[derive(Deserialize, Serialize, Debug, Clone, Default, PartialEq)]
    pub struct TPMS_COMMAND_HANDLES_CREATE_PRIMARY {
        pub primaryHandle: Hierarchy,
    }

    // TPM2B byte buffers are not aliased since we do not hold array size information, anyway
    // type TPM2B_DIGEST = Vec<u8>;
    // type TPM2B_DATA = Vec<u8>;
    // type TPM2B_NONCE = Vec<u8>;
    // type TPM2B_AUTH = Vec<u8>;
    // type TPM2B_OPERAND = Vec<u8>;
    // type TPM2B_EVENT = Vec<u8>;
    // type TPM2B_MAX_BUFFER = Vec<u8>;
    // type TPM2B_TIMEOUT = Vec<u8>;
    // type TPM2B_IV = Vec<u8>;
    // type TPM2B_NAME = Vec<u8>;
    // type TPM2B_ATTEST = Vec<u8>;
    // type TPM2B_SYM_KEY = Vec<u8>;
    // type TPM2B_LABEL = Vec<u8>;
    // type TPM2B_DERIVE = Vec<u8>;
    // type TPM2B_SENSITIVE_DATA = Vec<u8>;
    // type TPM2B_PUBLIC_KEY_RSA = Vec<u8>;
    // type TPM2B_PRIVATE_KEY_RSA = Vec<u8>;
    // type TPM2B_ECC_PARAMETER = Vec<u8>;
    // type TPM2B_ENCRYPTED_SECRET = Vec<u8>;
    // type TPM2B_TEMPLATE = Vec<u8>;
    // type TPM2B_PRIVATE_VENDOR_SPECIFIC = Vec<u8>;
    // type TPM2B_PRIVATE = Vec<u8>;
    // type TPM2B_ID_OBJECT = Vec<u8>;
    // type TPM2B_CONTEXT_SENSITIVE = Vec<u8>;
    // type TPM2B_CONTEXT_DATA = Vec<u8>;

    // Similarly, TPML types are not aliased since we do not hold array size information
    // type TPML_CC = Vec<CommandCode>; // count: u32
    // type TPML_CCA = Vec<CommandCodeAttributes>; // count: u32
    // type TPML_ALG = Vec<Alg>; // count: u32
    // type TPML_HANDLE = Vec<Handle>; // count: u32
    // type TPML_DIGEST = Vec<Vec<u8>>; // count: u32 (and Vec<u8> has size: u16)
    // type TPML_DIGEST_VALUES = Vec<Digest>; // count: u32
    // type TPML_PCR_SELECTION = Vec<PCRSelection>; // count: u32
    // type TPML_ALG_PROPERTY = Vec<AlgorithmProperty>; // count: u32
    // type TPML_TAGGED_TPM_PROPERTY = Vec<TaggedProperty>; // count: u32
    // type TPML_TAGGED_PCR_PROPERTY = Vec<TaggedPCRSelect>; // count: u32
    // type TPML_ECC_CURVE = Vec<EccCurve>; // count: u32
    // type TPML_TAGGED_POLICY = Vec<TaggedPolicy>; // count: u32
    // type TPML_ACT_DATA = Vec<ACTData>; // count: u32

    type TPMT_RSA_SCHEME = AsymScheme; // selected by TPMI_ALG_RSA_SCHEME

    /// TPMS_RSA_PARMS
    #[derive(Deserialize, Serialize, Debug, Clone, Default, PartialEq)]
    pub struct RSAParams {
        pub symmetric: SymDefObject, // TODO TPMT_SYM_DEF_OBJECT+
        pub scheme: TPMT_RSA_SCHEME, // TODO TPMT_SYM_DEF_OBJECT+
        pub key_bits: RSAKeyBits,
        pub exponent: u32,
    }

    type TPMS_SCHEME_HASH = AlgHash;

    pub type TPMS_KEY_SCHEME_ECDH = TPMS_SCHEME_HASH;
    pub type TPMS_KEY_SCHEME_ECMQV = TPMS_SCHEME_HASH;
    pub type TPMS_SIG_SCHEME_ECDAA = TPMS_SCHEME_HASH;
    pub type TPMS_SIG_SCHEME_ECDSA = TPMS_SCHEME_HASH;
    pub type TPMS_SIG_SCHEME_ECSCHNORR = TPMS_SCHEME_HASH;
    pub type TPMS_SIG_SCHEME_RSAPSS = TPMS_SCHEME_HASH;
    pub type TPMS_SIG_SCHEME_RSASSA = TPMS_SCHEME_HASH;
    pub type TPMS_SIG_SCHEME_SM2 = TPMS_SCHEME_HASH;
    pub type TPMS_ENC_SCHEME_OAEP = TPMS_SCHEME_HASH;
    pub type TPMS_ENC_SCHEME_RSAES = TPMS_SCHEME_HASH;

    /// TPMT_ASYM_SCHEME, TPMT_RSA_SCHEME, TPMT_RSA_DECRYPT, TPMT_ECC_SCHEME
    /// (incl. TPMU_ASYM_SCHEME)
    #[derive(Deserialize, Serialize, Debug, Default, Clone, PartialEq)]
    #[repr(u16)]
    pub enum AsymScheme {
        ECDH(TPMS_KEY_SCHEME_ECDH) = AlgAsymScheme::ECDH as u16,
        ECMQV(TPMS_KEY_SCHEME_ECMQV) = AlgAsymScheme::ECMQV as u16,
        ECDAA(TPMS_SIG_SCHEME_ECDAA) = AlgAsymScheme::ECDAA as u16,
        ECDSA(TPMS_SIG_SCHEME_ECDSA) = AlgAsymScheme::ECDSA as u16,
        ECSchnorr(TPMS_SIG_SCHEME_ECSCHNORR) = AlgAsymScheme::ECSCHNORR as u16,
        RSAPSS(TPMS_SIG_SCHEME_RSAPSS) = AlgAsymScheme::RSAPSS as u16,
        RSASSA(TPMS_SIG_SCHEME_RSASSA) = AlgAsymScheme::RSASSA as u16,
        SM2(TPMS_SIG_SCHEME_SM2) = AlgAsymScheme::SM2 as u16,
        OAEP(TPMS_ENC_SCHEME_OAEP) = AlgAsymScheme::OAEP as u16,
        RSAES(TPMS_ENC_SCHEME_RSAES) = AlgAsymScheme::RSAES as u16,
        #[default]
        Null = AlgAsymScheme::Null as u16,
    }

    pub type TPMS_SCHEME_MGF1 = TPMS_SCHEME_HASH;
    pub type TPMS_SCHEME_KDF1_SP800_108 = TPMS_SCHEME_HASH;
    pub type TPMS_SCHEME_KDF1_SP800_56A = TPMS_SCHEME_HASH;
    pub type TPMS_SCHEME_KDF2 = TPMS_SCHEME_HASH;

    /// TPMT_KDF_SCHEME (incl. TPMU_KDF_SCHEME)
    #[derive(Deserialize, Serialize, Debug, Default, Clone, PartialEq)]
    #[repr(u16)]
    pub enum KDFScheme {
        MGF1(TPMS_SCHEME_MGF1) = AlgKdf::MGF1 as u16,
        KDF1Sp800_108(TPMS_SCHEME_KDF1_SP800_108) = AlgKdf::Kdf1Sp800_108 as u16,
        Kdf1Sp800_56a(TPMS_SCHEME_KDF1_SP800_56A) = AlgKdf::Kdf1Sp800_56a as u16,
        KDF2(TPMS_SCHEME_KDF2) = AlgKdf::KDF2 as u16,
        #[default]
        Null = AlgKdf::Null as u16,
    }

    /// TPMS_ECC_PARMS
    #[derive(Deserialize, Serialize, Debug, Clone, Default, PartialEq)]
    pub struct ECCParams {
        pub symmetric: SymDefObject,
        pub scheme: AsymScheme,
        pub curve_id: EccCurve,
        pub kdf: KDFScheme,
    }

    /// TPMS_ECC_POINT
    #[derive(Deserialize, Serialize, Debug, Clone, Default, PartialEq)]
    pub struct TPMS_ECC_POINT {
        #[serde(deserialize_with = "deserialize_u16_sized_vec")]
        pub x: Vec<u8>,
        #[serde(deserialize_with = "deserialize_u16_sized_vec")]
        pub y: Vec<u8>,
    }

    pub type TPMA_OBJECT = u32; // TODO bitfield

    /// TPMT_PUBLIC (including TPMU_PUBLIC_PARMS, TPMU_PUBLIC_ID)
    #[derive(Deserialize, Debug, Serialize, Clone, PartialEq)]
    #[repr(u16)]
    pub enum Public {
        // TODO
        // keyedHash {
        //     nameAlg: AlgHash,
        //     objectAttributes: u32,
        //     authPolicy: TPM2B_DIGEST,
        //     parameters: TPMS_KEYEDHASH_PARMS,
        //     unique: TPM2B_DIGEST,
        // } = AlgPublic::KEYEDHASH as u16,
        // sym {
        //     nameAlg: AlgHash,
        //     objectAttributes: u32,
        //     authPolicy: TPM2B_DIGEST,
        //     parameters: TPMS_SYMCIPHER_PARMS,
        //     unique: TPM2B_DIGEST,
        // } = AlgPublic::SYMCIPHER as u16,
        RSA {
            name_alg: AlgHash,
            object_attributes: u32,
            auth_policy: Vec<u8>,
            parameters: RSAParams,
            #[serde(deserialize_with = "deserialize_u16_sized_vec")]
            unique: Vec<u8>,
        } = AlgPublic::RSA as u16,
        ECC {
            name_alg: AlgHash,
            object_attributes: u32,
            #[serde(deserialize_with = "deserialize_u16_sized_vec")]
            auth_policy: Vec<u8>,
            parameters: ECCParams,
            unique: TPMS_ECC_POINT,
        } = AlgPublic::ECC as u16,
    }

    impl Default for Public {
        fn default() -> Public {
            // TODO why does ..Default::default() result in an error?
            Public::ECC {
                name_alg: AlgHash::Null,
                object_attributes: Default::default(),
                auth_policy: Default::default(),
                parameters: Default::default(),
                unique: Default::default(),
            }
        }
    }

    /// TPM2B_PUBLIC
    //pub type PublicSized = StructWithSize<Public>;

    // pub struct TPMS_COMMAND_PARAMS_CREATE_PRIMARY {
    //     inSensitive: TPM2B_SENSITIVE_CREATE,
    //     inPublic: TPM2B_PUBLIC,
    //     outsideInfo: TPM2B_DATA,
    //     creationPCR: TPML_PCR_SELECTION,
    // }

    // pub struct Command<Handles, Params> {
    //     pub tag: u16,
    //     pub size: u32,
    //     //pub cc: u32,
    //     pub handles: Handles,
    //     pub params: Params,
    // }

    // pub type CreatePrimaryComand =
    //     Command<TPMS_COMMAND_HANDLES_CREATE_PRIMARY, TPMS_COMMAND_PARAMS_CREATE_PRIMARY>;

    #[derive(Deserialize, Debug, Clone, PartialEq)]
    pub struct TPML_PCR_SELECTION {
        count: u32,
        #[serde(deserialize_with = "deserialize_u16_sized_vec")]
        pcrSelections: Vec<TPMS_PCR_SELECTION>,
    }

    #[derive(Deserialize, Debug, Clone, PartialEq)]
    pub struct TPMS_PCR_SELECTION {
        hash: AlgHash,
        #[serde(deserialize_with = "deserialize_u8_sized_vec")]
        pcrSelect: Vec<u8>,
    }
}

fn is_normal<T: Sized + Send + Sync + Unpin>() {}

#[test]
fn normal_types() {
    is_normal::<types::Public>();
    // TODO do this for all pub types
}
