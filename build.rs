extern crate bindgen;

use std::env;
use std::path::PathBuf;

fn main() {
    // Make the prebuilt static library discoverable by rustc.
    println!("cargo:rustc-link-search=native=./native");

    // Link against system libudev.
    println!("cargo:rustc-link-lib=udev");
    println!("cargo:rustc-link-search=/usr/lib");

    let bindings = bindgen::Builder::default()
        .header("/usr/include/libudev.h")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
