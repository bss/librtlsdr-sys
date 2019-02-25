use std::env;
use std::path::PathBuf;
use bindgen;
use pkg_config;

fn main() {
    let librtlsdr = pkg_config::Config::new().atleast_version("0.6.0").probe("librtlsdr").unwrap();
    let librtlsdr_include_paths = std::env::join_paths(librtlsdr.include_paths).unwrap();
    println!("cargo:include={}", librtlsdr_include_paths.to_string_lossy());

    let bindings = bindgen::Builder::default()
        .header("wrapper.h")
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}