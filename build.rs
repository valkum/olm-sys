extern crate bindgen;

use std::process::{Command, Stdio};
use std::{env, fs, path::PathBuf};

fn main() {
    let manifest_dir = match env::var_os("CARGO_MANIFEST_DIR") {
        Some(d) => d,
        None => panic!("Unable to read manifest dir"),
    };
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());

    // path to olm source code
    let src = PathBuf::from(&manifest_dir).join("olm");
    // where we will put our built library for static linking
    let dst = PathBuf::from(&out_path).join("build");
    let _ = fs::create_dir(&dst);
    // path to our final libolm file
    let dst_file = dst.join("libolm.a");

    // building libolm as a static lib
    if !dst_file.exists() {
        run(Command::new("make").arg("static").current_dir(&src));
        let _ = fs::copy(&src.join("build/libolm.a"), &dst_file);
    }

    let bindings = bindgen::Builder::default()
        .header("wrapper.h")
        .clang_arg("-I./olm/include")
        .generate()
        .expect("Unable to generate bindings for libolm");

    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings for libolm to output directory");

    // Link to olm static library
    println!("cargo:rustc-link-lib=static=olm");
    println!("cargo:rustc-link-search={}", dst.display());
    // Olm still needs libstdc++
    println!("cargo:rustc-link-lib=stdc++");
}

fn run(cmd: &mut Command) {
    assert!(
        cmd.stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .status()
            .unwrap()
            .success()
    );
}
