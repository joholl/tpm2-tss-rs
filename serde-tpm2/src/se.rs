use serde::{ser, Serialize};
use std::any;

use crate::{
    error::{Error, Result},
    log::Logger,
};
use log;

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

// By convention, the public API of a Serde serializer is one or more `to_abc`
// functions such as `to_string`, `to_bytes`, or `to_writer` depending on what
// Rust types the serializer is able to produce as output.
//
// This basic serializer supports only `to_bytes`.
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
    // The output type produced by this `Serializer` during successful
    // serialization. Most serializers that produce text or binary output should
    // set `Ok = ()` and serialize into an `io::Write` or buffer contained
    // within the `Serializer` instance, as happens here. Serializers that build
    // in-memory data structures may be simplified by using `Ok` to propagate
    // the data structure around.
    type Ok = ();

    // The error type when some error occurs during serialization.
    type Error = Error;

    // Associated types for keeping track of additional state while serializing
    // compound data structures like sequences and maps. In this case no
    // additional state is required beyond what is already stored in the
    // Serializer struct.
    type SerializeSeq = Self;
    type SerializeTuple = Self;
    type SerializeTupleStruct = Self;
    type SerializeTupleVariant = Self;
    type SerializeMap = Self;
    type SerializeStruct = Self;
    type SerializeStructVariant = Self;

    // Here we go with the simple methods. The following 12 methods receive one
    // of the primitive types of the data model and map it to JSON by appending
    // into the output string.
    fn serialize_bool(self, v: bool) -> Result<()> {
        let v = v as u8;
        self.logger.log_primitive(v);
        self.serialize_u8(v)
    }

    // JSON does not distinguish between different sizes of integers, so all
    // signed integers will be serialized the same and all unsigned integers
    // will be serialized the same. Other formats, especially compact binary
    // formats, may need independent logic for the different sizes.
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

    fn serialize_f32(self, _v: f32) -> Result<()> {
        unimplemented!()
    }

    fn serialize_f64(self, _v: f64) -> Result<()> {
        unimplemented!()
    }

    // Serialize a char as a single-character string. Other formats may
    // represent this differently.
    fn serialize_char(self, v: char) -> Result<()> {
        let v = v as u8;
        self.serialize_u8(v)
    }

    // This only works for strings that don't require escape sequences but you
    // get the idea. For example it would emit invalid JSON if the input string
    // contains a '"' character.
    fn serialize_str(self, _v: &str) -> Result<()> {
        unimplemented!()
    }

    // Serialize a byte array as an array of bytes. Could also use a base64
    // string here. Binary formats will typically represent byte arrays more
    // compactly.
    fn serialize_bytes(self, _v: &[u8]) -> Result<()> {
        unimplemented!()
        // use serde::ser::SerializeSeq;
        // self.logger.log(format_args!("&[u8; {}]", v.len()));
        // let mut seq = self.serialize_seq(Some(v.len()))?;
        // for byte in v {
        //     log::info!("    {:02x}", byte);
        //     seq.serialize_element(byte)?;
        // }
        // seq.end()
    }

    // An absent optional is represented as the JSON `null`.
    fn serialize_none(self) -> Result<()> {
        log::info!("serializing {:i$}None", "", i = self.indent());
        Ok(())
    }

    // A present optional is represented as just the contained value. Note that
    // this is a lossy representation. For example the values `Some(())` and
    // `None` both serialize as just `null`. Unfortunately this is typically
    // what people expect when working with JSON. Other formats are encouraged
    // to behave more intelligently if possible.
    fn serialize_some<T>(self, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        // TODO is this needed?
        log::info!("serializing {:i$}Some(...)", "", i = self.indent());
        //self.level_push();
        let result = value.serialize(self);
        //self.level_pop();
        result
    }

    // In Serde, unit means an anonymous value containing no data. Map this to
    // JSON as `null`.
    fn serialize_unit(self) -> Result<()> {
        log::info!("serializing {:i$}unit", "", i = self.indent());
        unimplemented!()
    }

    // Unit struct means a named value containing no data. Again, since there is
    // no data, map this to JSON as `null`. There is no need to serialize the
    // name in most formats.
    fn serialize_unit_struct(self, name: &'static str) -> Result<()> {
        log::info!(
            "serializing {:i$}unit struct {}",
            "",
            name,
            i = self.indent()
        );
        unimplemented!()
    }

    // As is done here, serializers are encouraged to treat newtype structs as
    // insignificant wrappers around the data they contain.
    fn serialize_newtype_struct<T>(self, name: &'static str, _value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        log::info!(
            "serializing {:i$}new type struct {}",
            "",
            name,
            i = self.indent()
        );
        unimplemented!()
    }

    // Now we get to the serialization of compound types.
    //
    // The start of the sequence, each value, and the end are three separate
    // method calls. This one is responsible only for serializing the start,
    // which in JSON is `[`.
    //
    // The length of the sequence may or may not be known ahead of time. This
    // doesn't make a difference in JSON because the length is not represented
    // explicitly in the serialized form. Some serializers may only be able to
    // support sequences for which the length is known up front.
    fn serialize_seq(self, _len: Option<usize>) -> Result<Self::SerializeSeq> {
        self.logger.level_push();
        Ok(self)
    }

    // Tuples look just like sequences in JSON. Some formats may be able to
    // represent tuples more efficiently by omitting the length, since tuple
    // means that the corresponding `Deserialize implementation will know the
    // length without needing to look at the serialized data.
    fn serialize_tuple(self, len: usize) -> Result<Self::SerializeTuple> {
        self.logger.log(format_args!("tuple[{}]", len));
        Ok(self)
    }

    // Tuple structs look just like sequences in JSON.
    fn serialize_tuple_struct(
        self,
        name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleStruct> {
        self.logger
            .log(format_args!("enum variant: tuple_struct {}{}", name, len));
        unimplemented!()
    }

    // Unit types are serialized to nothing
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

    // Note that newtype variant (and all of the other variant serialization
    // methods) refer exclusively to the "externally tagged" enum
    // representation.
    //
    // Serialize this to JSON in externally tagged form as `{ NAME: VALUE }`.
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

    // Tuple variants are represented in JSON as `{ NAME: [DATA...] }`. Again
    // this method is only responsible for the externally tagged representation.
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
    // Struct variants are represented in JSON as `{ NAME: { K: V, ... } }`.
    // This is the externally tagged representation.
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

    // Maps are represented in JSON as `{ K: V, K: V, ... }`.
    fn serialize_map(self, len: Option<usize>) -> Result<Self::SerializeMap> {
        log::info!("serializing {:i$}map[{:?}]", "", len, i = self.indent());
        Ok(self)
    }

    // Structs look just like maps in JSON. In particular, JSON requires that we
    // serialize the field names of the struct. Other formats may be able to
    // omit the field names when serializing structs because the corresponding
    // Deserialize implementation is required to know what the keys are without
    // looking at the serialized data.
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

impl<'a> ser::SerializeMap for &'a mut Serializer {
    type Ok = ();
    type Error = Error;

    fn serialize_key<T>(&mut self, _key: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        log::info!("serializing {:i$}map.key", "", i = self.indent());
        unimplemented!()
    }

    fn serialize_value<T>(&mut self, _value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        log::info!("serializing {:i$}map.value", "", i = self.indent());
        unimplemented!()
    }

    fn end(self) -> Result<()> {
        log::info!("serializing {:i$}map.end", "", i = self.indent());
        unimplemented!()
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
fn test_struct() {
    #[derive(Serialize, PartialEq, Debug)]
    struct MyStruct {
        int: i16,
        buffer: Vec<u8>,
    }

    let my_struct = MyStruct {
        int: i16::from_be_bytes(*b"\xff\xee"),
        buffer: b"\xaa\xbb\xcc\xdd".to_vec(),
    };
    let serialized = to_bytes(&my_struct).unwrap();
    assert_eq!(serialized, b"\xff\xee\x00\x04\xaa\xbb\xcc\xdd");
}

#[test]
fn test_enum() {
    #[derive(Serialize, PartialEq, Debug)]
    #[repr(u16)]
    enum MyEnum {
        Unit = 0x1122,
        Newtype(u32) = 0x3344,
        Tuple(u32, u32) = 0x5566,
        Struct { field: u32 } = 0x7788,
    }

    // MyEnum::Unit
    let my_enum = MyEnum::Unit;
    let serialized = to_bytes(&my_enum).unwrap();
    let bytes = b"\x11\x22";
    assert_eq!(serialized, bytes);

    // MyEnum::Newtype
    let my_enum = MyEnum::Newtype(u32::from_be_bytes(*b"\xaa\xbb\xcc\xdd"));
    let serialized = to_bytes(&my_enum).unwrap();
    let bytes = b"\x33\x44\xaa\xbb\xcc\xdd";
    assert_eq!(serialized, bytes);

    // MyEnum::Tuple
    let my_enum = MyEnum::Tuple(
        u32::from_be_bytes(*b"\xaa\xbb\xcc\xdd"),
        u32::from_be_bytes(*b"\xcc\xdd\xee\xff"),
    );
    let serialized = to_bytes(&my_enum).unwrap();
    let bytes = b"\x55\x66\xaa\xbb\xcc\xdd\xcc\xdd\xee\xff";
    assert_eq!(serialized, bytes);

    // MyEnum::Struct
    let my_enum = MyEnum::Struct {
        field: u32::from_be_bytes(*b"\xaa\xbb\xcc\xdd"),
    };
    let serialized = to_bytes(&my_enum).unwrap();
    let bytes = b"\x77\x88\xaa\xbb\xcc\xdd";
    assert_eq!(serialized, bytes);
}
