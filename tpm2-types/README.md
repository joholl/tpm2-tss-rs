## Design

You will find that this crate deviates from the
[specification](https://trustedcomputinggroup.org/wp-content/uploads/TCG_TPM2_r1p59_Part2_Structures_pub.pdf).
The following section will explain how and why.


## Naming

Most importantly, the type names where adapted to fit the Rust naming
convention. For instance, struct fields are named in `snake_case`. Prefixes like
`TPM2B_` or `TPMS_` are omitted. This make the code more clean, readable and
idiomatic for users.

As an example, `TPMT_SYM_DEF_OBJECT` becomes `SymDefObject`.

If you cannot find your type, try to Rustify it. Alternatively you can grep the
docs for the specified name.


## Type Aliases

The spec defines many type aliases. For instance `TPMS_SIG_SCHEME_ECDSA` is a
`TPMS_SCHEME_HASH`, which is a `TPMI_ALG_HASH`, which is a `TPM_ALG_ID`, which
is a `TPM_ALG`, which is a `u16`. You get the idea.

Currently, the aliases do not add much value beyond semantic information. Since
aliases are overhead-free, they are used here in Rust, too.

For `TPMI_ALG_` types, enums are used, e.g. `TPMI_ALG_SYM_MODE` becomes
`AlgSymMode`. This enables compiler-checked strong typing.

Single-field structs are implemented as Rust type alias, too. This flattens the
data structure.


## Enums

Single-number enums like `AlgHash` are serialized with
[serde-repr](https://github.com/dtolnay/serde-repr).

For unions: in the spec, we have structs with a selector and one (or more) union(s).
That is mapped to a enum where the variant discriminants are the respective
selector value. Since the selector is now part of the enum, we can flatten
the structure,

* Spec/C: `TPMT_KDF_SCHEME kdf = {scheme: TPMI_ALG_KDF, details: TPMU_KDF_SCHEME}`
* Rust: `KDFScheme kdf`

The selector is always the first field except for in `TPMS_ATTEST`. The selector
is always a `u16` or `u32`.

If a struct with an enum member has more than two members (selector, enum), all
other members are combined into a single enum member.

If an enum variant has exactly one field, it is implemented as a tuple.
Otherwise, it is an anonymous struct variant.

## Arrays And Other Sized Things

The following is a summary of all occurances of arrays in the TPM spec.

The spec is a terribly inconsistent. All of the `TPM2B_` structs have a `size:
u16` as a first field. Most of the `TPM2B_` types have a byte buffer as a second
field. And if they don't, the second member is not an array, but a struct where
the size means its size in bytes.

There other types of arrays. Most of them are called `TPML_`. All of the
`TPML_` structs have a `count: u32` as a first field. Also, all of them
have an array as a second field.

Then, we have `TPMS_PCR_SELECT` which has a `size_of_select: u8` and a byte
buffer as first/second fields. And let's not forget about
`TPMS_PCR_SELECTION` and `TPMS_TAGGED_PCR_SELECT`, which both have
`size_of_select: u8` and a byte buffer as second/third fields, respectively.

And lastly, we have `TPMU_HA`, a union which has a statically typed byte array
without any explicit size, based on a hash algorithm selector.

Well, outside of the *Structures* spec, there is one more notable array. See, in
TPM commands, there is an `TPMS_AUTH_COMMAND` array, which is prepended by a
very special size. This special size is not the number of elements. No, it is
the size of all elements combined in bytes. An analogue occurance is found in
TPM responses.

Dear TCG, if you are reading this: For the TPM3.0 I suggest that all lists of
things are prepended by a `count: u16`. Thanks.


### Byte Arrays (TPM2B_)

Since not all arrays (and not even all byte arrays) are prepended with a `u16`
size, we have to explicitly specify which kind of size is needed.

```rust
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct OuterStruct {
    // ...
    // together, this is TPM2B_NONCE
    #[serde(deserialize_with = "deserialize_u16_sized_vec")]
    pub nonce: Vec<u8>,
    // ...
}
```


### Sized fields (TPM2B_)

All `TPM2B_` types which are not byte arrays are sized structs (_outer_). These
are:

* `TPM2B_SENSITIVE_CREATE`
* `TPM2B_ECC_POINT`
* `TPM2B_CREATION_DATA`
* `TPM2B_PUBLIC`
* `TPM2B_SENSITIVE`
* `TPM2B_NV_PUBLIC`

The fields of _outer_ are a size (`u16`) and a struct (_inner_), where the size
is the size of _inner_ in bytes. Note that _inner_ is not an array, just a
single object.

```rust
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct OuterStruct {
    // ...
    // TPM2B_SENSITIVE_CREATE
    #[serde(deserialize_with = "deserialize_u16_sized_field")]
    pub user_auth: EccPoint,  // TPMS_ECC_POINT
    // ...
}
```

Via `deserialize_with`, we get rid of one level of indirection (the `TPM2B_`
type).

Alternatively, one could try to implement a custom deserialize function for e.g.
`EccPoint` (either explicitly or via custom derive macro). The problem with this
is that you then cannot use its default deserialize function provided by the
serde #[derive(Deserialize)], anymore. I.e. you have to write it from scratch.

With the solution above, however, you just have a function which internally
delegates to the `EccPoint` default implementation.


### Non-TPM2B Arrays

All arrays with a count (`TPML_` and the PCR select types) are handled just like
byte arrays: with `deserialize_u8_sized_vec`, `deserialize_u16_sized_vec`, `deserialize_u32_sized_vec`.


### Statically-sized Arrays

TODO


### Command/Response AuthArea

TODO


