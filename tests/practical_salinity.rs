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
    scalar: FnvIndexMap<String<64>, f64, 2>,
    data_x: FnvIndexMap<String<64>, Vec<f64, 3>, 2>,
    data2d: FnvIndexMap<String<64>, Vec<Vec<f64, 45>, 3>, 32>,
}

#[test]
#[cfg(not(windows))]
fn sp_from_r() {
    let mut input = File::open("tests/data/gsw_practical_salinity_validation.bin")
        .expect("Unable to open file");
    let mut contents = vec![];
    input
        .read_to_end(&mut contents)
        .expect("Failed to read content");

    let out: DataRef = from_bytes(&contents).unwrap();

    let p = out.data2d.get(&String::from("p_chck_cast")).unwrap();
    let t = out.data2d.get(&String::from("t_chck_cast")).unwrap();
    let sp = out.data2d.get(&String::from("SP_chck_cast")).unwrap();
    let r = out.data2d.get(&String::from("R_from_SP")).unwrap();
    let sp_from_r = out.data2d.get(&String::from("SP_from_R")).unwrap();
    // Value obtained from sp_vs. Since there are no R_chck_cast, we do the
    // round trip conversion, which can lead to differences as large as this:
    let tol = 2.2e-14;
    for i in 0..3 {
        for j in 0..45 {
            if !sp_from_r[i][j].is_nan() {
                let new = gsw::practical_salinity::sp_from_r(r[i][j], t[i][j], p[i][j]).unwrap();
                dbg!(
                    i,
                    j,
                    sp[i][j],
                    t[i][j],
                    p[i][j],
                    r[i][j],
                    sp_from_r[i][j],
                    new,
                    sp_from_r[i][j] - new
                );
                assert!(
                    r[i][j]
                        - gsw::practical_salinity::r_from_sp(sp[i][j], t[i][j], p[i][j]).unwrap()
                        <= f64::EPSILON
                );
                assert!(
                    (gsw::practical_salinity::sp_from_r(r[i][j], t[i][j], p[i][j]).unwrap()
                        - sp_from_r[i][j])
                        .abs()
                        <= tol
                );
            }
        }
    }
}

//#[test]
#[cfg(not(windows))]
fn c_from_sp() {
    let mut input = File::open("tests/data/gsw_practical_salinity_validation.bin")
        .expect("Unable to open file");
    let mut contents = vec![];
    input
        .read_to_end(&mut contents)
        .expect("Failed to read content");

    let out: DataRef = from_bytes(&contents).unwrap();

    let p = out.data2d.get(&String::from("p_chck_cast")).unwrap();
    let sp = out.data2d.get(&String::from("SP_chck_cast")).unwrap();
    let t = out.data2d.get(&String::from("t_chck_cast")).unwrap();
    let c_from_sp = out.data2d.get(&String::from("C_from_SP")).unwrap();
    for i in 0..3 {
        for j in 0..45 {
            if !c_from_sp[i][j].is_nan() {
                let new = gsw::practical_salinity::c_from_sp(sp[i][j], t[i][j], p[i][j]).unwrap();
                dbg!(
                    i,
                    j,
                    sp[i][j],
                    t[i][j],
                    p[i][j],
                    c_from_sp[i][j],
                    new,
                    c_from_sp[i][j] - new
                );
                assert!(
                    (gsw::practical_salinity::c_from_sp(sp[i][j], t[i][j], p[i][j]).unwrap()
                        - c_from_sp[i][j])
                        .abs()
                        <= f64::EPSILON
                );
            }
        }
    }
}
