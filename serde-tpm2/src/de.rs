use crate::error;
use crate::log::Logger;
use error::{Error, Result};
use paste::paste;
use serde::de::{
    self, DeserializeSeed, EnumAccess, IntoDeserializer, MapAccess, SeqAccess, VariantAccess,
    Visitor,
};
use serde::Deserialize;
use std::{any, mem};

/// Starting point: https://serde.rs/impl-deserializer.html
pub struct Deserializer<'de> {
    // input data, and bytes are truncated off the beginning as data is parsed
    input: &'de [u8],
    logger: Logger,
    last_u8_u16_or_u32: u32,
}

impl<'de> Deserializer<'de> {
    pub fn from_bytes(input: &'de [u8]) -> Self {
        Deserializer {
            input,
            logger: Logger::new("deserializing".to_string()),
            last_u8_u16_or_u32: 0,
        }
    }
}

pub fn from_bytes<'a, T>(s: &'a [u8]) -> Result<T>
where
    T: Deserialize<'a>,
{
    let mut deserializer = Deserializer::from_bytes(s);
    let t = T::deserialize(&mut deserializer)?;
    if deserializer.input.is_empty() {
        Ok(t)
    } else {
        Err(Error::TrailingCharacters)
    }
}

macro_rules! define_parse {
    ($T:ty) => {
        paste! {
            /// Parse $T (u8, u16, ..., i8, i16, ...) from big-endian
            fn [<parse_ $T>] (&mut self) -> Result<$T> {
                let len = mem::size_of::<$T>();
                let buffer = self.input[..len].try_into().map_err(|_| Error::Eof)?;
                self.input = &self.input[len..];
                Ok($T::from_be_bytes(buffer))
            }
        }
    };
}

impl<'de> Deserializer<'de> {
    define_parse!(u8);
    define_parse!(u16);
    define_parse!(u32);
    define_parse!(u64);
    define_parse!(i8);
    define_parse!(i16);
    define_parse!(i32);
    define_parse!(i64);
}

