use std::{fs, path::Path};

fn main() {
    println!("cargo:rerun-if-changed=src/api.rs");

    cxx_build::bridge("src/api.rs")
        .flag_if_supported("-std=c++23")
        .compile("audio");

    let cxx_path = Path::new("target/cxxbridge/rust/cxx.h");
    let api_h_path = Path::new("target/cxxbridge/improved_rust_audio/src/api.rs.h");
    let api_cc_path = Path::new("target/cxxbridge/improved_rust_audio/src/api.rs.cc");

    fs::create_dir_all("include").unwrap();
    fs::copy(cxx_path, "include/rust/cxx.h").expect("Failed to copy cxx.h");
    fs::copy(api_h_path, "include/rust/audio.h").expect("Failed to copy api.h");
    fs::copy(api_cc_path, "include/rust/audio.cc").expect("Failed to copy api.cc");
}