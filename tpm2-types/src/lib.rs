pub mod de;

pub mod types {
    //use crate::de::{StructWithSizeVisitor, WithSize};
    use serde_repr::{Deserialize_repr, Serialize_repr};
    use std::{
        any::{self, type_name},
        collections::HashMap,
        default, fmt, marker,
    };
    use subenum::subenum;

    use serde::{
        de::{self, IgnoredAny, MapAccess, SeqAccess, Visitor},
        Deserialize, Deserializer, Serialize,
    };

    use crate::de::StructWithSize;

    #[subenum(
        AlgAsym,
        AlgSym,
        AlgHash,
        AlgSign,
        AlgAnonSign,
        AlgEnc,
        AlgMeth,
        AlgObj
    )]
    #[derive(Deserialize_repr, Serialize_repr, Debug, Clone, PartialEq)]
    #[repr(u16)]
    pub enum Alg {
        Error = 0x0000,
        #[subenum(AlgAsym, AlgObj)]
        RSA = 0x0001,
        #[subenum(AlgSym)]
        TDES = 0x0003,
        #[subenum(AlgHash)]
        SHA1 = 0x0004,
        #[subenum(AlgHash, AlgSign)]
        HMAC = 0x0005,
        #[subenum(AlgSym)]
        AES = 0x0006,
        #[subenum(AlgHash, AlgMeth)]
        MGF1 = 0x0007,
        #[subenum(AlgHash, AlgObj)]
        KeyedHash = 0x0008,
        #[subenum(AlgHash, AlgSym)]
        XOR = 0x000A,
        #[subenum(AlgHash)]
        SHA256 = 0x000B,
        #[subenum(AlgHash)]
        SHA384 = 0x000C,
        #[subenum(AlgHash)]
        SHA512 = 0x000D,
        #[subenum(
            AlgAsym,
            AlgSym,
            AlgHash,
            AlgSign,
            AlgAnonSign,
            AlgEnc,
            AlgMeth,
            AlgObj
        )]
        Null = 0x0010,
        #[subenum(AlgHash)]
        SM3_256 = 0x0012,
        #[subenum(AlgSym)]
        SM4 = 0x0013,
        #[subenum(AlgAsym, AlgSign)]
        RSASSA = 0x0014,
        #[subenum(AlgAsym, AlgEnc)]
        RSAES = 0x0015,
        #[subenum(AlgAsym, AlgSign)]
        RSAPSS = 0x0016,
        #[subenum(AlgAsym, AlgEnc, AlgSign)]
        OAEP = 0x0017,
        #[subenum(AlgAsym, AlgSign)]
        ECDSA = 0x0018,
        #[subenum(AlgAsym, AlgMeth)]
        ECDH = 0x0019,
        #[subenum(AlgAsym, AlgSign, AlgAnonSign)]
        ECDAA = 0x001A,
        #[subenum(AlgAsym, AlgSign)]
        SM2 = 0x001B,
        #[subenum(AlgAsym, AlgSign)]
        ECSCHNORR = 0x001C,
        #[subenum(AlgAsym, AlgMeth)]
        ECMQV = 0x001D,
        #[subenum(AlgHash, AlgMeth)]
        Kdf1Sp800_56a = 0x0020,
        #[subenum(AlgHash, AlgMeth)]
        KDF2 = 0x0021,
        #[subenum(AlgHash, AlgMeth)]
        Kdf1Sp800_108 = 0x0022,
        #[subenum(AlgAsym, AlgObj)]
        ECC = 0x0023,
        #[subenum(AlgObj, AlgSym)]
        SYMCIPHER = 0x0025,
        #[subenum(AlgSym)]
        CAMELLIA = 0x0026,
        #[subenum(AlgHash)]
        SHA3_256 = 0x0027,
        #[subenum(AlgHash)]
        SHA3_384 = 0x0028,
        #[subenum(AlgHash)]
        SHA3_512 = 0x0029,
        #[subenum(AlgSym, AlgEnc)]
        CMAC = 0x003F,
        #[subenum(AlgSym, AlgEnc)]
        CTR = 0x0040,
        #[subenum(AlgSym, AlgEnc)]
        OFB = 0x0041,
        #[subenum(AlgSym, AlgEnc)]
        CBC = 0x0042,
        #[subenum(AlgSym, AlgEnc)]
        CFB = 0x0043,
        #[subenum(AlgSym, AlgEnc)]
        ECB = 0x0044,
    }

    #[derive(Deserialize_repr, Serialize_repr, Debug, Clone, Default, PartialEq)]
    #[repr(u16)]
    pub enum EccCurve {
        #[default]
        None = 0x0000,
        NistP192 = 0x0001,
        NistP224 = 0x0002,
        NistP256 = 0x0003,
        NistP384 = 0x0004,
        NistP521 = 0x0005,
        BnP256 = 0x0010,
        BnP638 = 0x0011,
        Sm2P256 = 0x0020,
    }

    #[derive(Deserialize, Serialize, Debug, Clone, Default, PartialEq)]
    pub struct TPM_HANDLE(u32);

    #[derive(Deserialize, Serialize, Debug, Clone, Default, PartialEq)]
    pub struct TPMI_RH_HIERARCHY(TPM_HANDLE);

    #[derive(Deserialize, Serialize, Debug, Clone, Default, PartialEq)]
    pub struct TPMS_COMMAND_HANDLES_CREATE_PRIMARY {
        pub primaryHandle: TPMI_RH_HIERARCHY,
    }

    type TPM2B_DIGEST = Vec<u8>;
    type TPM2B_SENSITIVE_CREATE = Vec<u8>;
    type TPM2B_DATA = Vec<u8>;
    type TPM2B_PUBLIC_KEY_RSA = Vec<u8>;
    type TPM2B_ECC_PARAMETER = Vec<u8>;

    pub type TPM_ALG_ID = u16;
    pub type TPMI_ALG_SYM_OBJECT = TPM_ALG_ID;
    pub type TPMI_TDES_KEY_BITS = TPM_ALG_ID;
    pub type TPMI_AES_KEY_BITS = TPM_ALG_ID;
    pub type TPMI_SM4_KEY_BITS = TPM_ALG_ID;
    pub type TPMI_CAMELLIA_KEY_BITS = TPM_ALG_ID;
    pub type TPM_KEY_BITS = u16;
    pub type TPMI_ALG_HASH = AlgHash;
    pub type TPMI_ALG_SYM_MODE = TPM_ALG_ID;
    pub type TPMI_ALG_ECC_SCHEME = TPM_ALG_ID;
    pub type TPMI_ECC_CURVE = EccCurve;
    pub type TPMI_ALG_KDF = TPM_ALG_ID;
    pub type TPMI_ALG_PUBLIC = TPM_ALG_ID;
    pub type TPMI_RSA_KEY_BITS = TPM_KEY_BITS;

    type TPMT_RSA_SCHEME = AsymScheme; // selected by TPMI_ALG_RSA_SCHEME

    /// TPMS_RSA_PARMS
    #[derive(Deserialize, Serialize, Debug, Clone, Default, PartialEq)]
    pub struct RSAParams {
        pub symmetric: SymDefObject, // TODO TPMT_SYM_DEF_OBJECT+
        pub scheme: TPMT_RSA_SCHEME, // TODO TPMT_SYM_DEF_OBJECT+
        pub key_bits: TPMI_RSA_KEY_BITS,
        pub exponent: u32,
    }

    /// TPMT_SYM_DEF_OBJECT (including TPMU_SYM_DEF_OBJECT)
    #[derive(Deserialize, Serialize, Debug, Clone, Default, PartialEq)]
    #[repr(u16)]
    #[serde(use_repr)]
    pub enum SymDefObject {
        TDES {
            key_bits: TPMI_TDES_KEY_BITS,
            mode: TPMI_ALG_SYM_MODE,
        } = Alg::TDES as u16,
        AES {
            key_bits: TPMI_AES_KEY_BITS,
            mode: TPMI_ALG_SYM_MODE,
        } = Alg::AES as u16,
        SM4 {
            key_bits: TPMI_SM4_KEY_BITS,
            mode: TPMI_ALG_SYM_MODE,
        } = Alg::SM4 as u16,
        Camellia {
            key_bits: TPMI_CAMELLIA_KEY_BITS,
            mode: TPMI_ALG_SYM_MODE,
        } = Alg::CAMELLIA as u16,
        XOR {
            key_bits: TPMI_ALG_HASH,
        } = Alg::XOR as u16,
        #[default]
        Null = Alg::Null as u16,
    }

    type TPMS_SCHEME_HASH = TPMI_ALG_HASH;

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
    #[serde(use_repr)]
    pub enum AsymScheme {
        ECDH(TPMS_KEY_SCHEME_ECDH) = Alg::ECDH as u16,
        ECMQV(TPMS_KEY_SCHEME_ECMQV) = Alg::ECMQV as u16,
        ECDAA(TPMS_SIG_SCHEME_ECDAA) = Alg::ECDAA as u16,
        ECDSA(TPMS_SIG_SCHEME_ECDSA) = Alg::ECDSA as u16,
        ECSchnorr(TPMS_SIG_SCHEME_ECSCHNORR) = Alg::ECSCHNORR as u16,
        RSAPSS(TPMS_SIG_SCHEME_RSAPSS) = Alg::RSAPSS as u16,
        RSASSA(TPMS_SIG_SCHEME_RSASSA) = Alg::RSASSA as u16,
        SM2(TPMS_SIG_SCHEME_SM2) = Alg::SM2 as u16,
        OAEP(TPMS_ENC_SCHEME_OAEP) = Alg::OAEP as u16,
        RSAES(TPMS_ENC_SCHEME_RSAES) = Alg::RSAES as u16,
        AnySig(TPMS_SCHEME_HASH) = 0xffff,
        #[default]
        Null = Alg::Null as u16,
    }

    pub type TPMS_SCHEME_MGF1 = TPMS_SCHEME_HASH;
    pub type TPMS_SCHEME_KDF1_SP800_108 = TPMS_SCHEME_HASH;
    pub type TPMS_SCHEME_KDF1_SP800_56A = TPMS_SCHEME_HASH;
    pub type TPMS_SCHEME_KDF2 = TPMS_SCHEME_HASH;

    /// TPMT_KDF_SCHEME (incl. TPMU_KDF_SCHEME)
    #[derive(Deserialize, Serialize, Debug, Default, Clone, PartialEq)]
    #[repr(u16)]
    #[serde(use_repr)]
    pub enum KDFScheme {
        MGF1(TPMS_SCHEME_MGF1) = Alg::MGF1 as u16,
        KDF1Sp800_108(TPMS_SCHEME_KDF1_SP800_108) = Alg::Kdf1Sp800_108 as u16,
        Kdf1Sp800_56a(TPMS_SCHEME_KDF1_SP800_56A) = Alg::Kdf1Sp800_56a as u16,
        KDF2(TPMS_SCHEME_KDF2) = Alg::KDF2 as u16,
        #[default]
        Null = Alg::Null as u16,
    }

    /// TPMS_ECC_PARMS
    #[derive(Deserialize, Serialize, Debug, Clone, Default, PartialEq)]
    pub struct ECCParams {
        pub symmetric: SymDefObject,
        pub scheme: AsymScheme,
        pub curve_id: TPMI_ECC_CURVE,
        pub kdf: KDFScheme,
    }

    /// TPMS_ECC_POINT
    #[derive(Deserialize, Serialize, Debug, Clone, Default, PartialEq)]
    pub struct TPMS_ECC_POINT {
        pub x: TPM2B_ECC_PARAMETER,
        pub y: TPM2B_ECC_PARAMETER,
    }

    pub type TPMA_OBJECT = u32; // TODO bitfield

    /// TPMT_PUBLIC (including TPMU_PUBLIC_PARMS, TPMU_PUBLIC_ID)
    #[derive(Deserialize, Debug, Serialize, Clone, PartialEq)]
    #[repr(u16)]
    #[serde(use_repr)]
    pub enum Public {
        // TODO
        // keyedHash {
        //     nameAlg: TPMI_ALG_HASH,
        //     objectAttributes: u32,
        //     authPolicy: TPM2B_DIGEST,
        //     parameters: TPMS_KEYEDHASH_PARMS,
        //     unique: TPM2B_DIGEST,
        // } = TPM_ALG.KEYEDHASH as u16,
        // sym {
        //     nameAlg: TPMI_ALG_HASH,
        //     objectAttributes: u32,
        //     authPolicy: TPM2B_DIGEST,
        //     parameters: TPMS_SYMCIPHER_PARMS,
        //     unique: TPM2B_DIGEST,
        // } = TPM_ALG.SYMCIPHER as u16,
        RSA {
            name_alg: TPMI_ALG_HASH,
            object_attributes: u32,
            auth_policy: TPM2B_DIGEST,
            parameters: RSAParams,
            unique: TPM2B_PUBLIC_KEY_RSA,
        } = Alg::RSA as u16,
        ECC {
            name_alg: TPMI_ALG_HASH,
            object_attributes: u32,
            auth_policy: TPM2B_DIGEST,
            parameters: ECCParams,
            unique: TPMS_ECC_POINT,
        } = Alg::ECC as u16,
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
    pub type PublicSized = StructWithSize<Public>;

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
}

fn is_normal<T: Sized + Send + Sync + Unpin>() {}

#[test]
fn normal_types() {
    is_normal::<types::Public>();
    // TODO do this for all pub types
}