impl<'de, 'a> de::Deserializer<'de> for &'a mut Deserializer<'de> {
    type Error = Error;

    /// This protocol is not self-describing: unimplemented
    fn deserialize_any<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_bool<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        let v = self.parse_u8()? != 0;
        self.logger.log_primitive(v);
        visitor.visit_bool(v)
    }

    fn deserialize_i8<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        let v = self.parse_i8()?;
        self.logger.log_primitive(v);
        visitor.visit_i8(v)
    }

    fn deserialize_i16<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        let v = self.parse_i16()?;
        self.logger.log_primitive(v);
        visitor.visit_i16(v)
    }

    fn deserialize_i32<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        let v = self.parse_i32()?;
        self.logger.log_primitive(v);
        visitor.visit_i32(v)
    }

    fn deserialize_i64<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        let v = self.parse_i64()?;
        self.logger.log_primitive(v);
        visitor.visit_i64(v)
    }

    fn deserialize_u8<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        let v = self.parse_u8()?;
        self.logger.log_primitive(v);
        self.last_u8_u16_or_u32 = v.into();
        visitor.visit_u8(v)
    }

    fn deserialize_u16<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        let v = self.parse_u16()?;
        self.logger.log_primitive(v);
        self.last_u8_u16_or_u32 = v.into();
        visitor.visit_u16(v)
    }

    fn deserialize_u32<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        let v = self.parse_u32()?;
        self.logger.log_primitive(v);
        self.last_u8_u16_or_u32 = v.into();
        visitor.visit_u32(v)
    }

    fn deserialize_u64<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        let v = self.parse_u64()?;
        self.logger.log_primitive(v);
        visitor.visit_u64(v)
    }

    fn deserialize_f32<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_f64<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_char<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_str<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_string<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_bytes<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_byte_buf<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_option<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        todo!()
    }

    // Anonymous value containing no data.
    fn deserialize_unit<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        self.logger.log(format_args!("deserialize_unit"));
        visitor.visit_unit()
    }

    // Named value containing no data.
    fn deserialize_unit_struct<V>(self, _name: &'static str, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        self.logger.log(format_args!("deserialize_unit_struct"));
        self.deserialize_unit(visitor)
    }

    // Treat newtype_struct as the type it wraps
    fn deserialize_newtype_struct<V>(self, _name: &'static str, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        self.logger.log(format_args!("deserialize_newtype_struct"));
        visitor.visit_newtype_struct(self)
    }

    // Called for array elements
    fn deserialize_seq<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        self.logger.log(format_args!("deserialize_seq"));

        self.logger.level_push();
        let value = visitor.visit_seq(VecElemAccess::new(
            self,
            self.last_u8_u16_or_u32.try_into().unwrap(),
        ))?;
        self.logger.level_pop();
        Ok(value)
    }

    // Called by deserialize_struct() and for Vec<T>
    fn deserialize_tuple<V>(self, len: usize, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        self.logger.log(format_args!("deserialize_tuple"));

        self.logger.level_push();
        let value = visitor.visit_seq(VecElemAccess::new(self, len))?;
        self.logger.level_pop();
        Ok(value)
    }

    fn deserialize_tuple_struct<V>(
        self,
        _name: &'static str,
        _len: usize,
        visitor: V,
    ) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        self.logger.log(format_args!("deserialize_tuple_struct"));
        self.deserialize_seq(visitor)
    }

    fn deserialize_map<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        struct EnumMapAccess<'a, 'de: 'a> {
            de: &'a mut Deserializer<'de>,
        }

        impl<'de, 'a> MapAccess<'de> for EnumMapAccess<'a, 'de> {
            type Error = Error;

            fn next_key_seed<K>(&mut self, seed: K) -> Result<Option<K::Value>>
            where
                K: DeserializeSeed<'de>,
            {
                self.de.logger.log(format_args!(
                    "----------- next_key_seed: {}",
                    any::type_name::<K::Value>()
                ));
                Ok(None)
            }

            fn next_value_seed<V>(&mut self, seed: V) -> Result<V::Value>
            where
                V: DeserializeSeed<'de>,
            {
                //let value = DeserializeSeed::deserialize(seed, &mut *self.de)?;
                //Ok(Some(value))
                self.de.logger.log(format_args!(
                    "----------- next_value_seed: {}",
                    any::type_name::<V::Value>()
                ));
                seed.deserialize(&mut *self.de)
            }
        }

        visitor.visit_map(EnumMapAccess { de: self })
    }

    fn deserialize_struct<V>(
        self,
        name: &'static str,
        fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        self.logger.log(format_args!("= struct {}", name));

        // For logging field names: deserialize_tuple does not know field names,
        // so we need to statefully store the field names in a HashMap
        self.logger.level_push();
        self.logger.set_field_names(fields);
        let value = visitor.visit_seq(StructAccess::new(self, fields.len()));
        self.logger.level_pop();

        value
    }

    // Part of a struct. Selector (i.e. discriminant) is always first member of
    // the struct (except in TPMS_ATTEST where it is comes after magic:
    // TPM_GENERATED). Selector is always TPMI/TPM_ALG/u16 (except in
    // TPMS_CAPABILITY_DATA where it is TPM_CAP/u32)
    fn deserialize_enum<V>(
        self,
        name: &'static str,
        variants: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        self.logger.log(format_args!("= enum {}", name));

        self.logger.level_push();
        self.logger.set_field_names(variants);
        let value = visitor.visit_enum(MyVariantAccess::new(self))?;
        self.logger.level_pop();

        Ok(value)
    }

    fn deserialize_identifier<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_ignored_any<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        self.deserialize_any(visitor)
    }
}

struct StructAccess<'a, 'de: 'a> {
    de: &'a mut Deserializer<'de>,
    index: usize, // TODO only for logging
}

impl<'a, 'de> StructAccess<'a, 'de> {
    fn new(de: &'a mut Deserializer<'de>, _len: usize) -> Self {
        StructAccess { de, index: 0 }
    }
}

impl<'de, 'a> SeqAccess<'de> for StructAccess<'a, 'de> {
    type Error = Error;

    fn next_element_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>>
    where
        T: DeserializeSeed<'de>,
    {
        let field_name = self.de.logger.get_field_names()[self.index];
        self.de.logger.log(format_args!(".{}", field_name));

        let value = DeserializeSeed::deserialize(seed, &mut *self.de)?;
        self.index += 1;
        Ok(Some(value))
    }
}

struct VecElemAccess<'a, 'de: 'a> {
    de: &'a mut Deserializer<'de>,
    index: usize,
    len: usize,
}

impl<'a, 'de> VecElemAccess<'a, 'de> {
    fn new(de: &'a mut Deserializer<'de>, len: usize) -> Self {
        VecElemAccess { de, len, index: 0 }
    }
}

