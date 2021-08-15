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
    data_x: FnvIndexMap<String<16>, Vec<f64, 3>, 4>,
    data2d: FnvIndexMap<String<16>, Vec<Vec<f64, 45>, 3>, 16>,
}

#[test]
fn specvol() {
    let mut input = File::open("tests/data/gsw_validation.bin").expect("Unable to open file");
    let mut contents = vec![];
    input
        .read_to_end(&mut contents)
        .expect("Failed to read content");

    let out: DataRef = from_bytes(&contents).unwrap();

    let p = out.data2d.get(&String::from("p_chck_cast")).unwrap();
    let sa = out.data2d.get(&String::from("SA_chck_cast")).unwrap();
    let ct = out.data2d.get(&String::from("CT_chck_cast")).unwrap();
    let specvol = out.data2d.get(&String::from("specvol")).unwrap();
    for i in 0..3 {
        for j in 0..45 {
            if !specvol[i][j].is_nan() {
                assert!(
                    (gsw::volume::specvol(sa[i][j], ct[i][j], p[i][j]).unwrap() - specvol[i][j])
                        .abs()
                        <= f64::EPSILON
                );
            }
        }
    }
}
