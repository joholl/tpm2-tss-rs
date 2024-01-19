use crate::error;
use crate::log::Logger;
use error::{Error, Result};
use paste::paste;
use serde::de::{
    self, DeserializeSeed, EnumAccess, EnumVariantReprs, IntoDeserializer, SeqAccess,
    VariantAccess, Visitor,
};
use serde::Deserialize;
use std::mem;

/// Starting point: https://serde.rs/impl-deserializer.html
pub struct Deserializer<'de> {
    // input data, and bytes are truncated off the beginning as data is parsed
    input: &'de [u8],
    logger: Logger,
    last_u8_u16_or_u32: Option<u32>,
}

impl<'de> Deserializer<'de> {
    pub fn from_bytes(input: &'de [u8]) -> Self {
        Deserializer {
            input,
            logger: Logger::new("deserializing".to_string()),
            last_u8_u16_or_u32: None,
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
    define_parse!(u128);
    define_parse!(i8);
    define_parse!(i16);
    define_parse!(i32);
    define_parse!(i64);
    define_parse!(i128);
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

    fn deserialize_i128<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        let v = self.parse_i128()?;
        self.logger.log_primitive(v);
        visitor.visit_i128(v)
    }

    fn deserialize_u8<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        let v = self.parse_u8()?;
        self.logger.log_primitive(v);
        self.last_u8_u16_or_u32 = Some(v.into());
        visitor.visit_u8(v)
    }

    fn deserialize_u16<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        let v = self.parse_u16()?;
        self.logger.log_primitive(v);
        self.last_u8_u16_or_u32 = Some(v.into());
        visitor.visit_u16(v)
    }

    fn deserialize_u32<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        let v = self.parse_u32()?;
        self.logger.log_primitive(v);
        self.last_u8_u16_or_u32 = Some(v);
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

    fn deserialize_u128<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        let v = self.parse_u128()?;
        self.logger.log_primitive(v);
        visitor.visit_u128(v)
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

        let len = self.last_u8_u16_or_u32
            .ok_or(Error::Message("Error: deserializing sequence (e.g. Vec<_>) without having deserialized u8/u16/u32 before".to_string()))?
            .try_into()
            .expect("Could not cast u32 to usize");

        self.logger.level_push();
        let value = visitor.visit_seq(VecElemAccess::new(self, len))?;
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
        name: &'static str,
        len: usize,
        visitor: V,
    ) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        self.logger
            .log(format_args!("deserialize_tuple_struct {}", name));

        self.logger.level_push();
        let value = visitor.visit_seq(VecElemAccess::new(self, len))?;
        self.logger.level_pop();
        Ok(value)
    }

    fn deserialize_map<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        // Map is not needed for the TPM structures. In theory, this could be
        // implemented like sequences with assuming a parsed length and then
        // using a MapAccess visitor to parse keys/values.
        unimplemented!()
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
        _name: &'static str,
        _variants: &'static [&'static str],
        _visitor: V,
    ) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_enum_repr<V>(
        self,
        name: &'static str,
        variants: &'static EnumVariantReprs,
        visitor: V,
    ) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        self.logger.log(format_args!("= enum {}", name));

        self.logger.level_push();
        self.logger.set_field_names(variants.str_variants());
        let value = visitor.visit_enum(MyVariantAccess::new(self, variants))?;
        self.logger.level_pop();

        Ok(value)
    }

    fn deserialize_identifier<V>(self, _visitor: V) -> Result<V::Value>
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
        if self.index >= self.len {
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
    variants: &'static EnumVariantReprs,
}

