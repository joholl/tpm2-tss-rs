extern crate bindgen;
extern crate pkg_config;

use std::env;
use std::path::PathBuf;

fn main() {
    let tctildr_pkg = match pkg_config::Config::new().probe("tss2-tctildr") {
        Ok(pkg) => pkg,
        _ => panic!("pkg-config could not find tss2-tctildr"),
    };

    for path in tctildr_pkg.link_paths {
        println!("cargo:rustc-link-search={}", path.display());
    }
    for lib in &tctildr_pkg.libs {
        println!("cargo:rustc-link-lib={}", lib);
    }

    println!("cargo:rerun-if-pkgconfig-changed=tss2-tctildr");

    let include_path = tctildr_pkg
        .include_paths
        .iter()
        .filter(|path| path.ends_with("include"))
        .next()
        .expect(&format!(
            "No include path ends in 'include': {:#?}",
            tctildr_pkg.include_paths,
        ));

    let bindings = bindgen::builder()
        .header("tss2/tss2_tcti.h")
        .header("tss2/tss2_common.h")
        // TODO clang_args does not work unless the last header path is absolute
        .header([include_path.to_str().unwrap(), "tss2/tss2_tctildr.h"].join("/"))
        .clang_args(
            tctildr_pkg
                .include_paths
                .iter()
                .map(|path| format!("-I{}", path.to_string_lossy())),
        )
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .generate()
        .expect("Unable to generate rust bindings for TCTI C code");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
