use crate::{
    error::{Error, Result},
    log::Logger,
};
use log;
use serde::{
    ser::{self, Impossible},
    Serialize,
};
use std::any;

/// Starting point: https://serde.rs/impl-serializer.html
pub struct Serializer {
    // This byte string starts empty and is appended as values are serialized.
    output: Vec<u8>,
    logger: Logger,
}

impl Serializer {
    // TODO remove
    fn indent(&self) -> usize {
        0
    }
}

pub fn to_bytes<T>(value: &T) -> Result<Vec<u8>>
where
    T: Serialize,
{
    let mut serializer = Serializer {
        output: Vec::new(),
        logger: Logger::new("serializing".to_string()),
    };
    value.serialize(&mut serializer)?;
    Ok(serializer.output)
}

impl<'a> ser::Serializer for &'a mut Serializer {
    type Ok = ();
    type Error = Error;

    type SerializeSeq = Self;
    type SerializeTuple = Self;
    type SerializeTupleStruct = Self;
    type SerializeTupleVariant = Self;
    type SerializeMap = Impossible<Self::Ok, Self::Error>;
    type SerializeStruct = Self;
    type SerializeStructVariant = Self;

    fn serialize_bool(self, v: bool) -> Result<()> {
        let v = v as u8;
        self.logger.log_primitive(v);
        self.serialize_u8(v)
    }

    fn serialize_i8(self, v: i8) -> Result<()> {
        self.logger.log_primitive(v);
        self.output.append(&mut v.to_be_bytes().to_vec());
        Ok(())
    }

    fn serialize_i16(self, v: i16) -> Result<()> {
        self.logger.log_primitive(v);
        self.output.append(&mut v.to_be_bytes().to_vec());
        Ok(())
    }

    fn serialize_i32(self, v: i32) -> Result<()> {
        self.logger.log_primitive(v);
        self.output.append(&mut v.to_be_bytes().to_vec());
        Ok(())
    }

    fn serialize_i64(self, v: i64) -> Result<()> {
        self.logger.log_primitive(v);
        self.output.append(&mut v.to_be_bytes().to_vec());
        Ok(())
    }

    fn serialize_i128(self, v: i128) -> Result<()> {
        self.logger.log_primitive(v);
        self.output.append(&mut v.to_be_bytes().to_vec());
        Ok(())
    }

    fn serialize_u8(self, v: u8) -> Result<()> {
        self.logger.log_primitive(v);
        self.output.append(&mut v.to_be_bytes().to_vec());
        Ok(())
    }

    fn serialize_u16(self, v: u16) -> Result<()> {
        self.logger.log_primitive(v);
        self.output.append(&mut v.to_be_bytes().to_vec());
        Ok(())
    }

    fn serialize_u32(self, v: u32) -> Result<()> {
        self.logger.log_primitive(v);
        self.output.append(&mut v.to_be_bytes().to_vec());
        Ok(())
    }

    fn serialize_u64(self, v: u64) -> Result<()> {
        self.logger.log_primitive(v);
        self.output.append(&mut v.to_be_bytes().to_vec());
        Ok(())
    }

    fn serialize_u128(self, v: u128) -> Result<()> {
        self.logger.log_primitive(v);
        self.output.append(&mut v.to_be_bytes().to_vec());
        Ok(())
    }

    fn serialize_f32(self, _v: f32) -> Result<()> {
        unimplemented!()
    }

    fn serialize_f64(self, _v: f64) -> Result<()> {
        unimplemented!()
    }

    fn serialize_char(self, _v: char) -> Result<()> {
        // let v = v as u8;
        // self.serialize_u8(v)

        unimplemented!()
    }

    fn serialize_str(self, _v: &str) -> Result<()> {
        unimplemented!()
    }

    fn serialize_bytes(self, _v: &[u8]) -> Result<()> {
        // use serde::ser::SerializeSeq;
        // self.logger.log(format_args!("&[u8; {}]", v.len()));
        // let mut seq = self.serialize_seq(Some(v.len()))?;
        // for byte in v {
        //     log::info!("    {:02x}", byte);
        //     seq.serialize_element(byte)?;
        // }
        // seq.end()

        unimplemented!()
    }

    fn serialize_none(self) -> Result<()> {
        // For deserialization, we have to rely on outside information (like in
        // sequences) if we have an Some(_) or None.

        // log::info!("serializing {:i$}None", "", i = self.indent());
        // Ok(())

        unimplemented!()
    }

    fn serialize_some<T>(self, _value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        // For deserialization, we have to rely on outside information (like in
        // sequences) if we have an Some(_) or None.

        // // TODO is this needed?
        // log::info!("serializing {:i$}Some(...)", "", i = self.indent());
        // //self.level_push();
        // let result = value.serialize(self);
        // //self.level_pop();
        // result

        unimplemented!()
    }

