extern crate bindgen;

use std::env;
use std::path::PathBuf;

fn main() {
    let project_dir = env::current_dir().expect("Failed to retrieve current directory");

    let bindings = bindgen::Builder::default()
        .header(
            //.join("../../../include/tss2/tss2_tcti.h")
            "../../tpm2-tss/include/tss2/tss2_tcti.h",
        )
        .header("../../tpm2-tss/include/tss2/tss2_common.h")
        .header("../../tpm2-tss/include/tss2/tss2_tctildr.h")
        .clang_args([
            &format!(
                "-I{}",
                project_dir
                    .join("../../tpm2-tss/src/tss2-tcti/.libs")
                    .to_str()
                    .unwrap()
            ),
            // format!(
            //     "-L{}",
            //     project_dir
            //         .join("/home/johannes/persistent/dev-projects/tpm/tpm2-tss/src/tss2-tcti/.libs")
            //         .to_str()
            //         .unwrap()
            // ),
            // "-ltss2-tctildr".to_string(), // TODO not built? see https://github.com/parallaxsecond/rust-tss-esapi/blob/29b72787f8caa848727b05a9df3161ef2bf5918f/tss-esapi-sys/build.rs#L4
        ])
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .generate()
        .expect("Unable to generate rust bindings for TCTI C code");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");

    // TODO fix paths
    //println!("cargo:rustc-link-search=../../src/tss2-tcti/.libs");
    println!(
        "cargo:rustc-link-search={}",
        project_dir
            .join("../../tpm2-tss/src/tss2-tcti/.libs")
            .to_str()
            .unwrap()
    );
    println!("cargo:rustc-link-lib=tss2-tctildr");
}