impl<'de, 'a> SeqAccess<'de> for VecElemAccess<'a, 'de> {
    type Error = Error;

    fn next_element_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>>
    where
        T: DeserializeSeed<'de>,
    {
        if !(self.index < self.len) {
            return Ok(None);
        }

        self.de.logger.log(format_args!("element[{}]", self.index));

        let value = DeserializeSeed::deserialize(seed, &mut *self.de)?;
        self.index += 1;
        Ok(Some(value))
    }
}

struct MyVariantAccess<'a, 'de: 'a> {
    de: &'a mut Deserializer<'de>,
}

impl<'a, 'de> MyVariantAccess<'a, 'de> {
    fn new(de: &'a mut Deserializer<'de>) -> Self {
        MyVariantAccess { de }
    }
}

impl<'de, 'a> EnumAccess<'de> for MyVariantAccess<'a, 'de> {
    type Error = Error;
    type Variant = Self;

    /// Called to identify which variant to deserialize.
    fn variant_seed<V>(self, seed: V) -> Result<(V::Value, Self::Variant)>
    where
        V: DeserializeSeed<'de>,
    {
        let variant = self.de.parse_u16()?; // TODO what if selector is no u16
        self.de
            .logger
            .log(format_args!("discriminant = {} (u16)", variant));

        let value = seed.deserialize(variant.into_deserializer())?;
        Ok((value, self))
    }
}

impl<'de, 'a> VariantAccess<'de> for MyVariantAccess<'a, 'de> {
    type Error = Error;

    fn unit_variant(self) -> Result<()> {
        self.de.logger.log(format_args!("(unit_variant)"));
        Ok(())
    }

    fn newtype_variant_seed<T>(self, seed: T) -> Result<T::Value>
    where
        T: DeserializeSeed<'de>,
    {
        self.de.logger.log(format_args!("(newtype_variant_seed)"));
        seed.deserialize(self.de)
    }

    fn tuple_variant<V>(self, len: usize, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        self.de.logger.log(format_args!("(tuple_variant)"));
        de::Deserializer::deserialize_tuple(self.de, len, visitor)
    }

    fn struct_variant<V>(self, fields: &'static [&'static str], visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        self.de.logger.log(format_args!("(struct_variant)"));
        de::Deserializer::deserialize_struct(self.de, "", fields, visitor)
    }
}

////////////////////////////////////////////////////////////////////////////////

#[test]
fn test_struct() {
    #[derive(Deserialize, PartialEq, Debug)]
    struct MyStruct {
        int: i16,
        buffer: Vec<u8>,
    }

    let bytes = b"\xff\xee\x00\x04\xaa\xbb\xcc\xdd";
    let deserialized: MyStruct = from_bytes(bytes).unwrap();
    assert_eq!(
        deserialized,
        MyStruct {
            int: i16::from_be_bytes(*b"\xff\xee"),
            buffer: b"\xaa\xbb\xcc\xdd".to_vec(),
        }
    );
}

#[test]
fn test_enum() {
    #[derive(Deserialize, PartialEq, Debug)]
    #[repr(u16)]
    enum MyEnum {
        Unit = 0x1122,
        Newtype(u32) = 0x3344,
        Tuple(u32, u32) = 0x5566,
        Struct { field: u32 } = 0x7788,
    }

    // MyEnum::Unit
    let bytes = b"\x11\x22";
    let deserialized: MyEnum = from_bytes(bytes).unwrap();
    assert_eq!(deserialized, MyEnum::Unit);

    // MyEnum::Newtype
    let bytes = b"\x33\x44\xaa\xbb\xcc\xdd";
    let deserialized: MyEnum = from_bytes(bytes).unwrap();
    assert_eq!(
        deserialized,
        MyEnum::Newtype(u32::from_be_bytes(*b"\xaa\xbb\xcc\xdd"))
    );

    // MyEnum::Tuple
    let bytes = b"\x55\x66\xaa\xbb\xcc\xdd\xcc\xdd\xee\xff";
    let deserialized: MyEnum = from_bytes(bytes).unwrap();
    assert_eq!(
        deserialized,
        MyEnum::Tuple(
            u32::from_be_bytes(*b"\xaa\xbb\xcc\xdd"),
            u32::from_be_bytes(*b"\xcc\xdd\xee\xff"),
        )
    );

    // MyEnum::Struct
    let bytes = b"\x77\x88\xaa\xbb\xcc\xdd";
    let deserialized: MyEnum = from_bytes(bytes).unwrap();
    assert_eq!(
        deserialized,
        MyEnum::Struct {
            field: u32::from_be_bytes(*b"\xaa\xbb\xcc\xdd")
        }
    );
}
