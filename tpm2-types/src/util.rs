use serde::{de::Visitor, Deserialize, Deserializer, Serialize, Serializer};
use std::{fmt::Write, num::ParseIntError};

// TODO do we still need this?
pub fn from_hex(s: &str) -> Result<Vec<u8>, ParseIntError> {
    (0..s.len())
        .step_by(2)
        .map(|i| u8::from_str_radix(&s[i..i + 2], 16))
        .collect()
}

pub fn to_hex(bytes: &[u8]) -> String {
    let mut s = String::with_capacity(bytes.len() * 2);
    for &b in bytes {
        write!(&mut s, "{:02x}", b).unwrap();
    }
    s
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ConstantU32<const VALUE: u32>;

impl<const VALUE: u32> ConstantU32<VALUE> {
    pub const VALUE: u32 = VALUE;
}

impl<const VALUE: u32> TryFrom<u32> for ConstantU32<VALUE> {
    type Error = ();

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        if value == VALUE {
            Ok(Self)
        } else {
            Err(())
        }
    }
}

impl<const VALUE: u32> From<ConstantU32<VALUE>> for u32 {
    fn from(_value: ConstantU32<VALUE>) -> Self {
        VALUE
    }
}

impl<'de, const VALUE: u32> Deserialize<'de> for ConstantU32<VALUE> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct SingleHandleVisitor<const VALUE: u32>;
        impl<'de, const VALUE: u32> Visitor<'de> for SingleHandleVisitor<VALUE> {
            type Value = ConstantU32<VALUE>;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("u32 value")
            }

            fn visit_u32<E>(self, v: u32) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                ConstantU32::try_from(v)
                    .map_err(|_| E::invalid_value(serde::de::Unexpected::Unsigned(v.into()), &self))
            }
        }

        deserializer.deserialize_u32(SingleHandleVisitor::<VALUE>)
    }
}

impl<const VALUE: u32> Serialize for ConstantU32<VALUE> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_u32(u32::from(*self))
    }
}
