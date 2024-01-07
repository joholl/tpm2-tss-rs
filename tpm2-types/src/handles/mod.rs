mod handle_ranges;

use crate::{
    handles::handle_ranges::{
        ACTHandle, AttachedComponentHandle, AuthHandle, HmacOrLoadedSessionHandle, NvIndexHandle,
        PCRHandle, PersistentHandle, PolicyOrSavedSessionHandle, TransientHandle,
    },
    util::ConstantU32,
};
use serde::{de::Visitor, Deserialize, Deserializer, Serialize, Serializer};
use tpm2_types_macro::HandleSubset;

/// TPM_HANDLE
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Handle {
    /// TPM_HT_PCR, HR_PCR: 0x00000000..0x00000020
    PCR(PCRHandle),
    /// TPM_HT_NV_INDEX, HR_NV_INDEX: 0x01000000..0x02000000
    NvIndex(NvIndexHandle),
    /// TPM_HT_HMAC_SESSION/TPM_HT_LOADED_SESSION, HR_HMAC_SESSION:
    /// 0x02000000..0x02FFFFFF
    HmacOrLoadedSession(HmacOrLoadedSessionHandle),
    /// TPM_HT_POLICY_SESSION/TPM_HT_SAVED_SESSION, HR_POLICY_SESSION:
    /// 0x03000000..0x03FFFFFF
    PolicyOrSavedSession(PolicyOrSavedSessionHandle),
    // ============ Permanent handles (TPM_RH) ============
    // /// TPM_RH_SRK: 0x40000000, not used
    // // StorageRootKey
    /// TPM_RH_OWNER: 0x40000001
    ///
    /// Handle references the Storage Primary Seed
    /// (SPS), the *ownerAuth*, and the *ownerPolicy*
    Owner,
    // /// TPM_RH_REVOKE: 0x40000002, not used
    // Revoke,
    // /// TPM_RH_TRANSPORT: 0x40000003, not used
    // Transport,
    // /// TPM_RH_OPERATOR: 0x40000004, not used
    // Operator,
    // /// TPM_RH_ADMIN: 0x40000005
    // Admin,
    // /// TPM_RH_EK: 0x40000006, not used
    // EndorsementKey
    /// TPM_RH_NULL: 0x40000007
    ///
    /// A handle associated with the null hierarchy, an
    /// EmptyAuth *authValue*, and an Empty Policy *authPolicy*.
    Null,
    /// TPM_RH_UNASSIGNED: 0x40000008.
    ///
    /// Value reserved to the TPM to indicate a
    /// handle location that has not been initialized or assigned
    Unassigned,
    /// TPM_RS_PW: 0x40000009.
    ///
    /// Authorization value used to indicate a password authorization session
    PasswordSession,
    /// TPM_RH_LOCKOUT: 0x4000000A.
    ///
    /// References the authorization associated with the dictionary attack
    /// lockout reset
    Lockout,
    /// TPM_RH_ENDORSEMENT: 0x4000000B.
    ///
    /// References the Endorsement Primary Seed (EPS), *endorsementAuth*, and
    /// *endorsementPolicy*
    Endorsement,
    /// TPM_RH_PLATFORM: 0x4000000C.
    ///
    /// References the Platform Primary Seed (PPS), *platformAuth*, and
    /// *platformPolicy*
    Platform,
    /// TPM_RH_PLATFORM_NV: 0x4000000D.
    ///
    /// For phEnableNV.
    PlatformNV,
    /// TPM_RH_AUTH: 0x40000010..0x40000110
    ///
    /// Start of a range of authorization values that are vendor-specific. A TPM
    /// may support any (or none) of the values in this range as are needed for
    /// vendor-specific purposes. Disabled if ehEnable is CLEAR.
    Auth(AuthHandle),
    /// TPM_RH_ACT: 0x40000110..0x40000120.
    ///
    /// Range of authenticated countdown timers (ACT).
    ACT(ACTHandle),
    // ============ End Permanent Handles ============
    Transient(TransientHandle),
    Persistent(PersistentHandle),
    AttachedComponent(AttachedComponentHandle),
}
// TODO write own serialize/deserialize?

