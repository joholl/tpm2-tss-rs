use crate::{
    alg::{Alg, AlgHash, AlgKdf, EccCurve},
    bitfields::{
        ACTAttributes, AlgorithAttributes, LocalityAttributes, NVAttributes, SessionAttributes,
    },
    constants::{AttachedComponentTag, PCRPropertyTag, PropertyTag, GENERATED},
    enums::RSAKeyBits,
    handles::Hierarchy,
    handles::{AuthSession, NVIndex, Permanent, Saved},
    selectables::{
        AsymScheme, AttestBody, Digest, EccScheme, KdfScheme, KeyedHashScheme, RSAScheme,
        SymDefObject,
    },
    serde_types::sized_vector::{U16SizedVector, U8SizedVector},
};
use serde::{Deserialize, Serialize};

/// TPMS_ALGORITHM_DESCRIPTION
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct AlgorithmDescription {
    pub alg: Alg,
    pub attributes: AlgorithAttributes,
}

// TPMS_PCR_SELECT is #[serde(with = "U8SizedVector")] Vec<u8>

/// TPMS_PCR_SELECTION
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct PCRSelection {
    pub hash: AlgHash,
    #[serde(with = "U8SizedVector")]
    pub pcr_select: Vec<u8>,
}

/// TPMS_ALG_PROPERTY
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct AlgorithmProperty {
    pub alg: Alg,
    pub attributes: AlgorithAttributes,
}

/// TPMS_TAGGED_PROPERTY
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct TaggedProperty {
    pub property: PropertyTag,
    pub value: u32,
}

/// TPMS_TAGGED_PCR_SELECT
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct TaggedPCRSelect {
    pub tag: PCRPropertyTag,
    #[serde(with = "U8SizedVector")]
    pub pcr_select: Vec<u8>,
}

/// TPMS_TAGGED_POLICY
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct TaggedPolicy {
    pub handle: Permanent,
    pub policy_hash: Digest,
}

/// TPMS_ACT_DATA
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct ACTData {
    pub handle: Permanent,
    pub timeout: u32,
    pub attributes: ACTAttributes,
}

/// TPMS_CLOCK_INFO
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct ClockInfo {
    pub clock: u64,
    pub reset_count: u32,
    pub restart_count: u32,
    pub safe: bool,
}

/// TPMS_TIME_INFO
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct TimeInfo {
    pub time: u64,
    pub clock_info: ClockInfo,
}

/// TPMS_TIME_ATTEST_INFO
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct TimeAttestInfo {
    pub time: TimeInfo,
    pub firmware_version: u64,
}

/// TPMS_CERTIFY_INFO
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct CertifyInfo {
    #[serde(with = "U16SizedVector")]
    pub name: Vec<u8>,
    #[serde(with = "U16SizedVector")]
    pub qualified_name: Vec<u8>,
}

/// TPMS_QUOTE_INFO
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct QuoteInfo {
    // TODO deserialize_with_u32_size
    pub pcr_select: Vec<PCRSelection>,
    #[serde(with = "U16SizedVector")]
    pub pcr_digest: Vec<u8>,
}

/// TPMS_COMMAND_AUDIT_INFO
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct CommandAuditInfo {
    pub audit_counter: u64,
    pub digest_alg: AlgHash,
    #[serde(with = "U16SizedVector")]
    pub audit_digest: Vec<u8>,
    #[serde(with = "U16SizedVector")]
    pub command_digest: Vec<u8>,
}

/// TPMS_SESSION_AUDIT_INFO
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct SessionAuditInfo {
    pub exclusive_session: bool,
    #[serde(with = "U16SizedVector")]
    pub session_digest: Vec<u8>,
}

/// TPMS_CREATION_INFO
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct CreationInfo {
    #[serde(with = "U16SizedVector")]
    pub object_name: Vec<u8>,
    #[serde(with = "U16SizedVector")]
    pub creation_hash: Vec<u8>,
}

/// TPMS_NV_CERTIFY_INFO
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct NVCertifyInfo {
    #[serde(with = "U16SizedVector")]
    pub index_name: Vec<u8>,
    pub offset: u16,
    #[serde(with = "U16SizedVector")]
    pub nv_contents: Vec<u8>,
}