impl<'a, 'de> MyVariantAccess<'a, 'de> {
    fn new(de: &'a mut Deserializer<'de>, variants: &'static EnumVariantReprs) -> Self {
        MyVariantAccess { de, variants }
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
        let variant = match self.variants {
            EnumVariantReprs::U8(vars, _) => {
                let discriminant = self.de.parse_u8()?;
                vars.iter()
                    .position(|v| *v == discriminant)
                    .ok_or(discriminant as u64)
            }
            EnumVariantReprs::U16(vars, _) => {
                let discriminant = self.de.parse_u16()?;
                vars.iter()
                    .position(|v| *v == discriminant)
                    .ok_or(discriminant as u64)
            }
            EnumVariantReprs::U32(vars, _) => {
                let discriminant = self.de.parse_u32()?;
                vars.iter()
                    .position(|v| *v == discriminant)
                    .ok_or(discriminant as u64)
            }
            EnumVariantReprs::U64(vars, _) => {
                let discriminant = self.de.parse_u64()?;
                vars.iter()
                    .position(|v| *v == discriminant)
                    .ok_or(discriminant as u64)
            }
            EnumVariantReprs::I8(vars, _) => {
                let discriminant = self.de.parse_i8()?;
                vars.iter()
                    .position(|v| *v == discriminant)
                    .ok_or(discriminant as u64)
            }
            EnumVariantReprs::I16(vars, _) => {
                let discriminant = self.de.parse_i16()?;
                vars.iter()
                    .position(|v| *v == discriminant)
                    .ok_or(discriminant as u64)
            }
            EnumVariantReprs::I32(vars, _) => {
                let discriminant = self.de.parse_i32()?;
                vars.iter()
                    .position(|v| *v == discriminant)
                    .ok_or(discriminant as u64)
            }
            EnumVariantReprs::I64(vars, _) => {
                let discriminant = self.de.parse_i64()?;
                vars.iter()
                    .position(|v| *v == discriminant)
                    .ok_or(discriminant as u64)
            }
        };
        match variant {
            Ok(variant) => {
                self.de
                    .logger
                    .log(format_args!("discriminant = {}", variant));

                let value = seed.deserialize(variant.into_deserializer())?;
                Ok((value, self))
            }
            Err(discriminant) => {
                self.de.logger.log(format_args!(
                    "[ERROR] nvalid discriminant = {}",
                    discriminant
                ));
                Err(Error::ExpectedEnum)
            }
        }
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
fn test_bool() {
    assert_eq!(from_bytes::<bool>(b"\x00").unwrap(), false);
    assert_eq!(from_bytes::<bool>(b"\x01").unwrap(), true);
    assert_eq!(from_bytes::<bool>(b"\xff").unwrap(), true);
}

#[test]
fn test_u8_i8() {
    assert_eq!(from_bytes::<u8>(b"\x00").unwrap(), 0);
    assert_eq!(from_bytes::<u8>(b"\x01").unwrap(), 1);
    assert_eq!(from_bytes::<u8>(b"\x42").unwrap(), 0x42);
    assert_eq!(from_bytes::<u8>(b"\xff").unwrap(), 0xff);

    assert_eq!(from_bytes::<i8>(b"\x00").unwrap(), 0);
    assert_eq!(from_bytes::<i8>(b"\x01").unwrap(), 1);
    assert_eq!(from_bytes::<i8>(b"\xfe").unwrap(), -2);
    assert_eq!(from_bytes::<i8>(b"\xff").unwrap(), -1);
}

#[test]
fn test_u16_i16() {
    let bytes = [b"\x00\x00", b"\xde\xad", b"\xff\xff"];
    for b in bytes {
        assert_eq!(from_bytes::<u16>(b).unwrap(), u16::from_be_bytes(*b));
        assert_eq!(from_bytes::<i16>(b).unwrap(), i16::from_be_bytes(*b));
    }
}

#[test]
fn test_u32_i32() {
    let bytes = [
        b"\x00\x00\x00\x00",
        b"\xde\xad\xbe\xef",
        b"\xff\xff\xff\xff",
    ];
    for b in bytes {
        assert_eq!(from_bytes::<u32>(b).unwrap(), u32::from_be_bytes(*b));
        assert_eq!(from_bytes::<i32>(b).unwrap(), i32::from_be_bytes(*b));
    }
}

#[test]
fn test_u64_i64() {
    let bytes = [
        b"\x00\x00\x00\x00\x00\x00\x00\x00",
        b"\xde\xad\xbe\xef\xc0\xff\xeb\xad",
        b"\xff\xff\xff\xff\xff\xff\xff\xff",
    ];
    for b in bytes {
        assert_eq!(from_bytes::<u64>(b).unwrap(), u64::from_be_bytes(*b));
        assert_eq!(from_bytes::<i64>(b).unwrap(), i64::from_be_bytes(*b));
    }
}

#[test]
fn test_u128_i128() {
    let bytes = [
        b"\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00",
        b"\xde\xad\xbe\xef\xc0\xff\xeb\xad\xde\xad\xbe\xef\xc0\xff\xeb\xad",
        b"\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff\xff",
    ];
    for b in bytes {
        assert_eq!(from_bytes::<u128>(b).unwrap(), u128::from_be_bytes(*b));
        assert_eq!(from_bytes::<i128>(b).unwrap(), i128::from_be_bytes(*b));
    }
}

#[test]
#[should_panic]
fn test_string() {
    // unimplemented

    assert_eq!(
        from_bytes::<String>(b"\x48\x65\x6c\x6c\x6f\x20\x57\x6f\x72\x6c\x64\x21").unwrap(),
        "Hello World!"
    );
}

#[test]
fn test_byte_array() {
    assert_eq!(
        from_bytes::<[u8; 12]>(b"\x48\x65\x6c\x6c\x6f\x20\x57\x6f\x72\x6c\x64\x21").unwrap(),
        *b"Hello World!"
    );
    assert_eq!(from_bytes::<[u8; 0]>(b"").unwrap(), []);
    let bytes = b"\x00\x01\x02\x03\x04\x05\x06\x07\x08\x09\x0a\x0b\x0c\x0d\x0e\x0f\x10\x11\x12\x13\x14\x15\x16\x17\x18\x19\x1a\x1b\x1c\x1d\x1e\x1f";
    assert_eq!(from_bytes::<[u8; 32]>(bytes).unwrap(), *bytes);
    // Arrays with .len() > 32 are not supported by serde as of now and need to be handled elsewhere
}

#[test]
#[should_panic]
fn test_option() {
    // unimplemented

    // We have to rely on outside information (like in sequences) if we have an
    // Some(_) or None.

    // TODO maybe map e.g. Alg::Null to None etc.
    assert_eq!(
        from_bytes::<Option<u16>>(b"\x11\x11").unwrap(),
        Some(0x1111)
    );

    assert_eq!(from_bytes::<Option<u16>>(b"").unwrap(), None);
}

#[test]
fn test_unit() {
    assert_eq!(from_bytes::<()>(b"").unwrap(), ());
}

#[test]
fn test_unit_struct() {
    use std::marker::PhantomData;

    #[derive(Deserialize, PartialEq, Debug)]
    struct MyStruct;

    assert_eq!(from_bytes::<MyStruct>(b"").unwrap(), MyStruct);
    assert_eq!(
        from_bytes::<PhantomData<u128>>(b"").unwrap(),
        PhantomData::<u128>
    );
}

#[test]
fn test_newtype_struct() {
    #[derive(Deserialize, PartialEq, Debug)]
    struct MyStruct(u32);

    let bytes = b"\xde\xad\xbe\xef";
    assert_eq!(
        from_bytes::<MyStruct>(bytes).unwrap(),
        MyStruct(u32::from_be_bytes(*bytes))
    );
}

#[test]
fn test_seq() {
    // sequences rely on deserializing a u8/u16/u32 as len before

    assert_eq!(
        from_bytes::<(u8, Vec<u8>)>(b"\x03\x00\x01\x02").unwrap(),
        (3, vec![0x00, 0x01, 0x02])
    );
    assert_eq!(
        from_bytes::<(u16, Vec<u16>)>(b"\x00\x02\xee\xee\xff\xff").unwrap(),
        (2, vec![0xeeee, 0xffff])
    );
    assert_eq!(
        from_bytes::<(u32, Vec<u8>)>(b"\x00\x00\x00\x00").unwrap(),
        (0, vec![])
    );

    // try deserializing standalone seq without len
    assert!(from_bytes::<Vec<u8>>(b"\x00\x01\x02").is_err());
}

#[test]
fn test_tuple() {
    assert_eq!(
        from_bytes::<(u8, u8, u8)>(b"\x00\x01\x02").unwrap(),
        (0x00, 0x01, 0x02)
    );
    assert_eq!(
        from_bytes::<(i8, i16, i32)>(b"\x00\x11\x11\x22\x22\x22\x22").unwrap(),
        (0x00, 0x1111, 0x22222222)
    );
    assert_eq!(
        from_bytes::<(u16, Vec<u16>)>(b"\x00\x02\xee\xee\xff\xff").unwrap(),
        (2, vec![0xeeee, 0xffff])
    );
}

#[test]
fn test_tuple_struct() {
    #[derive(Deserialize, PartialEq, Debug)]
    struct MyStruct(u8, u16);

    let bytes = b"\x00\x11\x11";
    assert_eq!(
        from_bytes::<MyStruct>(bytes).unwrap(),
        MyStruct(0x00, 0x1111)
    );
}

#[test]
#[should_panic]
fn test_map() {
    // unimplemented

    use std::collections::BTreeMap;

    // maps would have to rely on deserializing a u8/u16/u32 as len before
    assert_eq!(
        from_bytes::<(u8, BTreeMap::<u8, u16>)>(b"\x02\x01\x11\x11\x02\x22\x22").unwrap(),
        (
            0x02,
            [(1, 0x1111), (2, 0x2222)]
                .iter()
                .cloned()
                .collect::<BTreeMap<_, _>>()
        )
    );
}

#[test]
fn test_struct() {
    #[derive(Deserialize, PartialEq, Debug)]
    struct MyStruct {
        a: u16,
        len: u8,
        vec: Vec<u16>,
        b: u16,
    }

    let bytes = b"\xff\xff\x02\x11\x11\x22\x22\xff\xff";
    let deserialized: MyStruct = from_bytes(bytes).unwrap();
    assert_eq!(
        deserialized,
        MyStruct {
            a: 0xffff,
            len: 2,
            vec: vec![0x1111, 0x2222],
            b: 0xffff,
        }
    );
}

#[test]
fn test_enum() {
    #[derive(Deserialize, PartialEq, Debug)]
    #[repr(u16)]
    enum MyEnum {
        Unit = 0x1111,
        Newtype(u16) = 0x2222,
        Tuple(u8, u16) = 0x3333,
        Struct { field: u16 } = 0x4444,
    }

    // MyEnum::Unit
    let bytes = b"\x11\x11";
    assert_eq!(from_bytes::<MyEnum>(bytes).unwrap(), MyEnum::Unit);

    // MyEnum::Newtype
    let bytes = b"\x22\x22\x11\x11";
    assert_eq!(
        from_bytes::<MyEnum>(bytes).unwrap(),
        MyEnum::Newtype(0x1111)
    );

    // MyEnum::Tuple
    let bytes = b"\x33\x33\x11\x22\x22";
    assert_eq!(
        from_bytes::<MyEnum>(bytes).unwrap(),
        MyEnum::Tuple(0x11, 0x2222)
    );

    // MyEnum::Struct
    let bytes = b"\x44\x44\x11\x11";
    assert_eq!(
        from_bytes::<MyEnum>(bytes).unwrap(),
        MyEnum::Struct { field: 0x1111 }
    );
}