impl TryFrom<u32> for Handle {
    type Error = ();

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        match value {
            // TODO MAX_INCL is a workaround until
            // https://github.com/rust-lang/rust/issues/67792 is part of stable
            PCRHandle::MIN..=PCRHandle::MAX_INCL => Ok(Handle::PCR(value.try_into()?)),
            NvIndexHandle::MIN..=NvIndexHandle::MAX_INCL => Ok(Handle::NvIndex(value.try_into()?)),
            HmacOrLoadedSessionHandle::MIN..=HmacOrLoadedSessionHandle::MAX_INCL => {
                Ok(Handle::HmacOrLoadedSession(value.try_into()?))
            }
            PolicyOrSavedSessionHandle::MIN..=PolicyOrSavedSessionHandle::MAX_INCL => {
                Ok(Handle::PolicyOrSavedSession(value.try_into()?))
            }
            0x40000001 => Ok(Handle::Owner),
            0x40000007 => Ok(Handle::Null),
            0x40000008 => Ok(Handle::Unassigned),
            0x40000009 => Ok(Handle::PasswordSession),
            0x4000000A => Ok(Handle::Lockout),
            0x4000000B => Ok(Handle::Endorsement),
            0x4000000C => Ok(Handle::Platform),
            0x4000000D => Ok(Handle::PlatformNV),
            AuthHandle::MIN..=AuthHandle::MAX_INCL => Ok(Handle::Auth(value.try_into()?)),
            ACTHandle::MIN..=ACTHandle::MAX_INCL => Ok(Handle::ACT(value.try_into()?)),
            TransientHandle::MIN..=TransientHandle::MAX_INCL => {
                Ok(Handle::Transient(value.try_into()?))
            }
            PersistentHandle::MIN..=PersistentHandle::MAX_INCL => {
                Ok(Handle::Persistent(value.try_into()?))
            }
            AttachedComponentHandle::MIN..=AttachedComponentHandle::MAX_INCL => {
                Ok(Handle::AttachedComponent(value.try_into()?))
            }
            _ => Err(()),
        }
    }
}

impl From<Handle> for u32 {
    fn from(value: Handle) -> Self {
        match value {
            Handle::PCR(handle) => u32::from(handle),
            Handle::NvIndex(handle) => u32::from(handle),
            Handle::HmacOrLoadedSession(handle) => u32::from(handle),
            Handle::PolicyOrSavedSession(handle) => u32::from(handle),
            Handle::Owner => 0x40000001,
            Handle::Null => 0x40000007,
            Handle::Unassigned => 0x40000008,
            Handle::PasswordSession => 0x40000009,
            Handle::Lockout => 0x4000000A,
            Handle::Endorsement => 0x4000000B,
            Handle::Platform => 0x4000000C,
            Handle::PlatformNV => 0x4000000D,
            Handle::Auth(handle) => u32::from(handle),
            Handle::ACT(handle) => u32::from(handle),
            Handle::Transient(handle) => u32::from(handle),
            Handle::Persistent(handle) => u32::from(handle),
            Handle::AttachedComponent(handle) => u32::from(handle),
        }
    }
}

impl<'de> Deserialize<'de> for Handle {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct HandleVisitor;
        impl<'de> Visitor<'de> for HandleVisitor {
            type Value = Handle;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("u32 handle")
            }

            fn visit_u32<E>(self, v: u32) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Handle::try_from(v)
                    .map_err(|_| E::invalid_value(serde::de::Unexpected::Unsigned(v.into()), &self))
            }
        }

        deserializer.deserialize_u32(HandleVisitor)
    }
}

impl Serialize for Handle {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_u32(u32::from(*self))
    }
}

//////////////////////////////////////////////////////////////////////

/// TPMI_DH_OBJECT
#[derive(HandleSubset, Debug, Clone, Copy, PartialEq)]
pub enum Object {
    /// see [Handle::Transient]
    Transient(TransientHandle),
    /// see [Handle::Persistent]
    Persistent(PersistentHandle),
    /// see [Handle::Null]
    Null,
}

