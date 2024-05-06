use std::env;
use std::path::PathBuf;

fn main() {
    cc::Build::new()
        .cpp(true)
        .file("cpp/magneto1.4.cpp")
        .file("cpp/mymathlib_matrix.cpp")
        .compile("magneto");

    let bindings = bindgen::Builder::default()
        .use_core()
        .header("cpp/bindings.hpp")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
