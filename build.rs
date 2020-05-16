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

    cc::Build::new()
        .cpp(true)
        .include(src.join("include"))
        .include(src.join("lib"))
        // Todo get version from src/common.mk
        .define("OLMLIB_VERSION_MAJOR", "3")
        .define("OLMLIB_VERSION_MINOR", "1")
        .define("OLMLIB_VERSION_PATCH", "2")
        .warnings(true)
        .file(src.join("src/account.cpp"))
        .file(src.join("src/base64.cpp"))
        .file(src.join("src/cipher.cpp"))
        .file(src.join("src/crypto.cpp"))
        .file(src.join("src/memory.cpp"))
        .file(src.join("src/message.cpp"))
        .file(src.join("src/olm.cpp"))
        .file(src.join("src/pickle.cpp"))
        .file(src.join("src/pk.cpp"))
        .file(src.join("src/ratchet.cpp"))
        .file(src.join("src/session.cpp"))
        .file(src.join("src/utility.cpp"))
        .file(src.join("src/ed25519.c"))
        .file(src.join("src/error.c"))
        .file(src.join("src/inbound_group_session.c"))
        .file(src.join("src/megolm.c"))
        .file(src.join("src/outbound_group_session.c"))
        .file(src.join("lib/crypto-algorithms/sha256.c"))
        .file(src.join("lib/crypto-algorithms/aes.c"))
        .file(src.join("lib/curve25519-donna/curve25519-donna.c"))
        .out_dir(&dst)
        .try_compile("libolm.a");


    // Link to olm static library
    println!("cargo:rustc-link-lib=static=olm");
    println!("cargo:rustc-link-search={}", dst.display());
    // Olm still needs libstdc++
    println!("cargo:rustc-link-lib=stdc++");
}