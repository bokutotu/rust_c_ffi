extern crate cc;
extern crate bindgen;

use std::env;
use std::path::PathBuf;

fn main() {
    //println!("cargo:rustc-link-lib=c++");
    println!("cargo:rerun-if-changed=src/dog.hpp");

    cc::Build::new()
        .file("C++/dog.cpp")
        //.flag("-std=c++17")
        .include("C++")
        .compile("dog");

    let bindings = bindgen::Builder::default()
        .header("C++/dog.hpp")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("dog.rs"))
        .expect("Couldn't write bindings!");
}
