#[allow(dead_code)]
#[allow(nonstandard_style)]
pub mod tpm2_tss {
    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

    // specified TCTI API definition (currently: non-public tpm2-tss header)
    pub const TCTI_VERSION: u32 = 2;
}

impl Default for tpm2_tss::TSS2_TCTI_CONTEXT_COMMON_V1 {
    fn default() -> tpm2_tss::TSS2_TCTI_CONTEXT_COMMON_V1 {
        tpm2_tss::TSS2_TCTI_CONTEXT_COMMON_V1 {
            magic: 0,
            version: tpm2_tss::TCTI_VERSION,
            transmit: None,
            receive: None,
            finalize: None,
            cancel: None,
            getPollHandles: None,
            setLocality: None,
        }
    }
}

impl Default for tpm2_tss::TSS2_TCTI_CONTEXT_COMMON_V2 {
    fn default() -> tpm2_tss::TSS2_TCTI_CONTEXT_COMMON_V2 {
        tpm2_tss::TSS2_TCTI_CONTEXT_COMMON_V2 {
            v1: Default::default(),
            makeSticky: None,
        }
    }
}
