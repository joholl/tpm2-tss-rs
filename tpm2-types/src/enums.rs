use crate::constants::StructureTag;
use serde_repr::{Deserialize_repr, Serialize_repr};

// TODO
// TPMI_ST_COMMAND_TAG
pub type StructureTagCommand = StructureTag;

// TODO
// TPMI_ST_ATTEST
pub type StructureTagAttest = StructureTag;

// Based on https://trustedcomputinggroup.org/wp-content/uploads/TCG-Algorithm-Registry-Revision-1.34_pub.pdf

/// TPMI_AES_KEY_BITS, AES_KEY_SIZES_BITS
#[derive(Deserialize_repr, Serialize_repr, Debug, Clone, PartialEq)]
#[repr(u16)]
pub enum AESKeyBits {
    _128 = 128,
    _192 = 192,
    _256 = 256,
}

/// TPMI_SM4_KEY_BITS, SM4_KEY_SIZES_BITS
#[derive(Deserialize_repr, Serialize_repr, Debug, Clone, PartialEq)]
#[repr(u16)]
pub enum SM4KeyBits {
    _128 = 128,
    _192 = 192,
    _256 = 256,
}

/// TPMI_CAMELLIA_KEY_BITS, CAMELLIA_KEY_SIZES_BITS
#[derive(Deserialize_repr, Serialize_repr, Debug, Clone, PartialEq)]
#[repr(u16)]
pub enum CAMELLIAKeyBits {
    _128 = 128,
    _192 = 192,
    _256 = 256,
}

/// TPMI_TDES_KEY_BITS, TDES_KEY_SIZES_BITS
#[derive(Deserialize_repr, Serialize_repr, Debug, Clone, PartialEq)]
#[repr(u16)]
pub enum TDESKeyBits {
    _128 = 128,
    _192 = 192,
}

/// TPMI_RSA_KEY_BITS, RSA_KEY_SIZES_BITS
#[derive(Deserialize_repr, Serialize_repr, Debug, Clone, PartialEq)]
#[repr(u16)]
pub enum RSAKeyBits {
    _1024 = 1024,
    _2048 = 2048,
    _3072 = 3072,
    _4096 = 4096,
}
