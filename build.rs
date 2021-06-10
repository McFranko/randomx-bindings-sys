extern crate bindgen;
extern crate cmake;

use std::env;
use std::path::PathBuf;

fn main() {
    // TODO: Env var for setting `-DARCH=native`.
    let dst = cmake::Config::new("randomx")
        .build_target("randomx")
        .build();

    println!("cargo:rustc-link-search=native={}/build", dst.display());
    println!("cargo:rustc-link-lib=static=randomx");

    // TODO: Some systems may use stdc++, or something else.
    println!("cargo:rustc-link-lib=dylib=libc++");

    let bindings = bindgen::Builder::default()
        .header("randomx/src/randomx.h")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Unable to write bindings");
}
