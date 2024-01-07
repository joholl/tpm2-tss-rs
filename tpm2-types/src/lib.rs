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

fn is_normal<T: Sized + Send + Sync + Unpin>() {}

#[test]
fn normal_types() {
    is_normal::<types::Public>();
    // TODO do this for all pub types
}
