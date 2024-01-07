use crate::alg::{
    AlgAsymScheme, AlgECCScheme, AlgKdf, AlgKeyedHashScheme, AlgPublic, AlgRSAScheme, AlgSigScheme,
    AlgSym, AlgSymMode, AlgSymObj,
};
use crate::alg::{AlgHash, EccCurve};
use crate::bitfields::{CommandCodeAttributes, ObjectAttributes};
use crate::constants::{Capability, CommandCode};
use crate::enums::{AESKeyBits, CAMELLIAKeyBits, SM4KeyBits, StructureTagAttest, TDESKeyBits};
use crate::handles::Handle;
use crate::serde_types::big_array::BigArray;
use crate::serde_types::de::{deserialize_u16_sized_vec, deserialize_u32_sized_vec};
use crate::structs::{
    ACTData, AlgorithmProperty, CertifyInfo, ClockInfo, CommandAuditInfo, CreationInfo, ECCParams,
    EccPoint, EncSchemeOAEP, KeySchemeECDH, KeySchemeECMQ, KeyedHashParams, NVCertifyInfo,
    NVDigestCertifyInfo, PCRSelection, QuoteInfo, RSAParams, SchemeHMAC, SchemeKDF2,
    SchemeKdf1Sp800_108, SchemeKdf1Sp800_56a, SchemeMGF1, SchemeXOR, SessionAuditInfo,
    SigSchemeECDAA, SigSchemeECDSA, SigSchemeECSCHNORR, SigSchemeRSAPSS, SigSchemeRSASSA,
    SigSchemeSM2, SignatureECDAA, SignatureECDSA, SignatureECSCHNORR, SignatureRSAPSS,
    SignatureRSASSA, SignatureSM2, TaggedPCRSelect, TaggedPolicy, TaggedProperty, TimeAttestInfo,
};
use crate::{constants::StructureTag, handles::Hierarchy};
use serde::{self, Deserialize, Serialize};

// TODO maybe use Vec<u8>? But then we would need to map AlgHash to the size.
/// TPMT_HA: TPMI_ALG_HASH (AlgHash), TPMU_HA
#[derive(Deserialize, Serialize, Debug, Clone, Default, PartialEq)]
#[repr(u16)]
pub enum Digest {
    Sha1([u8; 20]) = AlgHash::SHA1 as u16,
    Sha256([u8; 32]) = AlgHash::SHA256 as u16,
    #[serde(with = "BigArray")]
    Sha384([u8; 48]) = AlgHash::SHA384 as u16,
    #[serde(with = "BigArray")]
    Sha512([u8; 64]) = AlgHash::SHA512 as u16,
    Sm3_256([u8; 32]) = AlgHash::SM3_256 as u16,
    Sha3_256([u8; 32]) = AlgHash::SHA3_256 as u16,
    #[serde(with = "BigArray")]
    Sha3_384([u8; 48]) = AlgHash::SHA3_384 as u16,
    #[serde(with = "BigArray")]
    Sha3_512([u8; 64]) = AlgHash::SHA3_512 as u16,
    #[default]
    Null = AlgHash::Null as u16,
}

/// TPMT_TK
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[repr(u16)]
pub enum Ticket {
    /// TPMT_TK_CREATION
    Creation {
        hierarchy: Hierarchy,
        #[serde(deserialize_with = "deserialize_u16_sized_vec")]
        digest: Vec<u8>,
    } = StructureTag::Creation as u16,
    /// TPMT_TK_VERIFIED
    Verified {
        hierarchy: Hierarchy,
        #[serde(deserialize_with = "deserialize_u16_sized_vec")]
        digest: Vec<u8>,
    } = StructureTag::Verified as u16,
    /// TPMT_TK_AUTH
    AuthSecret {
        hierarchy: Hierarchy,
        #[serde(deserialize_with = "deserialize_u16_sized_vec")]
        digest: Vec<u8>,
    } = StructureTag::AuthSecret as u16,
    Hashcheck {
        hierarchy: Hierarchy,
        #[serde(deserialize_with = "deserialize_u16_sized_vec")]
        digest: Vec<u8>,
    } = StructureTag::Hashcheck as u16,
    /// TPMT_TK_AUTH
    AuthSigned {
        hierarchy: Hierarchy,
        #[serde(deserialize_with = "deserialize_u16_sized_vec")]
        digest: Vec<u8>,
    } = StructureTag::AuthSigned as u16,
}

