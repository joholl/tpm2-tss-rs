use crate::alg::{AlgHash, EccCurve};
use crate::bitfields::CommandCodeAttributes;
use crate::constants::{Capability, CommandCode};
use crate::enums::StructureTagAttest;
use crate::handles::Handle;
use crate::structs::{
    ACTData, AlgorithmProperty, CertifyInfo, ClockInfo, CommandAuditInfo, CreationInfo,
    NVCertifyInfo, NVDigestCertifyInfo, PCRSelection, QuoteInfo, SessionAuditInfo, TaggedPCRSelect,
    TaggedPolicy, TaggedProperty, TimeAttestInfo,
};
use crate::{constants::StructureTag, handles::Hierarchy};
use serde::{Deserialize, Serialize};

// TODO maybe use Vec<u8>? But then we would need to map AlgHash to the size.
/// TPMT_HA: TPMI_ALG_HASH (AlgHash), TPMU_HA
#[derive(Deserialize, Serialize, Debug, Clone, Default, PartialEq)]
#[repr(u16)]
#[serde(use_repr)]
pub enum Digest {
    Sha1([u8; 20]) = AlgHash::SHA1 as u16,
    Sha256([u8; 32]) = AlgHash::SHA256 as u16,
    Sha384([u8; 48]) = AlgHash::SHA384 as u16,
    Sha512([u8; 64]) = AlgHash::SHA512 as u16,
    Sm3_256([u8; 32]) = AlgHash::SM3_256 as u16,
    Sha3_256([u8; 32]) = AlgHash::SHA3_256 as u16,
    Sha3_384([u8; 48]) = AlgHash::SHA3_384 as u16,
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
        // TODO deserialize_with_u16_size
        digest: Vec<u8>,
    } = StructureTag::Creation as u16,
    /// TPMT_TK_VERIFIED
    Verified {
        hierarchy: Hierarchy,
        // TODO deserialize_with_u16_size
        digest: Vec<u8>,
    } = StructureTag::Verified as u16,
    /// TPMT_TK_AUTH
    AuthSecret {
        hierarchy: Hierarchy,
        // TODO deserialize_with_u16_size
        digest: Vec<u8>,
    } = StructureTag::AuthSecret as u16,
    Hashcheck {
        hierarchy: Hierarchy,
        // TODO deserialize_with_u16_size
        digest: Vec<u8>,
    } = StructureTag::Hashcheck as u16,
    /// TPMT_TK_AUTH
    AuthSigned {
        hierarchy: Hierarchy,
        // TODO deserialize_with_u16_size
        digest: Vec<u8>,
    } = StructureTag::AuthSigned as u16,
}

/// TPMS_CAPABILITY_DATA: TPM_CAP, TPMU_CAPABILITIES
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[repr(u16)]
pub enum Capabilities {
    Algorithms(
        // TODO deserialize_with_u32_size
        Vec<AlgorithmProperty>,
    ) = Capability::Algs as u16,
    Handles(
        // TODO deserialize_with_u32_size
        Vec<Handle>,
    ) = Capability::Handles as u16,
    Commands(
        // TODO deserialize_with_u32_size
        Vec<CommandCodeAttributes>,
    ) = Capability::Commands as u16,
    PPCommands(
        // TODO deserialize_with_u32_size
        Vec<CommandCode>,
    ) = Capability::PPCommands as u16,
    AuditCommands(
        // TODO deserialize_with_u32_size
        Vec<CommandCode>,
    ) = Capability::AuditCommands as u16,
    AssignedPCRs(
        // TODO deserialize_with_u32_size
        Vec<PCRSelection>,
    ) = Capability::Pcrs as u16,
    TpmProperties(
        // TODO deserialize_with_u32_size
        Vec<TaggedProperty>,
    ) = Capability::TpmProperties as u16,
    PcrProperties(
        // TODO deserialize_with_u32_size
        Vec<TaggedPCRSelect>,
    ) = Capability::PcrProperties as u16,
    EccCurves(
        // TODO deserialize_with_u32_size
        Vec<EccCurve>,
    ) = Capability::EccCurves as u16,
    AuthPolicies(
        // TODO deserialize_with_u32_size
        Vec<TaggedPolicy>,
    ) = Capability::AuthPolicies as u16,
    ACT(
        // TODO deserialize_with_u32_size
        Vec<ACTData>,
    ) = Capability::ACT as u16,
}

/// TPMS_ATTEST: TPMI_ST_ATTEST, TPMU_ATTEST
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[repr(u16)]
pub enum AttestBody {
    Certify {
        // TODO deserialize_with_u16_size
        qualified_signer: Vec<u8>,
        // TODO deserialize_with_u16_size
        extra_data: Vec<u8>,
        clock_info: ClockInfo,
        firmware_version: u64,
        attested: CertifyInfo,
    } = StructureTagAttest::Certify as u16,
    Creation {
        // TODO deserialize_with_u16_size
        qualified_signer: Vec<u8>,
        // TODO deserialize_with_u16_size
        extra_data: Vec<u8>,
        clock_info: ClockInfo,
        firmware_version: u64,
        attested: CreationInfo,
    } = StructureTagAttest::Creation as u16,
    Quote {
        // TODO deserialize_with_u16_size
        qualified_signer: Vec<u8>,
        // TODO deserialize_with_u16_size
        extra_data: Vec<u8>,
        clock_info: ClockInfo,
        firmware_version: u64,
        attested: QuoteInfo,
    } = StructureTagAttest::Quote as u16,
    CommandAudit {
        // TODO deserialize_with_u16_size
        qualified_signer: Vec<u8>,
        // TODO deserialize_with_u16_size
        extra_data: Vec<u8>,
        clock_info: ClockInfo,
        firmware_version: u64,
        attested: CommandAuditInfo,
    } = StructureTagAttest::CommandAudit as u16,
    SessionAudit {
        // TODO deserialize_with_u16_size
        qualified_signer: Vec<u8>,
        // TODO deserialize_with_u16_size
        extra_data: Vec<u8>,
        clock_info: ClockInfo,
        firmware_version: u64,
        attested: SessionAuditInfo,
    } = StructureTagAttest::SessionAudit as u16,
    Time {
        // TODO deserialize_with_u16_size
        qualified_signer: Vec<u8>,
        // TODO deserialize_with_u16_size
        extra_data: Vec<u8>,
        clock_info: ClockInfo,
        firmware_version: u64,
        attested: TimeAttestInfo,
    } = StructureTagAttest::Time as u16,
    NV {
        // TODO deserialize_with_u16_size
        qualified_signer: Vec<u8>,
        // TODO deserialize_with_u16_size
        extra_data: Vec<u8>,
        clock_info: ClockInfo,
        firmware_version: u64,
        attested: NVCertifyInfo,
    } = StructureTagAttest::Nv as u16,
    NvDigest {
        // TODO deserialize_with_u16_size
        qualified_signer: Vec<u8>,
        // TODO deserialize_with_u16_size
        extra_data: Vec<u8>,
        clock_info: ClockInfo,
        firmware_version: u64,
        attested: NVDigestCertifyInfo,
    } = StructureTagAttest::NvDigest as u16,
}
