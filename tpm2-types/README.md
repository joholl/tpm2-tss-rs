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


# Byte Arrays

In the spec, byte arrays (most of the `TPM2B_` structs) always consist of a size
(`u16`) and a buffer. In Rust, they are mapped to `Vec<u8>`.


# Size fields

Aside from byte arrays, there a sized structs (_outer_):

* `TPM2B_SENSITIVE_CREATE`
* `TPM2B_ECC_POINT`
* `TPM2B_CREATION_DATA`
* `TPM2B_PUBLIC`
* `TPM2B_SENSITIVE`
* `TPM2B_NV_PUBLIC`

The members of _outer_ are a size (`u16`) and a struct (_inner_). Since in Rust,
the `.size()`of _outer_ and _inner_ are different, they are two distinct types.

* in C: `TPM2B_PUBLIC { size: u16, sensitive: TPMT_PUBLIC}`
* in Rust: `StructWithSize<Public>` (where `Public` is equivalent to `TPMT_PUBLIC`)

# Non-byte Arrays

TODO

Count:

Needed as size for dynamically-sized arrays. The size is always before the
array. Size is always u8, u16 or u32. Note: there are statically-sized
arrays which are enum variants.



Struct size fields

Struct size fields means fields that quantify the size of the next field
("inner") in the struct ("outer") in bytes. In these structs, there is
always one u16 size field and one struct field. Note: there are other
count-type fields which are distinct from size fields.

In this type system, outer and inner are two distinct types to enable
inner.size()/outer.size()