    fn serialize_unit(self) -> Result<()> {
        log::info!("serializing {:i$}unit", "", i = self.indent());
        Ok(())
    }

    fn serialize_unit_struct(self, name: &'static str) -> Result<()> {
        log::info!(
            "serializing {:i$}unit struct {}",
            "",
            name,
            i = self.indent()
        );
        Ok(())
    }

    fn serialize_newtype_struct<T>(self, name: &'static str, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        log::info!(
            "serializing {:i$}new type struct {}",
            "",
            name,
            i = self.indent()
        );
        value.serialize(self)
    }

    fn serialize_seq(self, _len: Option<usize>) -> Result<Self::SerializeSeq> {
        self.logger.level_push();
        Ok(self)
    }

    fn serialize_tuple(self, len: usize) -> Result<Self::SerializeTuple> {
        self.logger.log(format_args!("tuple[{}]", len));
        self.logger.level_push();
        Ok(self)
    }

    fn serialize_tuple_struct(
        self,
        name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleStruct> {
        self.logger
            .log(format_args!("enum variant: tuple_struct {}{}", name, len));
        self.serialize_seq(Some(len))
    }

    fn serialize_unit_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
    ) -> Result<()> {
        unimplemented!()
    }

    fn serialize_unit_variant_repr(
        self,
        name: &'static str,
        _variant_index: u32,
        variant: ser::EnumReprVariant,
    ) -> std::prelude::v1::Result<Self::Ok, Self::Error> {
        self.logger.log(format_args!("enum variant: unit {}", name));
        self.serialize_enum_repr_as_int(variant)
    }

    fn serialize_newtype_variant<T>(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _value: &T,
    ) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        unimplemented!()
    }

    fn serialize_newtype_variant_repr<T: ?Sized>(
        self,
        name: &'static str,
        _variant_index: u32,
        variant: ser::EnumReprVariant,
        value: &T,
    ) -> std::prelude::v1::Result<Self::Ok, Self::Error>
    where
        T: Serialize,
    {
        self.logger
            .log(format_args!("enum variant: newtype {}", name));
        self.serialize_enum_repr_as_int(variant)?;
        value.serialize(&mut *self)
    }

    fn serialize_tuple_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleVariant> {
        unimplemented!()
    }

    fn serialize_tuple_variant_repr(
        self,
        name: &'static str,
        _variant_index: u32,
        variant: ser::EnumReprVariant,
        len: usize,
    ) -> std::prelude::v1::Result<Self::SerializeTupleVariant, Self::Error> {
        self.logger
            .log(format_args!("enum variant: tuple {}[{}]", name, len));
        self.logger.level_push();
        self.serialize_enum_repr_as_int(variant)?;
        Ok(self)
    }

    fn serialize_struct_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStructVariant> {
        unimplemented!()
    }

    fn serialize_struct_variant_repr(
        self,
        name: &'static str,
        _variant_index: u32,
        variant: ser::EnumReprVariant,
        _len: usize,
    ) -> std::prelude::v1::Result<Self::SerializeStructVariant, Self::Error> {
        self.logger
            .log(format_args!("enum variant: struct {}", name));
        self.logger.level_push();
        self.serialize_enum_repr_as_int(variant)?;
        Ok(self)
    }

    fn serialize_map(self, len: Option<usize>) -> Result<Self::SerializeMap> {
        log::info!("serializing {:i$}map[{:?}]", "", len, i = self.indent());
        unimplemented!()
    }

    fn serialize_struct(self, name: &'static str, _len: usize) -> Result<Self::SerializeStruct> {
        self.logger.log(format_args!(" = struct {}", name));
        self.logger.level_push();
        Ok(self)
    }
}

// TODO new struct
impl<'a> ser::SerializeSeq for &'a mut Serializer {
    type Ok = ();
    type Error = Error;

    fn serialize_element<T>(&mut self, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        self.logger
            .log(format_args!("seq element[...] ({})", any::type_name::<T>()));
        value.serialize(&mut **self)
    }

    fn end(self) -> Result<()> {
        self.logger.level_pop();
        Ok(())
    }
}

impl<'a> ser::SerializeTuple for &'a mut Serializer {
    type Ok = ();
    type Error = Error;

    fn serialize_element<T>(&mut self, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        self.logger.log(format_args!(
            "tuple element[...] ({})",
            any::type_name::<T>()
        ));
        value.serialize(&mut **self)
    }

    fn end(self) -> Result<()> {
        self.logger.level_pop();
        Ok(())
    }
}