/// TPMS_CAPABILITY_DATA: TPM_CAP, TPMU_CAPABILITIES
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[repr(u16)]
pub enum Capabilities {
    Algorithms(#[serde(deserialize_with = "deserialize_u32_sized_vec")] Vec<AlgorithmProperty>) =
        Capability::Algs as u16,
    Handles(#[serde(deserialize_with = "deserialize_u32_sized_vec")] Vec<Handle>) =
        Capability::Handles as u16,
    Commands(#[serde(deserialize_with = "deserialize_u32_sized_vec")] Vec<CommandCodeAttributes>) =
        Capability::Commands as u16,
    PPCommands(#[serde(deserialize_with = "deserialize_u32_sized_vec")] Vec<CommandCode>) =
        Capability::PPCommands as u16,
    AuditCommands(#[serde(deserialize_with = "deserialize_u32_sized_vec")] Vec<CommandCode>) =
        Capability::AuditCommands as u16,
    AssignedPCRs(#[serde(deserialize_with = "deserialize_u32_sized_vec")] Vec<PCRSelection>) =
        Capability::Pcrs as u16,
    TpmProperties(#[serde(deserialize_with = "deserialize_u32_sized_vec")] Vec<TaggedProperty>) =
        Capability::TpmProperties as u16,
    PcrProperties(#[serde(deserialize_with = "deserialize_u32_sized_vec")] Vec<TaggedPCRSelect>) =
        Capability::PcrProperties as u16,
    EccCurves(#[serde(deserialize_with = "deserialize_u32_sized_vec")] Vec<EccCurve>) =
        Capability::EccCurves as u16,
    AuthPolicies(#[serde(deserialize_with = "deserialize_u32_sized_vec")] Vec<TaggedPolicy>) =
        Capability::AuthPolicies as u16,
    ACT(#[serde(deserialize_with = "deserialize_u32_sized_vec")] Vec<ACTData>) =
        Capability::ACT as u16,
}

/// TPMS_ATTEST: TPMI_ST_ATTEST, TPMU_ATTEST
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[repr(u16)]
pub enum AttestBody {
    Certify {
        #[serde(deserialize_with = "deserialize_u16_sized_vec")]
        qualified_signer: Vec<u8>,
        #[serde(deserialize_with = "deserialize_u16_sized_vec")]
        extra_data: Vec<u8>,
        clock_info: ClockInfo,
        firmware_version: u64,
        attested: CertifyInfo,
    } = StructureTagAttest::AttestCertify as u16,
    Creation {
        #[serde(deserialize_with = "deserialize_u16_sized_vec")]
        qualified_signer: Vec<u8>,
        #[serde(deserialize_with = "deserialize_u16_sized_vec")]
        extra_data: Vec<u8>,
        clock_info: ClockInfo,
        firmware_version: u64,
        attested: CreationInfo,
    } = StructureTagAttest::AttestCreation as u16,
    Quote {
        #[serde(deserialize_with = "deserialize_u16_sized_vec")]
        qualified_signer: Vec<u8>,
        #[serde(deserialize_with = "deserialize_u16_sized_vec")]
        extra_data: Vec<u8>,
        clock_info: ClockInfo,
        firmware_version: u64,
        attested: QuoteInfo,
    } = StructureTagAttest::AttestQuote as u16,
    CommandAudit {
        #[serde(deserialize_with = "deserialize_u16_sized_vec")]
        qualified_signer: Vec<u8>,
        #[serde(deserialize_with = "deserialize_u16_sized_vec")]
        extra_data: Vec<u8>,
        clock_info: ClockInfo,
        firmware_version: u64,
        attested: CommandAuditInfo,
    } = StructureTagAttest::AttestCommandAudit as u16,
    SessionAudit {
        #[serde(deserialize_with = "deserialize_u16_sized_vec")]
        qualified_signer: Vec<u8>,
        #[serde(deserialize_with = "deserialize_u16_sized_vec")]
        extra_data: Vec<u8>,
        clock_info: ClockInfo,
        firmware_version: u64,
        attested: SessionAuditInfo,
    } = StructureTagAttest::AttestSessionAudit as u16,
    Time {
        #[serde(deserialize_with = "deserialize_u16_sized_vec")]
        qualified_signer: Vec<u8>,
        #[serde(deserialize_with = "deserialize_u16_sized_vec")]
        extra_data: Vec<u8>,
        clock_info: ClockInfo,
        firmware_version: u64,
        attested: TimeAttestInfo,
    } = StructureTagAttest::AttestTime as u16,
    NV {
        #[serde(deserialize_with = "deserialize_u16_sized_vec")]
        qualified_signer: Vec<u8>,
        #[serde(deserialize_with = "deserialize_u16_sized_vec")]
        extra_data: Vec<u8>,
        clock_info: ClockInfo,
        firmware_version: u64,
        attested: NVCertifyInfo,
    } = StructureTagAttest::AttestNV as u16,
    NVDigest {
        #[serde(deserialize_with = "deserialize_u16_sized_vec")]
        qualified_signer: Vec<u8>,
        #[serde(deserialize_with = "deserialize_u16_sized_vec")]
        extra_data: Vec<u8>,
        clock_info: ClockInfo,
        firmware_version: u64,
        attested: NVDigestCertifyInfo,
    } = StructureTagAttest::AttestNVDigest as u16,
}

/// TPMT_SYM_DEF: TPMI_ALG_SYM, TPMU_SYM_KEY_BITS, TPMU_SYM_MODE, TPMU_SYM_DETAILS
#[derive(Deserialize, Serialize, Debug, Clone, Default, PartialEq)]
#[repr(u16)]
pub enum SymDef {
    TDES {
        key_bits: TDESKeyBits,
        mode: AlgSymMode,
    } = AlgSym::TDES as u16,
    AES {
        key_bits: AESKeyBits,
        mode: AlgSymMode,
    } = AlgSym::AES as u16,
    XOR {
        key_bits: AlgHash,
    } = AlgSym::XOR as u16,
    SM4 {
        key_bits: SM4KeyBits,
        mode: AlgSymMode,
    } = AlgSym::SM4 as u16,
    Camellia {
        key_bits: CAMELLIAKeyBits,
        mode: AlgSymMode,
    } = AlgSym::CAMELLIA as u16,
    #[default]
    Null = AlgSym::Null as u16,
}

/// TPMT_SYM_DEF_OBJECT: TPMI_ALG_SYM_OBJECT, TPMU_SYM_DEF_OBJECT
#[derive(Deserialize, Serialize, Debug, Clone, Default, PartialEq)]
#[repr(u16)]
pub enum SymDefObject {
    TDES {
        key_bits: TDESKeyBits,
        mode: AlgSymMode,
    } = AlgSymObj::TDES as u16,
    AES {
        key_bits: AESKeyBits,
        mode: AlgSymMode,
    } = AlgSymObj::AES as u16,
    SM4 {
        key_bits: SM4KeyBits,
        mode: AlgSymMode,
    } = AlgSymObj::SM4 as u16,
    Camellia {
        key_bits: CAMELLIAKeyBits,
        mode: AlgSymMode,
    } = AlgSymObj::CAMELLIA as u16,
    #[default]
    Null = AlgSymObj::Null as u16,
}

/// TPMS_SYMCIPHER_PARMS
pub type SymCipherParams = SymDefObject;

// TPMU_SENSITIVE_CREATE is either TPM2B_SENSITVE_DATA or a TPM2B_DERIVE, two
// opaque data blobs. There is no selector, but selection is done "based on
// context". Skipping this. *sigh*

/// TPMT_KEYEDHASH_SCHEME: TPMI_ALG_KEYEDHASH_SCHEME, TPMU_SCHEME_KEYEDHASH
#[derive(Deserialize, Serialize, Debug, Clone, Default, PartialEq)]
#[repr(u16)]
pub enum KeyedHashScheme {
    HMAC(SchemeHMAC) = AlgKeyedHashScheme::HMAC as u16,
    XOR(SchemeXOR) = AlgKeyedHashScheme::XOR as u16,
    #[default]
    Null = AlgSymObj::Null as u16,
}

/// TPMT_SIG_SCHEME: TPMI_ALG_SIG_SCHEME, TPMU_SIG_SCHEME
#[derive(Deserialize, Serialize, Debug, Clone, Default, PartialEq)]
#[repr(u16)]
pub enum SigScheme {
    HMAC(SchemeHMAC) = AlgSigScheme::HMAC as u16,
    RSASSA(SigSchemeRSASSA) = AlgSigScheme::RSASSA as u16,
    RSAPSS(SigSchemeRSAPSS) = AlgSigScheme::RSAPSS as u16,
    ECDSA(SigSchemeECDSA) = AlgSigScheme::ECDSA as u16,
    ECDAA(SigSchemeECDAA) = AlgSigScheme::ECDAA as u16,
    SM2(SigSchemeSM2) = AlgSigScheme::SM2 as u16,
    ECSCHNORR(SigSchemeECSCHNORR) = AlgSigScheme::ECSCHNORR as u16,

    #[default]
    Null = AlgSymObj::Null as u16,
}

/// TPMT_KDF_SCHEME: TPMI_ALG_KDF, TPMU_KDF_SCHEME
#[derive(Deserialize, Serialize, Debug, Clone, Default, PartialEq)]
#[repr(u16)]
pub enum KdfScheme {
    MGF1(SchemeMGF1) = AlgKdf::MGF1 as u16,
    Kdf1Sp800_56a(SchemeKdf1Sp800_56a) = AlgKdf::Kdf1Sp800_56a as u16,
    KDF2(SchemeKDF2) = AlgKdf::KDF2 as u16,
    Kdf1Sp800_108(SchemeKdf1Sp800_108) = AlgKdf::Kdf1Sp800_108 as u16,

    #[default]
    Null = AlgSymObj::Null as u16,
}

/// TPMT_ASYM_SCHEME: TPMI_ALG_ASYM_SCHEME, TPMU_ASYM_SCHEME
#[derive(Deserialize, Serialize, Debug, Default, Clone, PartialEq)]
#[repr(u16)]
pub enum AsymScheme {
    RSASSA(SigSchemeRSASSA) = AlgAsymScheme::RSASSA as u16,
    RSAES = AlgAsymScheme::RSAES as u16,
    RSAPSS(SigSchemeRSAPSS) = AlgAsymScheme::RSAPSS as u16,
    OAEP(EncSchemeOAEP) = AlgAsymScheme::OAEP as u16,
    ECDSA(SigSchemeECDSA) = AlgAsymScheme::ECDSA as u16,
    ECDH(KeySchemeECDH) = AlgAsymScheme::ECDH as u16,
    ECDAA(SigSchemeECDAA) = AlgAsymScheme::ECDAA as u16,
    SM2(SigSchemeSM2) = AlgAsymScheme::SM2 as u16,
    ECSchnorr(SigSchemeECSCHNORR) = AlgAsymScheme::ECSCHNORR as u16,
    ECMQV(KeySchemeECMQ) = AlgAsymScheme::ECMQV as u16,
    #[default]
    Null = AlgAsymScheme::Null as u16,
}

/// TPMT_RSA_SCHEME, TPMI_ALG_RSA_SCHEME, TPMU_ASYM_SCHEME
#[derive(Deserialize, Serialize, Debug, Default, Clone, PartialEq)]
#[repr(u16)]
pub enum RSAScheme {
    RSASSA(SigSchemeRSASSA) = AlgRSAScheme::RSASSA as u16,
    RSAES = AlgRSAScheme::RSAES as u16,
    RSAPSS(SigSchemeRSAPSS) = AlgRSAScheme::RSAPSS as u16,
    #[default]
    Null = AlgRSAScheme::Null as u16,
}

// TODO if this is really just empty, skip it
/// TPMT_RSA_DECRYPT, TPMI_ALG_RSA_DECRYPT, TPMU_ASYM_SCHEME
#[derive(Deserialize, Serialize, Debug, Default, Clone, PartialEq)]
#[repr(u16)]
pub enum RSADecrypt {
    RSAES = AlgRSAScheme::RSAES as u16,
    #[default]
    Null = AlgRSAScheme::Null as u16,
}

/// TPMT_ECC_SCHEME, TPMI_ALG_ECC_SCHEME, TPMU_ASYM_SCHEME
#[derive(Deserialize, Serialize, Debug, Default, Clone, PartialEq)]
#[repr(u16)]
pub enum EccScheme {
    ECDSA(SigSchemeECDSA) = AlgECCScheme::ECDSA as u16,
    ECDAA(SigSchemeECDAA) = AlgECCScheme::ECDAA as u16,
    SM2(SigSchemeSM2) = AlgECCScheme::SM2 as u16,
    ECSCHNORR(SigSchemeECSCHNORR) = AlgECCScheme::ECSCHNORR as u16,
    ECDH(KeySchemeECDH) = AlgECCScheme::ECDH as u16,
    #[default]
    Null = AlgECCScheme::Null as u16,
}

/// TPMT_SIGNATURE, TPMI_ALG_SIG_SCHEME, TPMU_SIGNATURE
#[derive(Deserialize, Serialize, Debug, Default, Clone, PartialEq)]
#[repr(u16)]
pub enum Signature {
    HMAC(Digest) = AlgSigScheme::HMAC as u16,
    RSASSA(SignatureRSASSA) = AlgSigScheme::RSASSA as u16,
    RSAPSS(SignatureRSAPSS) = AlgSigScheme::RSAPSS as u16,
    ECDSA(SignatureECDSA) = AlgSigScheme::ECDSA as u16,
    ECDAA(SignatureECDAA) = AlgSigScheme::ECDAA as u16,
    SM2(SignatureSM2) = AlgSigScheme::SM2 as u16,
    ECSCHNORR(SignatureECSCHNORR) = AlgSigScheme::ECSCHNORR as u16,
    #[default]
    Null = AlgSigScheme::Null as u16,
}

// TPMU_ENCRYPTED_SECRET is just a byte blob

/// TPMT_PUBLIC_PARMS: TPMI_ALG_PUBLIC, TPMU_PUBLIC_PARMS
#[derive(Deserialize, Debug, Serialize, Clone, PartialEq)]
#[repr(u16)]
pub enum PublicParams {
    KeyedHash(KeyedHashParams) = AlgPublic::KeyedHash as u16,
    SymCipher(SymCipherParams) = AlgPublic::SymCipher as u16,
    RSA(RSAParams) = AlgPublic::RSA as u16,
    ECC(ECCParams) = AlgPublic::ECC as u16,
}

/// TPMT_PUBLIC: TPMI_ALG_PUBLIC, TPMU_PUBLIC_ID
#[derive(Deserialize, Debug, Serialize, Clone, PartialEq)]
#[repr(u16)]
pub enum Public {
    KeyedHash {
        name_alg: AlgHash,
        object_attributes: ObjectAttributes,
        #[serde(deserialize_with = "deserialize_u16_sized_vec")]
        auth_policy: Vec<u8>,
        parameters: KeyedHashParams,
        #[serde(deserialize_with = "deserialize_u16_sized_vec")]
        unique: Vec<u8>,
    } = AlgPublic::KeyedHash as u16,
    SymCipher {
        name_alg: AlgHash,
        object_attributes: ObjectAttributes,
        #[serde(deserialize_with = "deserialize_u16_sized_vec")]
        auth_policy: Vec<u8>,
        parameters: SymCipherParams,
        #[serde(deserialize_with = "deserialize_u16_sized_vec")]
        unique: Vec<u8>,
    } = AlgPublic::SymCipher as u16,
    RSA {
        name_alg: AlgHash,
        object_attributes: ObjectAttributes,
        #[serde(deserialize_with = "deserialize_u16_sized_vec")]
        auth_policy: Vec<u8>,
        parameters: RSAParams,
        #[serde(deserialize_with = "deserialize_u16_sized_vec")]
        unique: Vec<u8>,
    } = AlgPublic::RSA as u16,
    ECC {
        name_alg: AlgHash,
        object_attributes: ObjectAttributes,
        #[serde(deserialize_with = "deserialize_u16_sized_vec")]
        auth_policy: Vec<u8>,
        parameters: ECCParams,
        unique: EccPoint,
    } = AlgPublic::ECC as u16,
}

// TODO all variants are the same... do we unify this?
/// TPMT_SENSITIVE: TPMI_ALG_PUBLIC, TPMU_SENSITIVE_COMPOSITE
#[derive(Deserialize, Debug, Serialize, Clone, PartialEq)]
#[repr(u16)]
pub enum Sensitive {
    KeyedHash {
        #[serde(deserialize_with = "deserialize_u16_sized_vec")]
        auth_value: Vec<u8>,
        #[serde(deserialize_with = "deserialize_u16_sized_vec")]
        seed_value: Vec<u8>,
        #[serde(deserialize_with = "deserialize_u16_sized_vec")]
        sensitive: Vec<u8>,
    } = AlgPublic::KeyedHash as u16,
    SymCipher {
        #[serde(deserialize_with = "deserialize_u16_sized_vec")]
        auth_value: Vec<u8>,
        #[serde(deserialize_with = "deserialize_u16_sized_vec")]
        seed_value: Vec<u8>,
        #[serde(deserialize_with = "deserialize_u16_sized_vec")]
        sensitive: Vec<u8>,
    } = AlgPublic::SymCipher as u16,
    RSA {
        #[serde(deserialize_with = "deserialize_u16_sized_vec")]
        auth_value: Vec<u8>,
        #[serde(deserialize_with = "deserialize_u16_sized_vec")]
        seed_value: Vec<u8>,
        #[serde(deserialize_with = "deserialize_u16_sized_vec")]
        sensitive: Vec<u8>,
    } = AlgPublic::RSA as u16,
    ECC {
        #[serde(deserialize_with = "deserialize_u16_sized_vec")]
        auth_value: Vec<u8>,
        #[serde(deserialize_with = "deserialize_u16_sized_vec")]
        seed_value: Vec<u8>,
        #[serde(deserialize_with = "deserialize_u16_sized_vec")]
        sensitive: Vec<u8>,
    } = AlgPublic::ECC as u16,
}

// TODO have some macro which verifies that all variants are handled?
