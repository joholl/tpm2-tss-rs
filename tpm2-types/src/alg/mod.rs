use serde_repr::{Deserialize_repr, Serialize_repr};
use tpm2_types_macro::{alg_enum_all, alg_enum_for_at_least, alg_enum_for_exactly};

// Spec Notation:
//  * !ALG.AX and !ALG.AE: alg_enum_for_exactly!(SpecName, EnumName, [[asym, sign], [asym, enc]], [Null]);
//  * !ALG.ax and !ALG.ae: alg_enum_for_at_least!(SpecName, EnumName, [[asym, sign], [asym, enc]], [Null]);

// TODO document type selectors in doc string?

// Having an enum with all variants called "Alg" is necessary.
alg_enum_all!(TPM_ALG_ID, Alg);

alg_enum_for_exactly!(TPMI_ALG_HASH, AlgHash, [[hash]], [Null]);

alg_enum_for_exactly!(TPMI_ALG_ASYM, AlgAsym, [[asym, obj]], [Null]);

alg_enum_for_exactly!(TPMI_ALG_SYM, AlgSym, [[sym]], [Null, XOR]);

alg_enum_for_exactly!(TPMI_ALG_SYM_OBJECT, AlgSymObj, [[sym]], [Null]);

alg_enum_for_exactly!(
    TPMI_ALG_SYM_MODE,
    AlgSymMode,
    [[sym, enc], [sym, sign]],
    [Null]
);

alg_enum_for_exactly!(TPMI_ALG_KDF, AlgKdf, [[hash, meth]], [Null]);

alg_enum_for_at_least!(
    TPMI_ALG_SIG_SCHEME,
    AlgSigScheme,
    [[asym, sign]],
    [Null, HMAC]
);

// TODO Again, this is only ECC algorithms. Luckily we do not have any other
// (for now).
alg_enum_for_exactly!(
    TPMI_ECC_KEY_EXCHANGE,
    AlgEccKeyEchange,
    [[asym, meth]],
    [Null, SM2]
);

alg_enum_for_exactly!(
    TPMI_ALG_MAC_SCHEME,
    AlgMacScheme,
    [[sym, sign], [hash]],
    [Null]
);

alg_enum_for_exactly!(TPMI_ALG_CIPHER_MODE, AlgCipherMode, [[sym, enc]], [Null]);

alg_enum_for_at_least!(
    TPMI_ALG_KEYEDHASH_SCHEME,
    AlgKeyedHashScheme,
    [],
    [Null, HMAC, XOR]
);

alg_enum_for_at_least!(
    TPMI_ALG_ASYM_SCHEME,
    AlgAsymScheme,
    [[asym, meth], [asym, sign], [asym, enc]],
    [Null]
);

// Spec notation: TPM_ALG_!ALG.ae.ax ([[asym, enc], [asym, sign]])
// TODO However, this is only RSA algorithms. For now, hardcode.
alg_enum_for_at_least!(
    TPMI_ALG_RSA_SCHEME,
    AlgRSAScheme,
    [],
    [RSASSA, RSAES, RSAPSS, OAEP, Null]
);

// TODO Again, this is only RSA algorithms. Luckily we do not have any other
// (for now).
alg_enum_for_at_least!(TPMI_ALG_RSA_DECRYPT, AlgRSADecrypt, [[asym, enc]], [Null]);

// Notation: [[asym, sign], [asym, meth]]
// TODO Again, this is only ECC algorithms. For now, hardcode.
alg_enum_for_at_least!(
    TPMI_ALG_ECC_SCHEME,
    AlgECCScheme,
    [],
    [ECDSA, ECDH, ECDAA, SM2, ECSCHNORR, ECMQV, Null]
);

alg_enum_for_at_least!(TPMI_ALG_PUBLIC, AlgPublic, [[obj]], [Null]);

/// TPMI_ECC_CURVE
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