impl<'a> ser::SerializeTupleStruct for &'a mut Serializer {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T>(&mut self, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        self.logger.log(format_args!(
            "tuple_struct element[...] ({})",
            any::type_name::<T>()
        ));
        value.serialize(&mut **self)
    }

    fn end(self) -> Result<()> {
        self.logger.level_pop();
        Ok(())
    }
}

impl<'a> ser::SerializeTupleVariant for &'a mut Serializer {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T>(&mut self, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        self.logger.log(format_args!(
            "tuple_variant element[...] ({})",
            any::type_name::<T>()
        ));
        value.serialize(&mut **self)
    }

    fn end(self) -> Result<()> {
        self.logger.level_pop();
        Ok(())
    }
}

impl<'a> ser::SerializeStruct for &'a mut Serializer {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T>(&mut self, key: &'static str, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        self.logger
            .log(format_args!(".{} ({})", key, any::type_name::<T>()));
        value.serialize(&mut **self)
    }

    fn end(self) -> Result<()> {
        self.logger.level_pop();
        Ok(())
    }
}

impl<'a> ser::SerializeStructVariant for &'a mut Serializer {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T>(&mut self, key: &'static str, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        self.logger
            .log(format_args!(".{} ({})", key, any::type_name::<T>()));
        value.serialize(&mut **self)
    }

    fn end(self) -> Result<()> {
        Ok(())
    }
}

////////////////////////////////////////////////////////////////////////////////

#[test]
fn test_bool() {
    assert_eq!(to_bytes(&false).unwrap(), b"\x00");
    assert_eq!(to_bytes(&true).unwrap(), b"\x01");
}

#[test]
fn test_u8_i8() {
    assert_eq!(to_bytes::<u8>(&0).unwrap(), b"\x00");
    assert_eq!(to_bytes::<u8>(&1).unwrap(), b"\x01");
    assert_eq!(to_bytes::<u8>(&0x42).unwrap(), b"\x42");
    assert_eq!(to_bytes::<u8>(&0xff).unwrap(), b"\xff");

    assert_eq!(to_bytes::<i8>(&0).unwrap(), b"\x00");
    assert_eq!(to_bytes::<i8>(&1).unwrap(), b"\x01");
    assert_eq!(to_bytes::<i8>(&-2).unwrap(), b"\xfe");
    assert_eq!(to_bytes::<i8>(&-1).unwrap(), b"\xff");
}

#[test]
fn test_u16_i16() {
    let bytes = [b"\x00\x00", b"\xde\xad", b"\xff\xff"];
    for b in bytes {
        assert_eq!(to_bytes(&u16::from_be_bytes(*b)).unwrap(), b);
        assert_eq!(to_bytes(&i16::from_be_bytes(*b)).unwrap(), b);
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
        assert_eq!(to_bytes(&u32::from_be_bytes(*b)).unwrap(), b);
        assert_eq!(to_bytes(&i32::from_be_bytes(*b)).unwrap(), b);
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
        assert_eq!(to_bytes(&u64::from_be_bytes(*b)).unwrap(), b);
        assert_eq!(to_bytes(&i64::from_be_bytes(*b)).unwrap(), b);
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
        assert_eq!(to_bytes(&u128::from_be_bytes(*b)).unwrap(), b);
        assert_eq!(to_bytes(&i128::from_be_bytes(*b)).unwrap(), b);
    }
}

#[test]
#[should_panic]
fn test_string() {
    // unimplemented

    assert_eq!(
        to_bytes(&"Hello World!".to_string()).unwrap(),
        b"\x48\x65\x6c\x6c\x6f\x20\x57\x6f\x72\x6c\x64\x21"
    );
}

#[test]
fn test_byte_array() {
    assert_eq!(
        to_bytes::<[u8; 12]>(b"Hello World!").unwrap(),
        b"\x48\x65\x6c\x6c\x6f\x20\x57\x6f\x72\x6c\x64\x21"
    );
    assert_eq!(to_bytes::<[u8; 0]>(b"").unwrap(), []);
    let bytes = b"\x00\x01\x02\x03\x04\x05\x06\x07\x08\x09\x0a\x0b\x0c\x0d\x0e\x0f\x10\x11\x12\x13\x14\x15\x16\x17\x18\x19\x1a\x1b\x1c\x1d\x1e\x1f";
    assert_eq!(to_bytes::<[u8; 32]>(bytes).unwrap(), bytes);
    // Arrays with .len() > 32 are not supported by serde as of now and need to be handled elsewhere
}

