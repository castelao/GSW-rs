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
    scalar: FnvIndexMap<String<64>, f64, 512>,
    data_x: FnvIndexMap<String<64>, Vec<f64, 3>, 16>,
    data2d: FnvIndexMap<String<64>, Vec<Vec<f64, 45>, 3>, 512>,
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

#[test]
#[cfg(not(windows))]
fn check_1d() {
    let mut input = File::open("data/gsw_validation.bin").expect("Unable to open file");
    let mut contents = vec![];
    input
        .read_to_end(&mut contents)
        .expect("Failed to read content");
    let out: DataRef = from_bytes(&contents).unwrap();
    for key in out.data_x.keys() {
        dbg!(key);
    }
    for (key, val) in out.data_x.iter() {
        dbg!("key: {} val: {}", key, val);
    }
    let neutral_density = out.data_x.get(&String::from("Neutral_Density")).unwrap();
    dbg!(neutral_density);
    let f = out.data_x.get(&heapless::String::from("f"));
    dbg!(f);
    let f = out.data_x.get(&String::from("f")).unwrap();
    dbg!(f);
}
