pub mod alg;
pub mod bitfields;
pub mod constants;
pub mod enums;
pub mod handles;
pub mod selectables;
pub mod serde_types;
pub mod structs;
pub mod util;

pub mod types {
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
    // type TPML_AC_CAPABILITIES = Vec<TPMS_AC_OUTPUT>; // count: u32
}

#[test]
fn normal_types() {
    use crate::alg::{
        Alg, AlgAsym, AlgAsymScheme, AlgCipherMode, AlgECCScheme, AlgEccKeyEchange, AlgHash,
        AlgKdf, AlgKeyedHashScheme, AlgMacScheme, AlgPublic, AlgRSADecrypt, AlgRSAScheme,
        AlgSigScheme, AlgSym, AlgSymMode, AlgSymObj, EccCurve,
    };
    use crate::handles::{
        handle_ranges::{
            ACTHandle, AttachedComponentHandle, AuthHandle, HmacOrLoadedSessionHandle,
            NvIndexHandle, PCRHandle, PersistentHandle, PolicyOrSavedSessionHandle,
            TransientHandle,
        },
        AttachedComponent, AuthSession, Clear, Enables, Endorsement, Entity, Handle, Hierarchy,
        HierarchyAuth, HierarchyPolicy, Lockout, NVAuth, NVIndex, Object, Owner, Parent, Permanent,
        Persistent, Platform, Policy, Provision, Saved, ACT, HMAC, PCR,
    };
    use bitfields::{
        ACTAttributes, AlgorithAttributes, CommandCodeAttributes, LocalityAttributes,
        MemoryAttributes, NVAttributes, ObjectAttributes, PermanentAttributes, SessionAttributes,
        StartupClearAttributes,
    };
    use constants::{
        ArithmeticOperands, AttachedComponentErrorNone, AttachedComponentTag, Capability,
        ClockAdjust, CommandCode, PCRPropertyTag, PlatformSpecific, PropertyTag, ReturnCode,
        SessionType, StartupType, StructureTag, GENERATED,
    };
    use enums::{
        AESKeyBits, CAMELLIAKeyBits, RSAKeyBits, SM4KeyBits, StructureTagAttest,
        StructureTagCommand, TDESKeyBits,
    };
    use selectables::{
        AsymScheme, AttestBody, Capabilities, Digest, EccScheme, KdfScheme, KeyedHashScheme,
        Public, PublicParams, RSADecrypt, RSAScheme, Sensitive, SigScheme, Signature,
        SymCipherParams, SymDef, SymDefObject, Ticket,
    };
    use serde::{Deserialize, Serialize};
    use std::fmt::Debug;
    use structs::{
        ACTData, AlgorithmDescription, AlgorithmProperty, AlgortihmDetailECC, AsymParams,
        AttachedComponentOutput, Attest, AuthCommand, AuthResponse, CertifyInfo, ClockInfo,
        CommandAuditInfo, Context, ContextData, CreationData, CreationInfo, Derive, ECCParams,
        EccPoint, EncSchemeOAEP, IdObject, KeySchemeECDH, KeySchemeECMQ, KeyedHashParams,
        NVCertifyInfo, NVCounterParameters, NVDigestCertifyInfo, NVPublic, PCRSelection, QuoteInfo,
        RSAParams, SchemeECDAA, SchemeHMAC, SchemeHash, SchemeKDF2, SchemeKdf1Sp800_108,
        SchemeKdf1Sp800_56a, SchemeMGF1, SchemeXOR, SensitiveCreate, SessionAuditInfo,
        SigSchemeECDAA, SigSchemeECDSA, SigSchemeECSCHNORR, SigSchemeRSAPSS, SigSchemeRSASSA,
        SigSchemeSM2, SignatureECC, SignatureECDAA, SignatureECDSA, SignatureECSCHNORR,
        SignatureRSA, SignatureRSAPSS, SignatureRSASSA, SignatureSM2, TaggedPCRSelect,
        TaggedPolicy, TaggedProperty, TimeAttestInfo, TimeInfo,
    };

    /// Verify at compile-time, that all concrete types passed as a generic type
    /// parameter implement certain traits.
    fn is_normal<
        'a,
        T: Sized + Send + Sync + Unpin + Clone + Debug + PartialEq + Serialize + Deserialize<'a>,
    >() {
    }

    // structs
    is_normal::<AlgorithmDescription>();
    is_normal::<PCRSelection>();
    is_normal::<AlgorithmProperty>();
    is_normal::<TaggedProperty>();
    is_normal::<TaggedPCRSelect>();
    is_normal::<TaggedPolicy>();
    is_normal::<ACTData>();
    is_normal::<ClockInfo>();
    is_normal::<TimeInfo>();
    is_normal::<TimeAttestInfo>();
    is_normal::<CertifyInfo>();
    is_normal::<QuoteInfo>();
    is_normal::<CommandAuditInfo>();
    is_normal::<SessionAuditInfo>();
    is_normal::<CreationInfo>();
    is_normal::<NVCertifyInfo>();
    is_normal::<NVDigestCertifyInfo>();
    is_normal::<Attest>();
    is_normal::<AuthCommand>();
    is_normal::<AuthResponse>();
    is_normal::<Derive>();
    is_normal::<SensitiveCreate>();
    is_normal::<SchemeHash>();
    is_normal::<SchemeECDAA>();
    is_normal::<SchemeHMAC>();
    is_normal::<SchemeXOR>();
    is_normal::<SigSchemeRSASSA>();
    is_normal::<SigSchemeRSAPSS>();
    is_normal::<SigSchemeECDSA>();
    is_normal::<SigSchemeECDAA>();
    is_normal::<SigSchemeECSCHNORR>();
    is_normal::<SigSchemeSM2>();
    is_normal::<EncSchemeOAEP>();
    is_normal::<KeySchemeECDH>();
    is_normal::<KeySchemeECMQ>();
    is_normal::<SchemeMGF1>();
    is_normal::<SchemeKdf1Sp800_56a>();
    is_normal::<SchemeKDF2>();
    is_normal::<SchemeKdf1Sp800_108>();
    is_normal::<EccPoint>();
    is_normal::<AlgortihmDetailECC>();
    is_normal::<SignatureRSA>();
    is_normal::<SignatureRSASSA>();
    is_normal::<SignatureRSAPSS>();
    is_normal::<SignatureECC>();
    is_normal::<SignatureECDSA>();
    is_normal::<SignatureECDAA>();
    is_normal::<SignatureSM2>();
    is_normal::<SignatureECSCHNORR>();
    is_normal::<KeyedHashParams>();
    is_normal::<AsymParams>();
    is_normal::<RSAParams>();
    is_normal::<ECCParams>();
    is_normal::<IdObject>();
    is_normal::<NVCounterParameters>();
    is_normal::<NVPublic>();
    is_normal::<ContextData>();
    is_normal::<Context>();
    is_normal::<CreationData>();
    is_normal::<AttachedComponentOutput>();
    // selectables
    is_normal::<Digest>();
    is_normal::<Ticket>();
    is_normal::<Capabilities>();
    is_normal::<AttestBody>();
    is_normal::<SymDef>();
    is_normal::<SymDefObject>();
    is_normal::<SymCipherParams>();
    is_normal::<KeyedHashScheme>();
    is_normal::<SigScheme>();
    is_normal::<KdfScheme>();
    is_normal::<AsymScheme>();
    is_normal::<RSAScheme>();
    is_normal::<RSADecrypt>();
    is_normal::<EccScheme>();
    is_normal::<Signature>();
    is_normal::<PublicParams>();
    is_normal::<Public>();
    is_normal::<Sensitive>();
    // constants
    // TODO is_normal::<Spec>();
    is_normal::<GENERATED>();
    is_normal::<CommandCode>();
    is_normal::<ReturnCode>();
    is_normal::<ClockAdjust>();
    is_normal::<ArithmeticOperands>();
    is_normal::<StructureTag>();
    is_normal::<StartupType>();
    is_normal::<SessionType>();
    is_normal::<Capability>();
    is_normal::<PropertyTag>();
    is_normal::<PCRPropertyTag>();
    is_normal::<PlatformSpecific>();
    is_normal::<AttachedComponentTag>();
    is_normal::<AttachedComponentErrorNone>();
    // handles
    is_normal::<Handle>();
    is_normal::<Object>();
    is_normal::<Parent>();
    is_normal::<Persistent>();
    is_normal::<Entity>();
    is_normal::<PCR>();
    is_normal::<AuthSession>();
    is_normal::<HMAC>();
    is_normal::<Policy>();
    is_normal::<Context>();
    is_normal::<Saved>();
    is_normal::<Hierarchy>();
    is_normal::<Enables>();
    is_normal::<HierarchyAuth>();
    is_normal::<HierarchyPolicy>();
    is_normal::<Platform>();
    is_normal::<Owner>();
    is_normal::<Endorsement>();
    is_normal::<Provision>();
    is_normal::<Clear>();
    is_normal::<NVAuth>();
    is_normal::<Lockout>();
    is_normal::<NVIndex>();
    is_normal::<AttachedComponent>();
    is_normal::<ACT>();
    is_normal::<Permanent>();
    is_normal::<PCRHandle>();
    is_normal::<NvIndexHandle>();
    is_normal::<HmacOrLoadedSessionHandle>();
    is_normal::<PolicyOrSavedSessionHandle>();
    is_normal::<AuthHandle>();
    is_normal::<ACTHandle>();
    is_normal::<TransientHandle>();
    is_normal::<PersistentHandle>();
    is_normal::<AttachedComponentHandle>();
    // alg
    is_normal::<Alg>();
    is_normal::<AlgHash>();
    is_normal::<AlgAsym>();
    is_normal::<AlgSym>();
    is_normal::<AlgSymObj>();
    is_normal::<AlgSymMode>();
    is_normal::<AlgKdf>();
    is_normal::<AlgSigScheme>();
    is_normal::<AlgEccKeyEchange>();
    is_normal::<AlgMacScheme>();
    is_normal::<AlgCipherMode>();
    is_normal::<AlgKeyedHashScheme>();
    is_normal::<AlgAsymScheme>();
    is_normal::<AlgRSAScheme>();
    is_normal::<AlgRSADecrypt>();
    is_normal::<AlgECCScheme>();
    is_normal::<AlgPublic>();
    is_normal::<EccCurve>();
    // bitfields
    is_normal::<AlgorithAttributes>();
    is_normal::<ObjectAttributes>();
    is_normal::<SessionAttributes>();
    is_normal::<LocalityAttributes>();
    is_normal::<PermanentAttributes>();
    is_normal::<StartupClearAttributes>();
    is_normal::<MemoryAttributes>();
    is_normal::<ACTAttributes>();
    is_normal::<CommandCodeAttributes>();
    is_normal::<NVAttributes>();
    // enums
    is_normal::<StructureTagCommand>();
    is_normal::<StructureTagAttest>();
    is_normal::<AESKeyBits>();
    is_normal::<SM4KeyBits>();
    is_normal::<CAMELLIAKeyBits>();
    is_normal::<TDESKeyBits>();
    is_normal::<RSAKeyBits>();
}