/// TPMI_DH_PARENT
#[derive(HandleSubset, Debug, Clone, Copy, PartialEq)]
pub enum Parent {
    /// see [Handle::Transient]
    Transient(TransientHandle),
    /// see [Handle::Persistent]
    Persistent(PersistentHandle),
    /// see [Handle::Owner]
    Owner,
    /// see [Handle::Platform]
    Platform,
    /// see [Handle::Endorsement]
    Endorsement,
    /// see [Handle::Null]
    Null,
}

/// TPMI_DH_PERSISTENT
pub type Persistent = PersistentHandle;

/// TPMI_DH_ENTITY
#[derive(HandleSubset, Debug, Clone, Copy, PartialEq)]
pub enum Entity {
    /// see [Handle::Owner]
    Owner,
    /// see [Handle::Endorsement]
    Endorsement,
    /// see [Handle::Platform]
    Platform,
    /// see [Handle::Lockout]
    Lockout,
    /// see [Handle::Transient]
    Transient(TransientHandle),
    /// see [Handle::Persistent]
    Persistent(PersistentHandle),
    /// see [Handle::NvIndex]
    NvIndex(NvIndexHandle),
    /// see [Handle::PCR]
    PCR(PCRHandle),
    /// see [Handle::Auth]
    Auth(AuthHandle),
    /// see [Handle::Null]
    Null,
}

/// TPMI_DH_PCR
#[derive(HandleSubset, Debug, Clone, Copy, PartialEq)]
pub enum PCR {
    /// see [Handle::PCR]
    PCR(PCRHandle),
    /// see [Handle::Null]
    Null,
}

/// TPMI_SH_AUTH_SESSION
#[derive(HandleSubset, Debug, Clone, Copy, PartialEq)]
pub enum AuthSession {
    /// see [Handle::HmacOrLoadedSession]
    HmacOrLoadedSession(HmacOrLoadedSessionHandle),
    /// see [Handle::PolicyOrSavedSession]
    PolicyOrSavedSession(PolicyOrSavedSessionHandle),
    /// see [Handle::PasswordSession]
    PasswordSession,
}

/// TPMI_SH_HMAC
pub type HMAC = HmacOrLoadedSessionHandle;

/// TPMI_SH_POLICY
pub type Policy = PolicyOrSavedSessionHandle;

/// TPMI_DH_CONTEXT
#[derive(HandleSubset, Debug, Clone, Copy, PartialEq)]
pub enum Context {
    /// see [Handle::HmacOrLoadedSession]
    HmacOrLoadedSession(HmacOrLoadedSessionHandle),
    /// see [Handle::PolicyOrSavedSession]
    PolicyOrSavedSession(PolicyOrSavedSessionHandle),
    /// see [Handle::Transient]
    Transient(TransientHandle),
}

/// TPMI_DH_SAVED
#[derive(HandleSubset, Debug, Clone, Copy, PartialEq)]
pub enum Saved {
    /// see [Handle::HmacOrLoadedSession]
    HmacOrLoadedSession(HmacOrLoadedSessionHandle),
    /// see [Handle::PolicyOrSavedSession]
    PolicyOrSavedSession(PolicyOrSavedSessionHandle),
    /// see [Handle::Transient]
    /// TODO: more specifically, only 0x80000000, 0x80000001 and 0x80000002 are allowed
    Transient(TransientHandle),
}

/// TPMI_RH_HIERARCHY
#[derive(HandleSubset, Default, Debug, Clone, Copy, PartialEq)]
pub enum Hierarchy {
    /// see [Handle::Owner]
    #[default]
    Owner,
    /// see [Handle::Platform]
    Platform,
    /// see [Handle::Endorsement]
    Endorsement,
    /// see [Handle::Null]
    Null,
}

/// TPMI_RH_ENABLES
#[derive(HandleSubset, Debug, Clone, Copy, PartialEq)]
pub enum Enables {
    /// see [Handle::Owner]
    Owner,
    /// see [Handle::Platform]
    Platform,
    /// see [Handle::Endorsement]
    Endorsement,
    /// see [Handle::PlatformNV]
    PlatformNV,
    /// see [Handle::Null]
    Null,
}

