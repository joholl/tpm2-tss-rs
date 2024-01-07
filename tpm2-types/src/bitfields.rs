//TODO

/// TPMA_ALGORITHM
pub type AlgorithAttributes = u32;

/// TPMA_OBJECT
pub type ObjectAttributes = u32;

/// TPMA_SESSION
pub type SessionAttributes = u8;

/// TPMA_LOCALITY
pub type LocalityAttributes = u8;

/// TPMA_PERMANENT
pub type PermanentAttributes = u32;

/// TPMA_STARTUP_CLEAR
pub type StartupClearAttributes = u32;

/// TPMA_MEMORY
pub type MemoryAttributes = u32;

/// TPMA_ACT
pub type ACTAttributes = u32;

/// TPMA_CC
pub type CommandCodeAttributes = u32;

/// TPMA_NV
pub type NVAttributes = u32;

// /// TPM_NT
// #[repr(u8)]
// pub enum NVType {
//     Ordinary = 0x0,
//     Counter = 0x10,
//     Bits = 0x2,
//     Extend = 0x4,
//     PinFail = 0x8,
//     PinPass = 0x9,
// }
