use serde::{Deserialize, Deserializer};

pub fn deserialize_sized_vec<'de, D, Size, Element>(
    deserializer: D,
) -> Result<Vec<Element>, D::Error>
where
    D: Deserializer<'de>,
    Size: Deserialize<'de>,
    Element: Deserialize<'de>,
{
    #[derive(Deserialize)]
    struct SizedVec<Size, Element> {
        _size: Size,
        vec: Vec<Element>,
    }

    let sized_vec = SizedVec::<Size, Element>::deserialize(deserializer)?;
    Ok(sized_vec.vec)
}

pub fn deserialize_u8_sized_vec<'de, D, Elem>(deserializer: D) -> Result<Vec<Elem>, D::Error>
where
    D: Deserializer<'de>,
    Elem: Deserialize<'de>,
{
    deserialize_sized_vec::<D, u8, Elem>(deserializer)
}

pub fn deserialize_u16_sized_vec<'de, D, Elem>(deserializer: D) -> Result<Vec<Elem>, D::Error>
where
    D: Deserializer<'de>,
    Elem: Deserialize<'de>,
{
    deserialize_sized_vec::<D, u16, Elem>(deserializer)
}

pub fn deserialize_u32_sized_vec<'de, D, Elem>(deserializer: D) -> Result<Vec<Elem>, D::Error>
where
    D: Deserializer<'de>,
    Elem: Deserialize<'de>,
{
    deserialize_sized_vec::<D, u32, Elem>(deserializer)
}

////////////////////////////////////////////////////////////////////////////////

#[test]
fn test_deserialize_sized_u32_vec() {
    use serde_tpm2::de::from_bytes;

    #[derive(Deserialize, Debug, Clone, PartialEq)]
    pub struct Outer {
        pre: u16,
        #[serde(deserialize_with = "deserialize_u8_sized_vec")]
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

    #[derive(Deserialize, Debug, Clone, PartialEq)]
    pub struct Elem {
        a: u8,
        b: u16,
    }

    #[derive(Deserialize, Debug, Clone, PartialEq)]
    pub struct Outer {
        pre: u16,
        #[serde(deserialize_with = "deserialize_u16_sized_vec")]
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
