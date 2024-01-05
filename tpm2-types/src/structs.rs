use crate::{
    alg::Alg,
    alg::AlgHash,
    bitfields::{ACTAttributes, AlgorithAttributes},
    constants::{PCRPropertyTag, PropertyTag, StructureTag},
    handles::Hierarchy,
    handles::Permanent,
    selectables::{AttestBody, Digest},
};
use serde::{Deserialize, Serialize};

/// TPMS_ALGORITHM_DESCRIPTION
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct AlgorithmDescription {
    pub alg: Alg,
    pub attributes: AlgorithAttributes,
}

// TPMS_PCR_SELECT is deserialize_with_u8_size Vec<u8>

/// TPMS_PCR_SELECTION
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct PCRSelection {
    pub hash: AlgHash,
    // TODO deserialize_with_u8_size
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
    // TODO deserialize_with_u16_size
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
    // TODO deserialize_with_u16_size
    pub name: Vec<u8>,
    // TODO deserialize_with_u16_size
    pub qualified_name: Vec<u8>,
}

/// TPMS_QUOTE_INFO
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct QuoteInfo {
    // TODO deserialize_with_u32_size
    pub pcr_select: Vec<PCRSelection>,
    // TODO deserialize_with_u16_size
    pub pcr_digest: Vec<u8>,
}

/// TPMS_COMMAND_AUDIT_INFO
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct CommandAuditInfo {
    pub audit_counter: u64,
    pub digest_alg: AlgHash,
    // TODO deserialize_with_u16_size
    pub audit_digest: Vec<u8>,
    // TODO deserialize_with_u16_size
    pub command_digest: Vec<u8>,
}

/// TPMS_SESSION_AUDIT_INFO
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct SessionAuditInfo {
    pub exclusive_session: bool,
    // TODO deserialize_with_u16_size
    pub session_digest: Vec<u8>,
}

/// TPMS_CREATION_INFO
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct CreationInfo {
    // TODO deserialize_with_u16_size
    pub object_name: Vec<u8>,
    // TODO deserialize_with_u16_size
    pub creation_hash: Vec<u8>,
}

/// TPMS_NV_CERTIFY_INFO
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct NVCertifyInfo {
    // TODO deserialize_with_u16_size
    pub index_name: Vec<u8>,
    pub offset: u16,
    // TODO deserialize_with_u16_size
    pub nv_contents: Vec<u8>,
}

/// TPMS_NV_DIGEST_CERTIFY_INFO
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct NVDigestCertifyInfo {
    // TODO deserialize_with_u16_size
    pub index_name: Vec<u8>,
    // TODO deserialize_with_u16_size
    pub nv_digest: Vec<u8>,
}

/// TPMS_NV_DIGEST_CERTIFY_INFO
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct Attest {
    // TODO make handle single value constant a type
    pub magic: TPM_GENERATED,
    pub body: AttestBody,
}
