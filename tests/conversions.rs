#![cfg(not(feature = "capi"))]

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
    scalar: FnvIndexMap<String<64>, f64, 8>,
    data_x: FnvIndexMap<String<64>, Vec<f64, 3>, 2>,
    data2d: FnvIndexMap<String<64>, Vec<Vec<f64, 45>, 3>, 32>,
}

#[test]
#[cfg(not(windows))]
fn sr_from_sp() {
    let mut input =
        File::open("tests/data/gsw_conversions_validation.bin").expect("Unable to open file");
    let mut contents = vec![];
    input
        .read_to_end(&mut contents)
        .expect("Failed to read content");

    let out: DataRef = from_bytes(&contents).unwrap();

    let sp = out.data2d.get(&String::from("SP_chck_cast")).unwrap();
    let sr_from_sp = out.data2d.get(&String::from("SR_from_SP")).unwrap();
    let tol = if cfg!(feature = "compat") || (f64::EPSILON > 1e-12) {
        f64::EPSILON
    } else {
        1e-12
    };
    for i in 0..3 {
        for j in 0..45 {
            if !sr_from_sp[i][j].is_nan() {
                assert!((gsw::conversions::sr_from_sp(sp[i][j]) - sr_from_sp[i][j]).abs() <= tol);
            }
        }
    }
}
