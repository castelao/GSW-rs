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
    data_x: FnvIndexMap<String<24>, Vec<f64, 3>, 4>,
    data2d: FnvIndexMap<String<24>, Vec<Vec<f64, 45>, 3>, 64>,
}

#[test]
#[cfg(not(windows))]
fn alpha() {
    let mut input = File::open("tests/data/gsw_validation.bin").expect("Unable to open file");
    let mut contents = vec![];
    input
        .read_to_end(&mut contents)
        .expect("Failed to read content");

    let out: DataRef = from_bytes(&contents).unwrap();

    let p = out.data2d.get(&String::from("p_chck_cast")).unwrap();
    let sa = out.data2d.get(&String::from("SA_chck_cast")).unwrap();
    let ct = out.data2d.get(&String::from("CT_chck_cast")).unwrap();
    let alpha = out.data2d.get(&String::from("alpha")).unwrap();
    for i in 0..3 {
        for j in 0..45 {
            if !alpha[i][j].is_nan() {
                assert!(
                    (gsw::volume::alpha(sa[i][j], ct[i][j], p[i][j]).unwrap() - alpha[i][j]).abs()
                        <= f64::EPSILON
                );
            }
        }
    }
}

#[test]
#[cfg(not(windows))]
fn beta() {
    let mut input = File::open("tests/data/gsw_validation.bin").expect("Unable to open file");
    let mut contents = vec![];
    input
        .read_to_end(&mut contents)
        .expect("Failed to read content");

    let out: DataRef = from_bytes(&contents).unwrap();

    let p = out.data2d.get(&String::from("p_chck_cast")).unwrap();
    let sa = out.data2d.get(&String::from("SA_chck_cast")).unwrap();
    let ct = out.data2d.get(&String::from("CT_chck_cast")).unwrap();
    let beta = out.data2d.get(&String::from("beta")).unwrap();
    for i in 0..3 {
        for j in 0..45 {
            if !beta[i][j].is_nan() {
                assert!(
                    (gsw::volume::beta(sa[i][j], ct[i][j], p[i][j]).unwrap() - beta[i][j]).abs()
                        <= f64::EPSILON
                );
            }
        }
    }
}

#[test]
#[cfg(not(windows))]
fn rho() {
    let mut input = File::open("tests/data/gsw_validation.bin").expect("Unable to open file");
    let mut contents = vec![];
    input
        .read_to_end(&mut contents)
        .expect("Failed to read content");

    let out: DataRef = from_bytes(&contents).unwrap();

    let p = out.data2d.get(&String::from("p_chck_cast")).unwrap();
    let sa = out.data2d.get(&String::from("SA_chck_cast")).unwrap();
    let ct = out.data2d.get(&String::from("CT_chck_cast")).unwrap();
    let rho = out.data2d.get(&String::from("rho")).unwrap();
    let tol = if cfg!(feature = "compat") || (f64::EPSILON > 1e-12) {
        f64::EPSILON
    } else {
        1e-12
    };
    for i in 0..3 {
        for j in 0..45 {
            if !rho[i][j].is_nan() {
                assert!(
                    (gsw::volume::rho(sa[i][j], ct[i][j], p[i][j]).unwrap() - rho[i][j]).abs()
                        <= tol
                );
            }
        }
    }
}

#[test]
#[cfg(not(windows))]
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

#[test]
#[cfg(not(windows))]
fn sound_speed() {
    let mut input = File::open("tests/data/gsw_validation.bin").expect("Unable to open file");
    let mut contents = vec![];
    input
        .read_to_end(&mut contents)
        .expect("Failed to read content");

    let out: DataRef = from_bytes(&contents).unwrap();

    let p = out.data2d.get(&String::from("p_chck_cast")).unwrap();
    let sa = out.data2d.get(&String::from("SA_chck_cast")).unwrap();
    let ct = out.data2d.get(&String::from("CT_chck_cast")).unwrap();
    let sound_speed = out.data2d.get(&String::from("sound_speed")).unwrap();
    let tol = if cfg!(feature = "compat") || (f64::EPSILON > 1e-12) {
        f64::EPSILON
    } else {
        1e-12
    };
    for i in 0..3 {
        for j in 0..45 {
            if !sound_speed[i][j].is_nan() {
                assert!(
                    (gsw::volume::sound_speed(sa[i][j], ct[i][j], p[i][j]).unwrap()
                        - sound_speed[i][j])
                        .abs()
                        <= tol
                );
            }
        }
    }
}
