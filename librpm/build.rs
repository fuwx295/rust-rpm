
use bindgen::Builder;
use std::{env, path::PathBuf};

fn main() {
    println!("cargo:rustc-link-lib=rpm");
    println!("cargo:rustc-link-lib=rpmio");
    let builder = Builder::default().header("include/librpm.hpp");

    let output_path = PathBuf::from(env::var("OUT_DIR").unwrap()).join("binding.rs");

    builder.generate()
        .unwrap()
        .write_to_file(output_path)
        .unwrap();
}