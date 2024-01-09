use serde::{ser, Deserialize, Deserializer, Serialize, Serializer};
use std::any::type_name;

/// internal wrapper type for Vec<Element>
#[derive(Serialize, Deserialize)]
struct WithSize<Size, Element> {
    pub _size: Size,
    pub vec: Vec<Element>,
}

impl<Size, Element> TryFrom<&Vec<Element>> for WithSize<Size, Element>
where
    Element: Clone,
    Size: TryFrom<usize> + Copy,
{
    type Error = String;

    fn try_from(vec: &Vec<Element>) -> Result<Self, Self::Error> {
        let result = Self {
            _size: vec.len().try_into().map_err(|_| {
                format!(
                    "Could not convert size to {}: {}",
                    type_name::<Size>(),
                    vec.len()
                )
            })?,
            // TODO can we avoid cloning here?
            vec: vec.clone(),
        };
        Ok(result)
    }
}

////////////////////////////

macro_rules! def_and_impl_sized_vector_trait {
    ($name:ident, $size:ty) => {
        /// trait for usage with #[serde(with="...")]
        pub trait $name<'de, Element>: Sized {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: Serializer;
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: Deserializer<'de>;
        }

        impl<'de, Element> $name<'de, Element> for Vec<Element>
        where
            Element: Clone + Serialize + Deserialize<'de>,
        {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: Serializer,
            {
                WithSize::<$size, Element>::try_from(self)
                    .map_err(|msg| ser::Error::custom(msg))?
                    .serialize(serializer)
            }

            fn deserialize<D>(deserializer: D) -> Result<Vec<Element>, D::Error>
            where
                D: Deserializer<'de>,
            {
                let sized_vec = WithSize::<$size, Element>::deserialize(deserializer)?;
                Ok(sized_vec.vec)
            }
        }
    };
}

def_and_impl_sized_vector_trait!(U8SizedVector, u8);
def_and_impl_sized_vector_trait!(U16SizedVector, u16);
def_and_impl_sized_vector_trait!(U32SizedVector, u32);

////////////////////////////

#[test]
fn test_deserialize_sized_u32_vec() {
    use serde_tpm2::de::from_bytes;

    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct Outer {
        pre: u16,
        #[serde(with = "U8SizedVector")]
        vec: Vec<u32>,
        post: u16,
    }

    let bytes = b"\xff\xff\x03\x00\x00\x00\x00\x00\x00\x00\x01\x00\x00\x00\x02\xff\xff";
    let deserialized: Outer = from_bytes(bytes).unwrap();
    assert_eq!(
        deserialized,
        Outer {
            pre: u16::from_be_bytes(*b"\xff\xff"),
            vec: vec![
                u32::from_be_bytes(*b"\x00\x00\x00\x00"),
                u32::from_be_bytes(*b"\x00\x00\x00\x01"),
                u32::from_be_bytes(*b"\x00\x00\x00\x02"),
            ],
            post: u16::from_be_bytes(*b"\xff\xff"),
        }
    );
}

#[test]
fn test_deserialize_sized_struct_array() {
    use serde_tpm2::de::from_bytes;

    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct Elem {
        a: u8,
        b: u16,
    }

    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct Outer {
        pre: u16,
        #[serde(with = "U16SizedVector")]
        vec: Vec<Elem>,
        post: u16,
    }

    let bytes = b"\x00\x00\x00\x03\x01\x02\x03\x11\x12\x13\x21\x22\x23\xff\xff";
    let deserialized: Outer = from_bytes(bytes).unwrap();
    assert_eq!(
        deserialized,
        Outer {
            pre: u16::from_be_bytes(*b"\x00\x00"),
            vec: vec![
                Elem {
                    a: 0x01,
                    b: u16::from_be_bytes(*b"\x02\x03")
                },
                Elem {
                    a: 0x11,
                    b: u16::from_be_bytes(*b"\x12\x13")
                },
                Elem {
                    a: 0x21,
                    b: u16::from_be_bytes(*b"\x22\x23")
                }
            ],
            post: u16::from_be_bytes(*b"\xff\xff"),
        }
    );
}

#[test]
fn test_serialize_sized_u32_vec() {
    use serde_tpm2::se::to_bytes;

    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct Outer {
        pre: u16,
        #[serde(with = "U8SizedVector")]
        vec: Vec<u32>,
        post: u16,
    }

    assert_eq!(
        to_bytes(&Outer {
            pre: u16::from_be_bytes(*b"\xff\xff"),
            vec: vec![
                u32::from_be_bytes(*b"\x00\x00\x00\x00"),
                u32::from_be_bytes(*b"\x00\x00\x00\x01"),
                u32::from_be_bytes(*b"\x00\x00\x00\x02"),
            ],
            post: u16::from_be_bytes(*b"\xff\xff"),
        })
        .unwrap(),
        b"\xff\xff\x03\x00\x00\x00\x00\x00\x00\x00\x01\x00\x00\x00\x02\xff\xff"
    );
}

#[test]
fn test_serialize_sized_struct_array() {
    use serde_tpm2::se::to_bytes;

    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct Elem {
        a: u8,
        b: u16,
    }

    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct Outer {
        pre: u16,
        #[serde(with = "U16SizedVector")]
        vec: Vec<Elem>,
        post: u16,
    }

    assert_eq!(
        to_bytes(&Outer {
            pre: u16::from_be_bytes(*b"\x00\x00"),
            vec: vec![
                Elem {
                    a: 0x01,
                    b: u16::from_be_bytes(*b"\x02\x03")
                },
                Elem {
                    a: 0x11,
                    b: u16::from_be_bytes(*b"\x12\x13")
                },
                Elem {
                    a: 0x21,
                    b: u16::from_be_bytes(*b"\x22\x23")
                }
            ],
            post: u16::from_be_bytes(*b"\xff\xff"),
        })
        .unwrap(),
        b"\x00\x00\x00\x03\x01\x02\x03\x11\x12\x13\x21\x22\x23\xff\xff"
    );
}
