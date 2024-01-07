use crate::util::ConstantU32;
use serde_repr::{Deserialize_repr, Serialize_repr};

/// TPM_SPEC
pub struct Spec;
impl Spec {
    pub const FAMILY: u32 = 0x322e3000;
    pub const LEVEL: u32 = 00;
    pub const VERSION: u32 = 159;
    pub const YEART: u32 = 2020;
    pub const DAY_OF_YEAR: u32 = 170;
}

/// TPM_GENERATED: 0xff544347
pub type GENERATED = ConstantU32<{ u32::from_be_bytes(*b"\xffTCG") }>;

/// TPM_CC
#[derive(Deserialize_repr, Serialize_repr, Debug, Clone, Default, PartialEq)]
#[repr(u32)]
pub enum CommandCode {
    /// NV_UndefineSpaceSpecial
    NVUndefineSpaceSpecial = 0x0000011F,
    EvictControl = 0x00000120,
    HierarchyControl = 0x00000121,
    /// NV_UndefineSpace
    NVUndefineSpace = 0x00000122,
    ChangeEPS = 0x00000124,
    ChangePPS = 0x00000125,
    Clear = 0x00000126,
    ClearControl = 0x00000127,
    ClockSet = 0x00000128,
    HierarchyChangeAuth = 0x00000129,
    /// NV_DefineSpace
    NVDefineSpace = 0x0000012A,
    /// PCR_Allocate
    PCRAllocate = 0x0000012B,
    /// PCR_SetAuthPolicy
    PCRSetAuthPolicy = 0x0000012C,
    /// PP_Commands
    PPCommands = 0x0000012D,
    SetPrimaryPolicy = 0x0000012E,
    FieldUpgradeStart = 0x0000012F,
    ClockRateAdjust = 0x00000130,
    CreatePrimary = 0x00000131,
    /// NV_GlobalWriteLock
    NVGlobalWriteLock = 0x00000132,
    GetCommandAuditDigest = 0x00000133,
    /// NV_Increment
    NVIncrement = 0x00000134,
    /// NV_SetBits
    NVSetBits = 0x00000135,
    /// NV_Extend
    NVExtend = 0x00000136,
    /// NV_Write
    NVWrite = 0x00000137,
    /// NV_WriteLock
    NVWriteLock = 0x00000138,
    DictionaryAttackLockReset = 0x00000139,
    DictionaryAttackParameters = 0x0000013A,
    /// NV_ChangeAuth
    NVChangeAuth = 0x0000013B,
    /// PCR_Event
    PCREvent = 0x0000013C,
    /// PCR_Reset
    PCRReset = 0x0000013D,
    SequenceComplete = 0x0000013E,
    SetAlgorithmSet = 0x0000013F,
    SetCommandCodeAuditStatus = 0x00000140,
    FieldUpgradeData = 0x00000141,
    IncrementalSelfTest = 0x00000142,
    SelfTest = 0x00000143,
    Startup = 0x00000144,
    Shutdown = 0x00000145,
    StirRandom = 0x00000146,
    ActivateCredential = 0x00000147,
    Certify = 0x00000148,
    PolicyNV = 0x00000149,
    CertifyCreation = 0x0000014A,
    Duplicate = 0x0000014B,
    GetTime = 0x0000014C,
    GetSessionAuditDigest = 0x0000014D,
    /// NV_Read
    NVRead = 0x0000014E,
    /// NV_ReadLock
    NVReadLock = 0x0000014F,
    ObjectChangeAuth = 0x00000150,
    PolicySecret = 0x00000151,
    Rewrap = 0x00000152,
    Create = 0x00000153,
    /// ECDH_ZGen
    ECDHZGen = 0x00000154,
    HMAC = 0x00000155,
    Import = 0x00000156,
    Load = 0x00000157,
    Quote = 0x00000158,
    /// RSA_Decrypt
    RSADecrypt = 0x00000159,
    /// HMAC_Start
    HMACStart = 0x0000015B,
    SequenceUpdate = 0x0000015C,
    Sign = 0x0000015D,
    Unseal = 0x0000015E,
    PolicySigned = 0x00000160,
    ContextLoad = 0x00000161,
    ContextSave = 0x00000162,
    /// ECDH_KeyGen
    ECDHKeyGen = 0x00000163,
    EncryptDecrypt = 0x00000164,
    FlushContext = 0x00000165,
    LoadExternal = 0x00000167,
    MakeCredential = 0x00000168,
    /// NV_ReadPublic
    NVReadPublic = 0x00000169,
    PolicyAuthorize = 0x0000016A,
    PolicyAuthValue = 0x0000016B,
    PolicyCommandCode = 0x0000016C,
    PolicyCounterTimer = 0x0000016D,
    PolicyCpHash = 0x0000016E,
    PolicyLocality = 0x0000016F,
    PolicyNameHash = 0x00000170,
    PolicyOR = 0x00000171,
    PolicyTicket = 0x00000172,
    ReadPublic = 0x00000173,
    /// RSA_Encrypt
    RSAEncrypt = 0x00000174,
    StartAuthSession = 0x00000176,
    VerifySignature = 0x00000177,
    /// ECC_Parameters
    ECCParameters = 0x00000178,
    FirmwareRead = 0x00000179,
    GetCapability = 0x0000017A,
    #[default]
    GetRandom = 0x0000017B,
    GetTestResult = 0x0000017C,
    Hash = 0x0000017D,
    /// PCR_Read
    PCRRead = 0x0000017E,
    PolicyPCR = 0x0000017F,
    PolicyRestart = 0x00000180,
    ReadClock = 0x00000181,
    /// PCR_Extend
    PCRExtend = 0x00000182,
    /// PCR_SetAuthValue
    PCRSetAuthValue = 0x00000183,
    /// NV_Certify
    NVCertify = 0x00000184,
    EventSequenceComplete = 0x00000185,
    HashSequenceStart = 0x00000186,
    PolicyPhysicalPresence = 0x00000187,
    PolicyDuplicationSelect = 0x00000188,
    PolicyGetDigest = 0x00000189,
    TestParms = 0x0000018A,
    Commit = 0x0000018B,
    PolicyPassword = 0x0000018C,
    /// ZGen_2Phase
    ZGen2Phase = 0x0000018D,
    /// EC_Ephemeral
    ECEphemeral = 0x0000018E,
    PolicyNvWritten = 0x0000018F,
    PolicyTemplate = 0x00000190,
    CreateLoaded = 0x00000191,
    PolicyAuthorizeNV = 0x00000192,
    EncryptDecrypt2 = 0x00000193,
    /// AC_GetCapability
    ACGetCapability = 0x00000194,
    /// AC_Send
    ACSend = 0x00000195,
    /// Policy_AC_SendSelect
    PolicyACSendSelect = 0x00000196,
    CertifyX509 = 0x00000197,
    /// ACT_SetTimeout
    ACTSetTimeout = 0x00000198,
}

