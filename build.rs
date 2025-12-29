extern crate bindgen;

use std::env;
use std::path::PathBuf;

fn main() {
    // Build and link the Qt C++ shim.
    let qt = pkg_config::Config::new()
        .atleast_version("6.0")
        .probe("Qt6Widgets")
        .expect("Qt6Widgets not found via pkg-config");

    cc::Build::new()
        .cpp(true)
        .files(["native/qt_bridge.cpp"])
        .includes(&qt.include_paths)
        .flag_if_supported("-std=c++17")
        .compile("qt_bridge");

    for lib in qt.libs {
        println!("cargo:rustc-link-lib={lib}");
    }
    for path in qt.link_paths {
        println!("cargo:rustc-link-search=native={}", path.display());
    }

    // Make the prebuilt static library discoverable by rustc.
    println!("cargo:rustc-link-lib=my_func");
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
