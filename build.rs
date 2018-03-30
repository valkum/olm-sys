extern crate bindgen;

use std::{env,path::PathBuf};

fn main() {
    // Link to olm shared library
    println!("cargo:rustc-link-lib=olm");

    let bindings = bindgen::Builder::default()
        .header("wrapper.h")
        .generate()
        .expect("Unable to generate bindings for libolm");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());

    bindings.write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings for libolm to output directory");
}