/// TPMS_NV_DIGEST_CERTIFY_INFO
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct NVDigestCertifyInfo {
    #[serde(with = "U16SizedVector")]
    pub index_name: Vec<u8>,
    #[serde(with = "U16SizedVector")]
    pub nv_digest: Vec<u8>,
}

/// TPMS_NV_DIGEST_CERTIFY_INFO
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct Attest {
    pub magic: GENERATED,
    pub body: AttestBody,
}

/// TPMS_AUTH_COMMAND
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct AuthCommand {
    pub session_handle: AuthSession,
    #[serde(with = "U16SizedVector")]
    pub nonce: Vec<u8>,
    pub session_attributes: SessionAttributes,
    #[serde(with = "U16SizedVector")]
    pub hmac: Vec<u8>,
}

/// TPMS_AUTH_RESPONSE
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct AuthResponse {
    #[serde(with = "U16SizedVector")]
    pub nonce: Vec<u8>,
    pub session_attributes: SessionAttributes,
    #[serde(with = "U16SizedVector")]
    pub hmac: Vec<u8>,
}

/// TPMS_DERIVE
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct Derive {
    #[serde(with = "U16SizedVector")]
    pub label: Vec<u8>,
    #[serde(with = "U16SizedVector")]
    pub context: Vec<u8>,
}

/// TPMS_SENSITIVE_CREATE
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct SensitiveCreate {
    #[serde(with = "U16SizedVector")]
    pub user_auth: Vec<u8>,
    #[serde(with = "U16SizedVector")]
    pub data: Vec<u8>,
}

/// TPMS_SCHEME_HASH
pub type SchemeHash = AlgHash;

/// TPMS_SCHEME_ECDAA
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct SchemeECDAA {
    pub hash_alg: AlgHash,
    pub count: u16,
}

// TODO The heading of this type seems to be wrong
/// HMAC_SIG_SCHEME/TPMS_SCHEME_HASH
pub type SchemeHMAC = SchemeHash;

/// TPMS_SCHEME_XOR
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct SchemeXOR {
    pub hash_alg: AlgHash,
    pub kdf: AlgKdf,
}

/// TPMS_SIG_SCHEME_RSASSA
pub type SigSchemeRSASSA = SchemeHash;

/// TPMS_SIG_SCHEME_RSAPSSS
pub type SigSchemeRSAPSS = SchemeHash;

/// TPMS_SIG_SCHEME_ECDSA
pub type SigSchemeECDSA = SchemeHash;

/// TPMS_SIG_SCHEME_ECDAA
pub type SigSchemeECDAA = SchemeECDAA;

/// TPMS_SIG_SCHEME_ECSCHNORR
pub type SigSchemeECSCHNORR = SchemeHash;

// TODO technically not specified
/// TPMS_SIG_SCHEME_SM2
pub type SigSchemeSM2 = SchemeHash;

/// TPMS_ENC_SCHEME_OAEP
pub type EncSchemeOAEP = SchemeHash;

// TPMS_ENC_SCHEME_RSAES is empty

/// TPMS_KEY_SCHEME_ECDH
pub type KeySchemeECDH = SchemeHash;

/// TPMS_KEY_SCHEME_ECMQV
pub type KeySchemeECMQ = SchemeHash;

/// TPMS_SCHEME_MGF1
pub type SchemeMGF1 = SchemeHash;

/// TPMS_SCHEME_KDF1SP800_56A
pub type SchemeKdf1Sp800_56a = SchemeHash;

/// TPMS_SCHEME_KDF2
pub type SchemeKDF2 = SchemeHash;

/// TPMS_SCHEME_KDF1SP800_108
pub type SchemeKdf1Sp800_108 = SchemeHash;

/// TPMS_ECC_POINT
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct EccPoint {
    #[serde(with = "U16SizedVector")]
    pub x: Vec<u8>,
    #[serde(with = "U16SizedVector")]
    pub y: Vec<u8>,
}

