use std::env;

#[cfg(feature = "cbindgen")]
fn main() {
    let crate_dir = env::var("CARGO_MANIFEST_DIR").unwrap();

    cbindgen::generate(crate_dir)
        .expect("Unable to generate bindings")
        .write_to_file("include/gswteos-10.h");
}

#[cfg(not(feature = "cbindgen"))]
fn main() {}
