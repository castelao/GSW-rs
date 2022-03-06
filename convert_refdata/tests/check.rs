use heapless::{FnvIndexMap, String, Vec};
use postcard::from_bytes;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::Read;

// Structure of the validation dataset
#[derive(Serialize, Deserialize, Debug)]
struct DataRef {
    version: String<8>,
    src: String<32>,
    src_md5: String<32>,
    // scalar: FnvIndexMap<String<48>, f64, 400>,
    //data_x: FnvIndexMap<String<48>, Vec<f64, 3>, 11>,
    //data2d: FnvIndexMap<String<48>, Vec<Vec<f64, 45>, 3>, 304>,
}

#[test]
#[cfg(not(windows))]
fn check_version() {
    let mut input = File::open("data/gsw_validation.bin").expect("Unable to open file");
    let mut contents = vec![];
    input
        .read_to_end(&mut contents)
        .expect("Failed to read content");
    let out: DataRef = from_bytes(&contents).unwrap();
    assert_eq!(out.version, "3.06.12");
    assert_eq!(out.src, "gsw_data_v3_0.mat");
}