/// TPMS_ALGORITHM_DETAIL_ECC
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct AlgortihmDetailECC {
    pub curve_id: EccCurve,
    pub key_size: u16,
    pub kdf: KdfScheme,
    pub sign: EccScheme,
    #[serde(with = "U16SizedVector")]
    pub p: Vec<u8>,
    #[serde(with = "U16SizedVector")]
    pub a: Vec<u8>,
    #[serde(with = "U16SizedVector")]
    pub b: Vec<u8>,
    #[serde(with = "U16SizedVector")]
    pub g_x: Vec<u8>,
    #[serde(with = "U16SizedVector")]
    pub g_y: Vec<u8>,
    #[serde(with = "U16SizedVector")]
    pub n: Vec<u8>,
    #[serde(with = "U16SizedVector")]
    pub h: Vec<u8>,
}

/// TPMS_SIGNATURE_RSA
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct SignatureRSA {
    pub hash: AlgHash,
    #[serde(with = "U16SizedVector")]
    pub sig: Vec<u8>,
}

/// TPMS_SIGNATURE_RSASSA
pub type SignatureRSASSA = SignatureRSA;

/// TPMS_SIGNATURE_RSAPSS
pub type SignatureRSAPSS = SignatureRSA;

/// TPMS_SIGNATURE_ECC
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct SignatureECC {
    pub hash: AlgHash,
    #[serde(with = "U16SizedVector")]
    pub signature_r: Vec<u8>,
    #[serde(with = "U16SizedVector")]
    pub signature_s: Vec<u8>,
}

/// TPMS_SIGNATURE_ECDSA
pub type SignatureECDSA = SignatureECC;

/// TPMS_SIGNATURE_ECDAA
pub type SignatureECDAA = SignatureECC;

/// TPMS_SIGNATURE_SM2
pub type SignatureSM2 = SignatureECC;

/// TPMS_SIGNATURE_ECSCHNORR
pub type SignatureECSCHNORR = SignatureECC;

/// TPMS_KEYEDHASH_PARMS
pub type KeyedHashParams = KeyedHashScheme;

/// TPMS_KEYEDHASH_PARMS
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct AsymParams {
    pub symmetric: SymDefObject,
    pub scheme: AsymScheme,
}

/// TPMS_RSA_PARMS
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct RSAParams {
    pub symmetric: SymDefObject,
    pub scheme: RSAScheme,
    pub key_bits: RSAKeyBits,
    pub exponent: u32,
}

/// TPMS_ECC_PARMS
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct ECCParams {
    pub symmetric: SymDefObject,
    pub scheme: EccScheme,
    pub curve_id: EccCurve,
    pub kdf: KdfScheme,
}

/// TPMS_ID_OBJECT
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct IdObject {
    #[serde(with = "U16SizedVector")]
    pub integrity_hmac: Vec<u8>,
    #[serde(with = "U16SizedVector")]
    pub enc_identity: Vec<u8>,
}

/// TPMS_NV_PIN_COUNTER_PARAMETERS
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct NVCounterParameters {
    pub pin_count: u32,
    pub pin_limit: u32,
}

/// TPMS_NV_PUBLIC
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct NVPublic {
    pub nv_index: NVIndex,
    pub name_alg: AlgHash,
    pub attributes: NVAttributes,
    #[serde(with = "U16SizedVector")]
    pub auth_policy: Vec<u8>,
    pub data_size: u16,
}

/// TPMS_CONTEXT_DATA
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct ContextData {
    #[serde(with = "U16SizedVector")]
    pub integrity: Vec<u8>,
    #[serde(with = "U16SizedVector")]
    pub encrypted: Vec<u8>,
}

/// TPMS_CONTEXT
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct Context {
    pub sequence: u64,
    pub saved_handle: Saved,
    pub hierarchy: Hierarchy,
    #[serde(with = "U16SizedVector")]
    pub context_blob: Vec<u8>,
}

/// TPMS_CREATION_DATA
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct CreationData {
    pub pcr_select: PCRPropertyTag,
    #[serde(with = "U16SizedVector")]
    pub digest: Vec<u8>,
    pub locality: LocalityAttributes,
    pub parent_name_alg: AlgHash,
    #[serde(with = "U16SizedVector")]
    pub parent_name: Vec<u8>,
    #[serde(with = "U16SizedVector")]
    pub parent_qualified_name: Vec<u8>,
    #[serde(with = "U16SizedVector")]
    pub outside_info: Vec<u8>,
}

/// TPMS_AC_OUTPUT
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct AttachedComponentOutput {
    pub tag: AttachedComponentTag,
    pub data: u32,
}
