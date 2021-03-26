use std::env;

fn main() {
    let crate_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    generate_c_bindings(&crate_dir);
}

#[cfg(feature = "cbindgen")]
fn generate_c_bindings(crate_dir: &str) {
    cbindgen::generate(crate_dir)
        .expect("Unable to generate bindings")
        .write_to_file("assets/gswteos-10.h");
}

#[cfg(not(feature = "cbindgen"))]
fn generate_c_bindings(_crate_dir: &str) {}