// TODO
/// TPM_RC
pub type ReturnCode = u32;

/// TPM_CLOCK_ADJUST
#[derive(Deserialize_repr, Serialize_repr, Debug, Clone, Default, PartialEq)]
#[repr(i8)]
pub enum ClockAdjust {
    CoarseSlower = -3,
    MediumSlower = -2,
    FineSlower = -1,
    #[default]
    NoChange = 0,
    FineFaster = 1,
    MediumFaster = 2,
    CoarseFaster = 3,
}

/// TPM_EO
#[derive(Deserialize_repr, Serialize_repr, Debug, Clone, Default, PartialEq)]
#[repr(u16)]
pub enum ArithmeticOperands {
    #[default]
    Eq = 0x0000,
    NEq = 0x0001,
    SignedGT = 0x0002,
    UnsignedGT = 0x0003,
    SignedLT = 0x0004,
    UnsignedLT = 0x0005,
    SignedGE = 0x0006,
    UnsignedGE = 0x0007,
    SignedLE = 0x0008,
    UnsignedLE = 0x0009,
    BitSet = 0x000A,
    BitClear = 0x000B,
}

/// TPM_ST
#[derive(Deserialize_repr, Serialize_repr, Debug, Clone, Default, PartialEq)]
#[repr(u16)]
pub enum StructureTag {
    #[default]
    ResponseCommand = 0x00C4,
    Null = 0x8000,
    NoSessions = 0x8001,
    Sessions = 0x8002,
    AttestNV = 0x8014,
    AttestCommandAudit = 0x8015,
    AttestSessionAudit = 0x8016,
    AttestCertify = 0x8017,
    AttestQuote = 0x8018,
    AttestTime = 0x8019,
    AttestCreation = 0x801A,
    AttestNVDigest = 0x801C,
    Creation = 0x8021,
    Verified = 0x8022,
    AuthSecret = 0x8023,
    Hashcheck = 0x8024,
    AuthSigned = 0x8025,
    FUManifest = 0x8029,
}

/// TPM_SU
#[derive(Deserialize_repr, Serialize_repr, Debug, Clone, Default, PartialEq)]
#[repr(u16)]
pub enum StartupType {
    #[default]
    Clear = 0x0000,
    State = 0x0001,
}

/// TPM_SE
#[derive(Deserialize_repr, Serialize_repr, Debug, Clone, Default, PartialEq)]
#[repr(u8)]
pub enum SessionType {
    #[default]
    HMAC = 0x00,
    Policy = 0x01,
    Trial = 0x03,
}

/// TPM_CAP
#[derive(Deserialize_repr, Serialize_repr, Debug, Clone, Default, PartialEq)]
#[repr(u32)]
pub enum Capability {
    #[default]
    Algs = 0x00000000,
    Handles = 0x00000001,
    Commands = 0x00000002,
    PPCommands = 0x00000003,
    AuditCommands = 0x00000004,
    Pcrs = 0x00000005,
    TpmProperties = 0x00000006,
    PcrProperties = 0x00000007,
    EccCurves = 0x00000008,
    AuthPolicies = 0x00000009,
    ACT = 0x0000000A,
    VendorProperty = 0x00000100,
}

