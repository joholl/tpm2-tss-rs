use strum::IntoEnumIterator;
use strum_macros::EnumIter;
use thiserror::Error;
use tss2_tcti_sys::tpm2_tss;

#[macro_export]
macro_rules! tcti_layer_rc {
    ($x:expr) => {
        (10 << tpm2_tss::TSS2_RC_LAYER_SHIFT) | $x
    };
}

// macro_rules! impl_from_for_specific_errors {
//     ($parent:ident, $($child:ident),*) => {
//         $(
//             impl fmt::Debug for $child {
//                 fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//                     // TODO
//                     write!(f, "{:?}", self)
//                 }
//             }

//             impl fmt::Display for $child {
//                 fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//                     // TODO
//                     write!(f, "{:?}", self)
//                 }
//             }

//             impl From<tpm2_tss::TPM2_RC> for $child {
//                 fn from(error_code: tpm2_tss::TPM2_RC) -> Self {
//                     let error = TctiError::from(error_code);
//                     match Self::try_from(error) {
//                         Ok(specific_error) => specific_error,
//                         Err(_) => Self::Unknown(error.into()),
//                     }
//                 }
//             }

//             impl $child {
//                 // we cannot implement From<TctiError> because it is implemented by the subenum macro already
//                 fn from_or_unknown(error: $parent) -> Self {
//                     match Self::try_from(error) {
//                         Ok(specific_error) => specific_error,
//                         Err(_) => Self::Unknown(error.into()),
//                     }
//                 }
//             }
//         )*
//     };
// }

macro_rules! debug_opt {
    ($x:expr) => {
        match $x {
            Some(x) => format!("{:?}", x),
            None => String::from("-"),
        }
    };
}

#[repr(u32)]
#[derive(Debug, Error, EnumIter, Copy, Clone, PartialEq, Eq)]
pub enum TctiError {
    #[error("GeneralFailure")]
    GeneralFailure = tcti_layer_rc!(1),

    #[error("Function is not implemented for this TCTI")]
    NotImplemented = tcti_layer_rc!(2),

    #[error(
        "BadContext: magic={}, expected magic={}",
        debug_opt!(magic),
        debug_opt!(expected_magic)
    )]
    BadContext {
        magic: Option<u64>,
        expected_magic: Option<u64>,
    } = tcti_layer_rc!(3),

    #[error("BadReference")]
    BadReference = tcti_layer_rc!(5),

    #[error("InsufficientBuffer")]
    InsufficientBuffer = tcti_layer_rc!(6),

    #[error("BadSequence")]
    BadSequence = tcti_layer_rc!(7),

    #[error("TryAgain")]
    TryAgain = tcti_layer_rc!(9),

    #[error("IoError")]
    IoError = tcti_layer_rc!(10),

    #[error("Bad Value")]
    BadValue = tcti_layer_rc!(11),

    #[error("Setting locality is not permitted")]
    NotPermitted = tcti_layer_rc!(12),

    // some error codes (e.g. InvalidSessions) are not applicable for TCTIs
    #[error("Function is not supported")]
    NotSupported = tcti_layer_rc!(21),

    #[error("Out of memory")]
    Memory = tcti_layer_rc!(23),

    #[error("A bad handle was passed")]
    // TODO value not specified yet
    BadHandle = tcti_layer_rc!(0xFFF),

    #[error("Unknown error. TCTI seems not to be compliant to the specification")]
    Unknown(u32) = tcti_layer_rc!(0xFFFFFFFF),
}

impl TctiError {
    pub fn discriminant(&self) -> u32 {
        // SAFETY: Because `Self` is marked `repr(u32)`, its layout is a `repr(C)` `union`
        // between `repr(C)` structs, each of which has the `u32` discriminant as its first
        // field, so we can read the discriminant without offsetting the pointer.
        unsafe { *<*const _>::from(self).cast::<u32>() }
    }
}

impl From<u32> for TctiError {
    fn from(value: u32) -> Self {
        for variant in TctiError::iter() {
            if value == variant.discriminant() {
                return variant;
            }
        }

        return TctiError::Unknown(value);
    }
}

impl From<TctiError> for u32 {
    fn from(value: TctiError) -> Self {
        return value.discriminant();
    }
}

// define_tcti_error_subgroup!(
//     TctiError,
//     GeneralFailure,
//     Memory,
//     NotSupported,
//     IoError,
//     BadValue,
//     BadReference,
//     BadContext,
//     Unknown
// );

// define_tcti_error_subgroup!(
//     TctiError,
//     GeneralFailure,
//     IoError,
//     BadContext,
//     BadSequence,
//     BadReference,
//     BadValue,
//     Unknown
// );

// define_tcti_error_subgroup!(
//     TctiError,
//     GeneralFailure,
//     InsufficientBuffer,
//     NotImplemented,
//     BadContext,
//     TryAgain,
//     IoError,
//     BadReference,
//     BadValue,
//     BadSequence,
//     Unknown
// );

// define_tcti_error_subgroup!(
//     CancelError,
//     GeneralFailure,
//     NotImplemented,
//     IoError,
//     BadSequence,
//     BadReference,
//     BadContext,
//     Unknown
// );

// define_tcti_error_subgroup!(
//     GetPollHandlesError,
//     GeneralFailure,
//     NotImplemented,
//     BadReference,
//     InsufficientBuffer,
//     BadContext,
//     Unknown
// );

// define_tcti_error_subgroup!(
//     SetLocalityError,
//     GeneralFailure,
//     NotImplemented,
//     BadReference,
//     IoError,
//     BadSequence,
//     BadValue,
//     NotPermitted,
//     NotSupported,
//     BadContext,
//     Unknown
// );

// define_tcti_error_subgroup!(
//     MakeStickyError,
//     GeneralFailure,
//     NotImplemented,
//     BadReference,
//     IoError,
//     BadSequence,
//     BadContext,
//     BadHandle,
//     BadValue,
//     Unknown
// );
