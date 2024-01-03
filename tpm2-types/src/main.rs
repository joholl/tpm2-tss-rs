pub mod alg;
pub mod util;

use crate::util::{from_hex, to_hex};
use core::panic;
use serde_tpm2::{de, se};
use std::{any::type_name, io::Write};
use tpm2_types::alg::{
    AlgAsym, AlgAsymScheme, AlgCipherMode, AlgEccKeyEchange, AlgHash, AlgKdf, AlgMacScheme,
    AlgPublic, AlgSigScheme, AlgSym, AlgSymMode, AlgSymObj, EccCurve,
};
use tpm2_types::{de::StructWithSize, types::*};

// TPM2B_PUBLIC                              .
// UINT16                                    |   .size                                      001e                 30
// TPMT_PUBLIC                               |   .publicArea
// TPMI_ALG_PUBLIC                           |   |   .type                                  0023                 TPMI_ALG_PUBLIC.ECC
// TPMI_ALG_HASH                             |   |   .nameAlg                               000b                 TPMI_ALG_HASH.SHA256
// TPMA_OBJECT                               |   |   .objectAttributes                      00020072             TPMA_OBJECT.fixedTPM | TPMA_OBJECT.fixedParent | TPMA_OBJECT.sensitiveDataOrigin | TPMA_OBJECT.userWithAuth | TPMA_OBJECT.decrypt
//                                           |   |   |   .reserved                                               ...............................0
//                                           |   |   |   .fixedTPM                                               ..............................1.
//                                           |   |   |   .stClear                                                .............................0..
//                                           |   |   |   .reserved0                                              ............................0...
//                                           |   |   |   .fixedParent                                            ...........................1....
//                                           |   |   |   .sensitiveDataOrigin                                    ..........................1.....
//                                           |   |   |   .userWithAuth                                           .........................1......
//                                           |   |   |   .adminWithPolicy                                        ........................0.......
//                                           |   |   |   .reserved1                                              ......................00........
//                                           |   |   |   .noDA                                                   .....................0..........
//                                           |   |   |   .encryptedDuplication                                   ....................0...........
//                                           |   |   |   .reserved2                                              ................0000............
//                                           |   |   |   .restricted                                             ...............0................
//                                           |   |   |   .decrypt                                                ..............1.................
//                                           |   |   |   .sign_decrypt                                           .............0..................
//                                           |   |   |   .sign                                                   ............0...................
//                                           |   |   |   .reserved3                                              000000000000....................
// TPM2B_DIGEST                              |   |   .authPolicy
// UINT16                                    |   |   |   .size                              0003                 3
// list[BYTE]                                |   |   |   .buffer                            aabbcc               ...
// TPMU_PUBLIC_PARMS                         |   |   .parameters
// TPMS_ECC_PARMS                            |   |   |   .eccDetail
// TPMT_SYM_DEF_OBJECT                       |   |   |   |   .symmetric
// TPMI_ALG_SYM_OBJECT                       |   |   |   |   |   .algorithm                 0010                 TPMI_ALG_SYM_OBJECT.NULL
// TPMU_SYM_KEY_BITS                         |   |   |   |   |   .keyBits
// TPMU_SYM_MODE                             |   |   |   |   |   .mode
// TPMU_SYM_DETAILS                          |   |   |   |   |   .details
// TPMT_ECC_SCHEME                           |   |   |   |   .scheme
// TPMI_ALG_ECC_SCHEME                       |   |   |   |   |   .scheme                    0019                 TPMI_ALG_ECC_SCHEME.ECDH
// TPMU_ASYM_SCHEME                          |   |   |   |   |   .details
// TPMS_KEY_SCHEME_ECDH                      |   |   |   |   |   |   .ecdh
// TPMI_ALG_HASH                             |   |   |   |   |   |   |   .hashAlg           000b                 TPMI_ALG_HASH.SHA256
// TPMI_ECC_CURVE                            |   |   |   |   .curveID                       0003                 TPMI_ECC_CURVE.NIST_P256
// TPMT_KDF_SCHEME                           |   |   |   |   .kdf
// TPMI_ALG_KDF                              |   |   |   |   |   .scheme                    0010                 TPMI_ALG_KDF.NULL
// TPMU_KDF_SCHEME                           |   |   |   |   |   .details
// TPMU_PUBLIC_ID                            |   |   .unique
// TPMS_ECC_POINT                            |   |   |   .ecc
// TPM2B_ECC_PARAMETER                       |   |   |   |   .x
// UINT16                                    |   |   |   |   |   .size                      0000                 0
// list[BYTE]                                |   |   |   |   |   .buffer
// TPM2B_ECC_PARAMETER                       |   |   |   |   .y
// UINT16                                    |   |   |   |   |   .size                      0003                 3
// list[BYTE]                                |   |   |   |   |   .buffer                    aabbcc               ...

// 00180023000b00020072000000100019000b0003001000000000
fn main() {
    env_logger::init();

    let in_public = StructWithSize(Public::ECC {
        name_alg: AlgHash::SHA256,
        object_attributes: 0x00020072,
        auth_policy: [0xaa, 0xbb, 0xcc].to_vec(),
        parameters: ECCParams {
            symmetric: SymDefObject::Null,
            scheme: AsymScheme::ECDH(AlgHash::SHA256),
            curve_id: EccCurve::NistP256,
            kdf: KDFScheme::Null,
        },
        unique: TPMS_ECC_POINT {
            x: [].to_vec(),
            y: [0xaa, 0xbb, 0xcc].to_vec(),
        },
    });

    println!("{:#?}", in_public);

    let serialized = se::to_bytes(&in_public).unwrap();
    let serialized_hex = to_hex(&serialized);
    println!("serialized_hex = {:?}", serialized_hex);

    let in_public_hex = "001e0023000b000200720003aabbcc00100019000b0003001000000003aabbcc";
    println!("expected = {:?}", in_public_hex);
    assert_eq!(serialized_hex, in_public_hex);

    let serialized: Vec<u8> = from_hex(in_public_hex).unwrap();
    let in_public_de: PublicSized = de::from_bytes(&serialized).unwrap();
    println!("deserialized_hex = {:#?}", in_public_de);
    assert_eq!(in_public_de, in_public);
}
