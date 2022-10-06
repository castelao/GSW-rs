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
fn coriolis_parameter() {
    let mut input = File::open("tests/data/gsw_earth_validation.bin").expect("Unable to open file");
    let mut contents = vec![];
    input
        .read_to_end(&mut contents)
        .expect("Failed to read content");

    let out: DataRef = from_bytes(&contents).unwrap();

    let f = out.data_x.get(&String::from("f")).unwrap();
    let lat = out.data_x.get(&String::from("lat_chck_cast")).unwrap();
    for i in 0..3 {
        if !f[i].is_nan() {
            assert!(
                (gsw::earth::coriollis_parameter(lat[i]).unwrap() - f[i]).abs() <= f64::EPSILON
            );
        }
    }
}

#[test]
#[cfg(not(windows))]
fn gravity() {
    let mut input = File::open("tests/data/gsw_earth_validation.bin").expect("Unable to open file");
    let mut contents = vec![];
    input
        .read_to_end(&mut contents)
        .expect("Failed to read content");

    let out: DataRef = from_bytes(&contents).unwrap();

    let lat = out.data_x.get(&String::from("lat_chck_cast")).unwrap();
    let p = out.data2d.get(&String::from("p_chck_cast")).unwrap();
    let g = out.data2d.get(&String::from("grav")).unwrap();
    for i in 0..3 {
        for j in 0..45 {
            if !g[i][j].is_nan() {
                assert!(
                    (gsw::earth::gravity(lat[i], p[i][j]).unwrap() - g[i][j]).abs() <= f64::EPSILON
                );
            }
        }
    }
}