/// TPM_PT
#[derive(Deserialize_repr, Serialize_repr, Debug, Clone, Default, PartialEq)]
#[repr(u32)]
pub enum PropertyTag {
    #[default]
    None = 0x00000000,
    FamilyIndicator = 0x00000100,
    Level = 0x00000101,
    Revision = 0x00000102,
    DayOfYear = 0x00000103,
    Year = 0x00000104,
    Manufacturer = 0x00000105,
    VendorString1 = 0x00000106,
    VendorString2 = 0x00000107,
    VendorString3 = 0x00000108,
    VendorString4 = 0x00000109,
    VendorTpmType = 0x00000110,
    FirmwareVersion1 = 0x00000111,
    FirmwareVersion2 = 0x00000112,
    InputBuffer = 0x00000113,
    HRTransientMin = 0x00000114,
    HRPersistentMin = 0x00000115,
    HRLoadedMin = 0x00000116,
    ActiveSessionsMax = 0x00000117,
    PCRCount = 0x00000118,
    PCRSelectMin = 0x00000119,
    ContextGapMax = 0x00000120,
    NvCountersMax = 0x00000122,
    NvIndexMax = 0x00000123,
    Memory = 0x00000124,
    ClockUpdate = 0x00000125,
    ContextHash = 0x00000126,
    ContextSym = 0x00000127,
    ContextSymSize = 0x00000128,
    OrderlyCount = 0x00000129,
    MaxCommandSize = 0x00000130,
    MaxResponseSize = 0x00000131,
    MaxDigest = 0x00000132,
    MaxObjectContext = 0x00000133,
    MaxSessionContext = 0x00000134,
    PSFamilyIndicator = 0x00000135,
    PSLevel = 0x00000136,
    PSRevision = 0x00000137,
    PSDayOfYear = 0x00000138,
    PSYear = 0x00000139,
    SplitMax = 0x00000140,
    TotalCommands = 0x00000141,
    LibraryCommands = 0x00000142,
    VendorCommands = 0x00000143,
    NvBufferMax = 0x00000144,
    Modes = 0x00000145,
    MaxCapBuffer = 0x00000146,
    Permanent = 0x00000200,
    StartupClear = 0x00000201,
    HRNvIndex = 0x00000202,
    HRLoaded = 0x00000203,
    HRLoadedAvail = 0x00000204,
    HRActive = 0x00000205,
    HRActiveAvail = 0x00000206,
    HRTransientAvail = 0x00000207,
    HRPersistent = 0x00000208,
    HRPersistentAvail = 0x00000209,
    NVCounters = 0x00000210,
    NVCountersAvail = 0x00000211,
    AlgorithmSet = 0x00000212,
    LoadedCurves = 0x00000213,
    LockoutCounter = 0x00000214,
    MaxAuthFail = 0x00000215,
    LockoutInterval = 0x00000216,
    LockoutRecovery = 0x00000217,
    NVWriteRecovery = 0x00000218,
    AuditCounter0 = 0x00000219,
    AuditCounter1 = 0x00000220,
}

/// TPM_PT_PCR
#[derive(Deserialize_repr, Serialize_repr, Debug, Clone, Default, PartialEq)]
#[repr(u32)]
pub enum PCRPropertyTag {
    #[default]
    Save = 0x00000000,
    ExtendL0 = 0x00000001,
    ResetL0 = 0x00000002,
    ExtendL1 = 0x00000003,
    ResetL1 = 0x00000004,
    ExtendL2 = 0x00000005,
    ResetL2 = 0x00000006,
    ExtendL3 = 0x00000007,
    ResetL3 = 0x00000008,
    ExtendL4 = 0x00000009,
    ResetL4 = 0x0000000A,
    PCRNoIncrement = 0x00000011,
    PCRDRTMReset = 0x00000012,
    PCRPolicy = 0x00000013,
    PCRAuth = 0x00000014,
}

/// TPM_PS
#[derive(Deserialize_repr, Serialize_repr, Debug, Clone, Default, PartialEq)]
#[repr(u32)]
pub enum PlatformSpecific {
    #[default]
    Main = 0x00000000,
    PC = 0x00000001,
    PDA = 0x00000002,
    CellPhone = 0x00000003,
    Server = 0x00000004,
    Peripheral = 0x00000005,
    TPMSoftwareStack = 0x00000006,
    Storage = 0x00000007,
    Authentication = 0x00000008,
    Embedded = 0x00000009,
    Hardcopy = 0x0000000A,
    Infrastructure = 0x0000000B,
    Virtualization = 0x0000000C,
    TrustedNetworkConnect = 0x0000000D,
    MultiTenant = 0x0000000E,
    TechnicalCommittee = 0x0000000F,
}

////////////////////

/// TPM_AT
#[derive(Deserialize_repr, Serialize_repr, Debug, Clone, PartialEq)]
#[repr(u32)]
pub enum AttachedComponentTag {
    Any = 0,
    Error = 1,
    PairingValue1 = 2,
    Vendor = 0x80000000,
}

/// TPM_AE_NONE
pub type AttachedComponentErrorNone = ConstantU32<0>;
