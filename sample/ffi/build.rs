extern crate bindgen;
extern crate cc;

use std::env;
use std::path::PathBuf;

fn main() {
    // static library name and path
    let lib_name = "foo";
    let lib_path = "lib/libfoo.a";

    // link to static library
    println!("cargo:rustc-link-lib=static={}", lib_name);
    println!("cargo:rustc-link-search=native={}", lib_path);

    // header file ane path
    let header_name = "foo.h";
    let header_path = "include/foo.h";

    // generate rust bindings from header file
    let bindings = bindgen::Builder::default()
        .header(header_path)
        .clang_arg("-I/path/to/include")
        .generate()
        .expect("Unable to generate bindings");

    // Writes the generated bindings to a file
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