/// TPMI_RH_HIERARCHY_AUTH
#[derive(HandleSubset, Debug, Clone, Copy, PartialEq)]
pub enum HierarchyAuth {
    /// see [Handle::Owner]
    Owner,
    /// see [Handle::Platform]
    Platform,
    /// see [Handle::Endorsement]
    Endorsement,
    /// see [Handle::Lockout]
    Lockout,
}

/// TPMI_RH_HIERARCHY_POLICY
#[derive(HandleSubset, Debug, Clone, Copy, PartialEq)]
pub enum HierarchyPolicy {
    /// see [Handle::Owner]
    Owner,
    /// see [Handle::Platform]
    Platform,
    /// see [Handle::Endorsement]
    Endorsement,
    /// see [Handle::Lockout]
    Lockout,
    /// see [Handle::ACT]
    ACT(ACTHandle),
}

/// TPMI_RH_PLATFORM
pub type Platform = ConstantU32<0x4000000C>; // u32::from(Handle::Platform)

impl TryFrom<Handle> for Platform {
    type Error = ();

    fn try_from(value: Handle) -> Result<Self, Self::Error> {
        u32::from(value).try_into()
    }
}

impl From<Platform> for Handle {
    fn from(_value: Platform) -> Self {
        // An u32 from a valid handle can always be converted back to a handle
        Handle::try_from(Platform::VALUE).unwrap()
    }
}

/// TPMI_RH_OWNER
#[derive(HandleSubset, Debug, Clone, Copy, PartialEq)]
pub enum Owner {
    /// see [Handle::Owner]
    Owner,
    /// see [Handle::Null]
    Null,
}

/// TPMI_RH_ENDORSEMENT
#[derive(HandleSubset, Debug, Clone, Copy, PartialEq)]
pub enum Endorsement {
    /// see [Handle::Endorsement]
    Endorsement,
    /// see [Handle::Null]
    Null,
}

/// TPMI_RH_PROVISION
#[derive(HandleSubset, Debug, Clone, Copy, PartialEq)]
pub enum Provision {
    /// see [Handle::Owner]
    Owner,
    /// see [Handle::Endorsement]
    Endorsement,
}

/// TPMI_RH_CLEAR
#[derive(HandleSubset, Debug, Clone, Copy, PartialEq)]
pub enum Clear {
    /// see [Handle::Lockout]
    Lockout,
    /// see [Handle::Platform]
    Platform,
}

/// TPMI_RH_NV_AUTH
#[derive(HandleSubset, Debug, Clone, Copy, PartialEq)]
pub enum NVAuth {
    /// see [Handle::Platform]
    Platform,
    /// see [Handle::Owner]
    Owner,
    /// see [Handle::NvIndex]
    NvIndex(NvIndexHandle),
}

/// TPMI_RH_LOCKOUT
pub type Lockout = ConstantU32<0x4000000A>; // u32::from(Handle::Lockout)

impl TryFrom<Handle> for Lockout {
    type Error = ();

    fn try_from(value: Handle) -> Result<Self, Self::Error> {
        u32::from(value).try_into()
    }
}

impl From<Lockout> for Handle {
    fn from(_value: Lockout) -> Self {
        // An u32 from a valid handle can always be converted back to a handle
        Handle::try_from(Lockout::VALUE).unwrap()
    }
}

/// TPMI_RH_NV_INDEX
pub type NVIndex = NvIndexHandle;

/// TPMI_RH_AC
pub type AttachedComponent = AttachedComponentHandle;

/// TPMI_RH_AC
pub type ACT = ACTHandle;

/// Not specified, added for convenience.
#[derive(HandleSubset, Debug, Clone, Copy, PartialEq)]
pub enum Permanent {
    /// see [Handle::Owner]
    Owner,
    /// see [Handle::Null]
    Null,
    /// see [Handle::Unassigned]
    Unassigned,
    /// see [Handle::PasswordSession]
    PasswordSession,
    /// see [Handle::Lockout]
    Lockout,
    /// see [Handle::Endorsement]
    Endorsement,
    /// see [Handle::Platform]
    Platform,
    /// see [Handle::PlatformNV]
    PlatformNV,
    /// see [Handle::Auth]
    Auth(AuthHandle),
    /// see [Handle::ACT]
    ACT(ACTHandle),
}