#[test]
#[should_panic]
fn test_option() {
    // unimplemented

    // For deserialization, we have to rely on outside information (like in
    // sequences) if we have an Some(_) or None.

    // TODO maybe map e.g. Alg::Null to None etc.
    assert_eq!(to_bytes::<Option<u16>>(&Some(0x1111)).unwrap(), b"\x11\x11");
    assert_eq!(to_bytes::<Option<u16>>(&None).unwrap(), b"");
}

#[test]
fn test_unit() {
    assert_eq!(to_bytes::<()>(&()).unwrap(), b"");
}

#[test]
fn test_unit_struct() {
    use std::marker::PhantomData;

    #[derive(Serialize, PartialEq, Debug)]
    struct MyStruct;

    assert_eq!(to_bytes(&MyStruct).unwrap(), b"");
    assert_eq!(to_bytes(&PhantomData::<u128>).unwrap(), b"");
}

#[test]
fn test_newtype_struct() {
    #[derive(Serialize, PartialEq, Debug)]
    struct MyStruct(u32);

    let bytes = b"\xde\xad\xbe\xef";
    assert_eq!(
        to_bytes(&MyStruct(u32::from_be_bytes(*bytes))).unwrap(),
        bytes
    );
}

#[test]
fn test_seq() {
    assert_eq!(
        to_bytes::<Vec<u8>>(&vec![0x00, 0x01, 0x02]).unwrap(),
        b"\x00\x01\x02"
    );
    assert_eq!(
        to_bytes::<Vec<u16>>(&vec![0xeeee, 0xffff]).unwrap(),
        b"\xee\xee\xff\xff"
    );
    assert_eq!(to_bytes::<Vec<u8>>(&vec![]).unwrap(), b"");
}

#[test]
fn test_tuple() {
    assert_eq!(
        to_bytes::<(u8, u8, u8)>(&(0x00, 0x01, 0x02)).unwrap(),
        b"\x00\x01\x02"
    );
    assert_eq!(
        to_bytes::<(i8, i16, i32)>(&(0x00, 0x1111, 0x22222222)).unwrap(),
        b"\x00\x11\x11\x22\x22\x22\x22"
    );
    assert_eq!(
        to_bytes::<(u16, Vec<u16>)>(&(2, vec![0xeeee, 0xffff])).unwrap(),
        b"\x00\x02\xee\xee\xff\xff"
    );
}

#[test]
fn test_tuple_struct() {
    #[derive(Serialize, PartialEq, Debug)]
    struct MyStruct(u8, u16);

    assert_eq!(to_bytes(&MyStruct(0x00, 0x1111)).unwrap(), b"\x00\x11\x11");
}

#[test]
#[should_panic]
fn test_map() {
    // unimplemented

    use std::collections::BTreeMap;

    // maps would have to rely on deserializing a u8/u16/u32 as len before
    assert_eq!(
        to_bytes::<(u8, BTreeMap::<u8, u16>)>(&(
            0x02,
            [(1, 0x1111), (2, 0x2222)]
                .iter()
                .cloned()
                .collect::<BTreeMap<_, _>>()
        ))
        .unwrap(),
        b"\x02\x01\x11\x11\x02\x22\x22"
    );
}

#[test]
fn test_struct() {
    #[derive(Serialize, PartialEq, Debug)]
    struct MyStruct {
        a: u16,
        len: u8,
        vec: Vec<u16>,
        b: u16,
    }

    assert_eq!(
        to_bytes(&MyStruct {
            a: 0xffff,
            len: 2,
            vec: vec![0x1111, 0x2222],
            b: 0xffff,
        })
        .unwrap(),
        b"\xff\xff\x02\x11\x11\x22\x22\xff\xff"
    );
}

#[test]
fn test_enum() {
    #[derive(Serialize, PartialEq, Debug)]
    #[repr(u16)]
    enum MyEnum {
        Unit = 0x1111,
        Newtype(u16) = 0x2222,
        Tuple(u8, u16) = 0x3333,
        Struct { field: u16 } = 0x4444,
    }

    // MyEnum::Unit
    assert_eq!(to_bytes(&MyEnum::Unit).unwrap(), b"\x11\x11");

    // MyEnum::Newtype
    assert_eq!(
        to_bytes(&MyEnum::Newtype(0x1111)).unwrap(),
        b"\x22\x22\x11\x11"
    );

    // MyEnum::Tuple
    assert_eq!(
        to_bytes(&MyEnum::Tuple(0x11, 0x2222)).unwrap(),
        b"\x33\x33\x11\x22\x22"
    );

    // MyEnum::Struct
    assert_eq!(
        to_bytes(&MyEnum::Struct { field: 0x1111 }).unwrap(),
        b"\x44\x44\x11\x11"
    );
}
