use serde::{de::Visitor, Deserialize, Deserializer, Serialize, Serializer};

use crate::handles::Handle;

/// MAX is exclusive
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct HandleRange<const MIN: u32, const MAX: u32> {
    value: u32,
}

impl<const MIN: u32, const MAX: u32> HandleRange<MIN, MAX> {
    pub const MIN: u32 = MIN;
    pub const MAX: u32 = MAX;

    pub fn new(value: u32) -> Result<Self, ()> {
        if value >= MIN && value < MAX {
            Ok(Self { value })
        } else {
            Err(())
        }
    }
}

impl<const MIN: u32, const MAX: u32> TryFrom<u32> for HandleRange<MIN, MAX> {
    type Error = ();

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

impl<const MIN: u32, const MAX: u32> From<HandleRange<MIN, MAX>> for u32 {
    fn from(value: HandleRange<MIN, MAX>) -> Self {
        value.value
    }
}

impl<const MIN: u32, const MAX: u32> TryFrom<Handle> for HandleRange<MIN, MAX> {
    type Error = ();

    fn try_from(value: Handle) -> Result<Self, Self::Error> {
        u32::from(value).try_into()
    }
}

impl<const MIN: u32, const MAX: u32> From<HandleRange<MIN, MAX>> for Handle {
    fn from(value: HandleRange<MIN, MAX>) -> Self {
        // An u32 from a handle range can always be converted to a handle
        Handle::try_from(u32::from(value)).unwrap()
    }
}

impl<'de, const MIN: u32, const MAX: u32> Deserialize<'de> for HandleRange<MIN, MAX> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct HandleRangeVisitor<const MIN: u32, const MAX: u32>;
        impl<'de, const MIN: u32, const MAX: u32> Visitor<'de> for HandleRangeVisitor<MIN, MAX> {
            type Value = HandleRange<MIN, MAX>;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str(&format!("u32 handle"))
            }

            fn visit_u32<E>(self, v: u32) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                HandleRange::try_from(v)
                    .map_err(|_| E::invalid_value(serde::de::Unexpected::Unsigned(v.into()), &self))
            }
        }

        deserializer.deserialize_u32(HandleRangeVisitor::<MIN, MAX>)
    }
}

impl<const MIN: u32, const MAX: u32> Serialize for HandleRange<MIN, MAX> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_u32(u32::from(*self))
    }
}

pub type PCRHandle = HandleRange<0x00000000, 0x00000020>;
pub type NvIndexHandle = HandleRange<0x01000000, 0x02000000>;
pub type HmacOrLoadedSessionHandle = HandleRange<0x02000000, 0x02FFFFFF>;
pub type PolicyOrSavedSessionHandle = HandleRange<0x03000000, 0x03FFFFFF>;
pub type AuthHandle = HandleRange<0x40000010, 0x40000110>;
pub type ACTHandle = HandleRange<0x40000110, 0x40000120>;
pub type TransientHandle = HandleRange<0x80000000, 0x80FFFFFE>;
pub type PersistentHandle = HandleRange<0x81000000, 0x81FFFFFF>;
pub type AttachedComponentHandle = HandleRange<0x90000000, 0x90010000>;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SingleHandle<const HANDLE: u32>;

impl<const HANDLE: u32> SingleHandle<HANDLE> {
    pub const HANDLE: u32 = HANDLE;
}

impl<const HANDLE: u32> TryFrom<u32> for SingleHandle<HANDLE> {
    type Error = ();

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        if value == HANDLE {
            Ok(Self)
        } else {
            Err(())
        }
    }
}

impl<const HANDLE: u32> From<SingleHandle<HANDLE>> for u32 {
    fn from(_value: SingleHandle<HANDLE>) -> Self {
        HANDLE
    }
}

impl<const HANDLE: u32> TryFrom<Handle> for SingleHandle<HANDLE> {
    type Error = ();

    fn try_from(value: Handle) -> Result<Self, Self::Error> {
        u32::from(value).try_into()
    }
}

impl<const HANDLE: u32> From<SingleHandle<HANDLE>> for Handle {
    fn from(_value: SingleHandle<HANDLE>) -> Self {
        // An u32 from a valid SingleHandle can always be converted to a handle
        Handle::try_from(HANDLE).unwrap()
    }
}

impl<'de, const HANDLE: u32> Deserialize<'de> for SingleHandle<HANDLE> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct SingleHandleVisitor<const HANDLE: u32>;
        impl<'de, const HANDLE: u32> Visitor<'de> for SingleHandleVisitor<HANDLE> {
            type Value = SingleHandle<HANDLE>;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str(&format!("u32 handle"))
            }

            fn visit_u32<E>(self, v: u32) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                SingleHandle::try_from(v)
                    .map_err(|_| E::invalid_value(serde::de::Unexpected::Unsigned(v.into()), &self))
            }
        }

        deserializer.deserialize_u32(SingleHandleVisitor::<HANDLE>)
    }
}

impl<const HANDLE: u32> Serialize for SingleHandle<HANDLE> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_u32(u32::from(*self))
    }
}
