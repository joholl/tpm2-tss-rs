use std::{any, fmt, marker};

use serde::ser::SerializeTuple;
use serde::{de, Deserializer, Serialize, Serializer};
use serde::{
    de::{MapAccess, SeqAccess, Visitor},
    Deserialize,
};
use serde_tpm2::de::from_bytes;

/// Wrapper for structs which are prepended by a size (e.g. TPM2B_PUBLIC which
/// wraps TPMT_PUBLIC). The outer type is distinct from the inner type because
/// their .size() evaluate differently.
///
/// If Inner implements Deserialize, this type does, too.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct StructWithSize<Inner>(pub Inner);

impl<'de, Inner> StructWithSize<Inner> {
    fn new(inner: Inner) -> Self {
        Self(inner)
    }
}

pub struct StructWithSizeVisitor<Outer> {
    outer: marker::PhantomData<Outer>,
}

impl<Outer> StructWithSizeVisitor<Outer> {
    pub fn new() -> StructWithSizeVisitor<Outer> {
        StructWithSizeVisitor {
            outer: marker::PhantomData,
        }
    }
}

impl<'de, Inner> Visitor<'de> for StructWithSizeVisitor<StructWithSize<Inner>>
where
    Inner: Deserialize<'de>,
{
    type Value = StructWithSize<Inner>;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        let type_name = any::type_name::<Self::Value>();
        let expecting = format!("struct {}", type_name);
        formatter.write_str(&expecting)
    }

    fn visit_seq<V>(self, mut seq: V) -> Result<Self::Value, V::Error>
    where
        V: SeqAccess<'de>,
    {
        let _size: u16 = seq
            .next_element()?
            .ok_or_else(|| de::Error::invalid_length(0, &self))?;
        let inner: Inner = seq
            .next_element()?
            .ok_or_else(|| de::Error::invalid_length(1, &self))?;
        Ok(Self::Value::new(inner))
    }

    fn visit_map<V>(self, mut _map: V) -> Result<Self::Value, V::Error>
    where
        V: MapAccess<'de>,
    {
        unimplemented!()
    }
}

impl<'de, Inner: Deserialize<'de>> Deserialize<'de> for StructWithSize<Inner> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_tuple(2, StructWithSizeVisitor::<StructWithSize<Inner>>::new())
    }
}

impl<Inner: Serialize> Serialize for StructWithSize<Inner> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        // tuple consumes u16 first, and then inner
        let mut tup = serializer.serialize_tuple(1)?;
        tup.serialize_element(&self.0)?;
        tup.end()
    }
}

#[test]
fn test_enum() {
    #[derive(Deserialize, PartialEq, Debug)]
    struct InnerStruct {
        foo: u8,
        bar: u16,
    }

    type OuterStruct = StructWithSize<InnerStruct>;

    let outer_struct = StructWithSize(InnerStruct {
        foo: 0xff,
        bar: u16::from_be_bytes(*b"\xaa\xbb"),
    });
    let bytes = b"\x00\x03\xff\xaa\xbb";
    let deserialized: OuterStruct = from_bytes(bytes).unwrap();
    assert_eq!(deserialized, outer_struct);
}
