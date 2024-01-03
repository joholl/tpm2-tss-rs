use serde_repr::{Deserialize_repr, Serialize_repr};
use tpm2_types_alg::{alg_enum_for_at_least, alg_enum_for_exactly};

// Spec Notation:
//  * !ALG.AX and !ALG.AE: alg_enum_for_exactly!(AlgOnlySymSign, [[asym, sign], [asym, enc]], [Null]);
//  * !ALG.ax and !ALG.ax: alg_enum_for_at_least!(AlgSymSign, [[asym, sign], [asym, enc]], [Null]);

// TPMI_ALG_HASH
alg_enum_for_exactly!(AlgHash, [[hash]], [Null]);

// TPMI_ALG_ASYM
alg_enum_for_exactly!(AlgAsym, [[asym, obj]], [Null]);

// TPMI_ALG_SYM
alg_enum_for_exactly!(AlgSym, [[sym]], [Null, XOR]);

// TPMI_ALG_SYM_OBJECT
alg_enum_for_exactly!(AlgSymObj, [[sym]], [Null]);

// TPMI_ALG_SYM_MODE
alg_enum_for_exactly!(AlgSymMode, [[sym, enc], [sym, sign]], [Null]);

// TPMI_ALG_KDF
alg_enum_for_exactly!(AlgKdf, [[hash, meth]], [Null]);

// TPMI_ALG_SIG_SCHEME
alg_enum_for_at_least!(AlgSigScheme, [[asym, sign]], [Null, HMAC]);

// TPMI_ECC_KEY_EXCHANGE
alg_enum_for_exactly!(AlgEccKeyEchange, [[asym, meth]], [Null, SM2]);

// TPMI_ALG_MAC_SCHEME
alg_enum_for_exactly!(AlgMacScheme, [[sym, sign], [hash]], [Null]);

// TPMI_ALG_CIPHER_MODE
alg_enum_for_exactly!(AlgCipherMode, [[sym, enc]], [Null]);

// TPMI_ALG_KEYEDHASH_SCHEME
alg_enum_for_at_least!(AlgKeyedHashScheme, [], [Null, HMAC, XOR]);

// TPMI_ALG_ASYM_SCHEME
alg_enum_for_at_least!(
    AlgAsymScheme,
    [[asym, meth], [asym, sign], [asym, enc]],
    [Null]
);

// TPMI_ALG_RSA_SCHEME
// TODO weird spec notation: TPM_ALG_!ALG.ae.ax
alg_enum_for_at_least!(AlgRSAScheme, [[asym, enc], [asym, sign]], [Null]);

// TPMI_ALG_RSA_DECRYPT
alg_enum_for_at_least!(AlgRSADecrypt, [[asym, enc]], [Null]);

// TPMI_ALG_ECC_SCHEME
alg_enum_for_at_least!(AlgEccScheme, [[asym, sign], [asym, meth]], [Null]);

// TPMI_ALG_PUBLIC
alg_enum_for_at_least!(AlgPublic, [[obj]], [Null]);

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
