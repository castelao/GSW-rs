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
fn specvol() {
    let mut input =
        File::open("tests/data/gsw_volume_validation.bin").expect("Unable to open file");
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
fn alpha() {
    let mut input =
        File::open("tests/data/gsw_volume_validation.bin").expect("Unable to open file");
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
    let mut input =
        File::open("tests/data/gsw_volume_validation.bin").expect("Unable to open file");
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
fn specvol_anom_standard() {
    let mut input =
        File::open("tests/data/gsw_volume_validation.bin").expect("Unable to open file");
    let mut contents = vec![];
    input
        .read_to_end(&mut contents)
        .expect("Failed to read content");

    let out: DataRef = from_bytes(&contents).unwrap();

    let p = out.data2d.get(&String::from("p_chck_cast")).unwrap();
    let sa = out.data2d.get(&String::from("SA_chck_cast")).unwrap();
    let ct = out.data2d.get(&String::from("CT_chck_cast")).unwrap();
    let specvol_anom_standard = out
        .data2d
        .get(&String::from("specvol_anom_standard"))
        .unwrap();
    for i in 0..3 {
        for j in 0..45 {
            if !specvol_anom_standard[i][j].is_nan() {
                assert!(
                    (gsw::volume::specvol_anom_standard(sa[i][j], ct[i][j], p[i][j]).unwrap()
                        - specvol_anom_standard[i][j])
                        .abs()
                        <= f64::EPSILON
                );
            }
        }
    }
}

#[test]
#[cfg(not(windows))]
fn sigma0() {
    let mut input =
        File::open("tests/data/gsw_volume_validation.bin").expect("Unable to open file");
    let mut contents = vec![];
    input
        .read_to_end(&mut contents)
        .expect("Failed to read content");

    let out: DataRef = from_bytes(&contents).unwrap();

    let sa = out.data2d.get(&String::from("SA_chck_cast")).unwrap();
    let ct = out.data2d.get(&String::from("CT_chck_cast")).unwrap();
    let sigma0 = out.data2d.get(&String::from("sigma0")).unwrap();
    let tol = 1e-12;
    for i in 0..3 {
        for j in 0..45 {
            if !sigma0[i][j].is_nan() {
                assert!(
                    (gsw::volume::sigma0(sa[i][j], ct[i][j]).unwrap() - sigma0[i][j]).abs() <= tol
                );
            }
        }
    }
}

#[test]
#[cfg(not(windows))]
fn sigma1() {
    let mut input =
        File::open("tests/data/gsw_volume_validation.bin").expect("Unable to open file");
    let mut contents = vec![];
    input
        .read_to_end(&mut contents)
        .expect("Failed to read content");

    let out: DataRef = from_bytes(&contents).unwrap();

    let sa = out.data2d.get(&String::from("SA_chck_cast")).unwrap();
    let ct = out.data2d.get(&String::from("CT_chck_cast")).unwrap();
    let sigma1 = out.data2d.get(&String::from("sigma1")).unwrap();
    let tol = 1e-12;
    for i in 0..3 {
        for j in 0..45 {
            if !sigma1[i][j].is_nan() {
                assert!(
                    (gsw::volume::sigma1(sa[i][j], ct[i][j]).unwrap() - sigma1[i][j]).abs() <= tol
                );
            }
        }
    }
}

#[test]
#[cfg(not(windows))]
fn sigma2() {
    let mut input =
        File::open("tests/data/gsw_volume_validation.bin").expect("Unable to open file");
    let mut contents = vec![];
    input
        .read_to_end(&mut contents)
        .expect("Failed to read content");

    let out: DataRef = from_bytes(&contents).unwrap();

    let sa = out.data2d.get(&String::from("SA_chck_cast")).unwrap();
    let ct = out.data2d.get(&String::from("CT_chck_cast")).unwrap();
    let sigma2 = out.data2d.get(&String::from("sigma2")).unwrap();
    let tol = 1e-12;
    for i in 0..3 {
        for j in 0..45 {
            if !sigma2[i][j].is_nan() {
                assert!(
                    (gsw::volume::sigma2(sa[i][j], ct[i][j]).unwrap() - sigma2[i][j]).abs() <= tol
                );
            }
        }
    }
}

#[test]
#[cfg(not(windows))]
fn sigma3() {
    let mut input =
        File::open("tests/data/gsw_volume_validation.bin").expect("Unable to open file");
    let mut contents = vec![];
    input
        .read_to_end(&mut contents)
        .expect("Failed to read content");

    let out: DataRef = from_bytes(&contents).unwrap();

    let sa = out.data2d.get(&String::from("SA_chck_cast")).unwrap();
    let ct = out.data2d.get(&String::from("CT_chck_cast")).unwrap();
    let sigma3 = out.data2d.get(&String::from("sigma3")).unwrap();
    let tol = 1e-12;
    for i in 0..3 {
        for j in 0..45 {
            if !sigma3[i][j].is_nan() {
                assert!(
                    (gsw::volume::sigma3(sa[i][j], ct[i][j]).unwrap() - sigma3[i][j]).abs() <= tol
                );
            }
        }
    }
}

#[test]
#[cfg(not(windows))]
fn sigma4() {
    let mut input =
        File::open("tests/data/gsw_volume_validation.bin").expect("Unable to open file");
    let mut contents = vec![];
    input
        .read_to_end(&mut contents)
        .expect("Failed to read content");

    let out: DataRef = from_bytes(&contents).unwrap();

    let sa = out.data2d.get(&String::from("SA_chck_cast")).unwrap();
    let ct = out.data2d.get(&String::from("CT_chck_cast")).unwrap();
    let sigma4 = out.data2d.get(&String::from("sigma4")).unwrap();
    let tol = 1e-12;
    for i in 0..3 {
        for j in 0..45 {
            if !sigma4[i][j].is_nan() {
                assert!(
                    (gsw::volume::sigma4(sa[i][j], ct[i][j]).unwrap() - sigma4[i][j]).abs() <= tol
                );
            }
        }
    }
}

// Must use p_chck_cast_shallow & p_chck_cast_deep
/*
#[test]
#[cfg(not(windows))]
fn enthalpy_diff() {
    let mut input = File::open("tests/data/gsw_volume_validation.bin").expect("Unable to open file");
    let mut contents = vec![];
    input
        .read_to_end(&mut contents)
        .expect("Failed to read content");

    let out: DataRef = from_bytes(&contents).unwrap();

    let p = out.data2d.get(&String::from("p_chck_cast")).unwrap();
    let sa = out.data2d.get(&String::from("SA_chck_cast")).unwrap();
    let ct = out.data2d.get(&String::from("CT_chck_cast")).unwrap();
    let enthalpy_diff = out.data2d.get(&String::from("enthalpy_diff")).unwrap();
    let tol = if cfg!(feature = "compat") || (f64::EPSILON > 1e-10) {
        f64::EPSILON
    } else {
        1e-10
    };
    for i in 0..3 {
        for j in 0..45 {
            if !enthalpy_diff[i][j].is_nan() {
                assert_eq!(
                    gsw::volume::enthalpy_diff(sa[i][j], ct[i][j], p[i][j], 0.0).unwrap(),
                    enthalpy_diff[i][j]
                );
                assert!(
                    (gsw::volume::enthalpy_diff(sa[i][j], ct[i][j], p[i][j], 0.0).unwrap()
                        - enthalpy_diff[i][j])
                        .abs()
                        <= tol
                );
            }
        }
    }
}
*/

#[test]
#[cfg(not(windows))]
fn rho() {
    let mut input =
        File::open("tests/data/gsw_volume_validation.bin").expect("Unable to open file");
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

/*
#[test]
#[cfg(not(windows))]
fn specvol() {
    let mut input =
        File::open("tests/data/gsw_volume_validation.bin").expect("Unable to open file");
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
*/

#[test]
#[cfg(not(windows))]
fn sound_speed() {
    let mut input =
        File::open("tests/data/gsw_volume_validation.bin").expect("Unable to open file");
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

// kappa

#[test]
#[cfg(not(windows))]
fn internal_energy() {
    let mut input =
        File::open("tests/data/gsw_volume_validation.bin").expect("Unable to open file");
    let mut contents = vec![];
    input
        .read_to_end(&mut contents)
        .expect("Failed to read content");

    let out: DataRef = from_bytes(&contents).unwrap();

    let p = out.data2d.get(&String::from("p_chck_cast")).unwrap();
    let sa = out.data2d.get(&String::from("SA_chck_cast")).unwrap();
    let ct = out.data2d.get(&String::from("CT_chck_cast")).unwrap();
    let internal_energy = out.data2d.get(&String::from("internal_energy")).unwrap();
    let tol = if cfg!(feature = "compat") || (f64::EPSILON > 1e-10) {
        // f64::EPSILON
        1e-11
    } else {
        1e-10
    };
    for i in 0..3 {
        for j in 0..45 {
            if !internal_energy[i][j].is_nan() {
                assert!(
                    (gsw::volume::internal_energy(sa[i][j], ct[i][j], p[i][j]).unwrap()
                        - internal_energy[i][j])
                        .abs()
                        <= tol
                );
            }
        }
    }
}

#[test]
#[cfg(not(windows))]
fn dynamic_enthalpy() {
    let mut input =
        File::open("tests/data/gsw_volume_validation.bin").expect("Unable to open file");
    let mut contents = vec![];
    input
        .read_to_end(&mut contents)
        .expect("Failed to read content");

    let out: DataRef = from_bytes(&contents).unwrap();

    let p = out.data2d.get(&String::from("p_chck_cast")).unwrap();
    let sa = out.data2d.get(&String::from("SA_chck_cast")).unwrap();
    let ct = out.data2d.get(&String::from("CT_chck_cast")).unwrap();
    let dynamic_enthalpy = out.data2d.get(&String::from("dynamic_enthalpy")).unwrap();
    let tol = if cfg!(feature = "compat") || (f64::EPSILON > 1e-10) {
        // f64::EPSILON
        1e-11
    } else {
        1e-10
    };
    for i in 0..3 {
        for j in 0..45 {
            if !dynamic_enthalpy[i][j].is_nan() {
                assert!(
                    (gsw::volume::dynamic_enthalpy(sa[i][j], ct[i][j], p[i][j]).unwrap()
                        - dynamic_enthalpy[i][j])
                        .abs()
                        <= tol
                );
            }
        }
    }
}

#[test]
#[cfg(not(windows))]
fn sa_from_rho() {
    let mut input =
        File::open("tests/data/gsw_volume_validation.bin").expect("Unable to open file");
    let mut contents = vec![];
    input
        .read_to_end(&mut contents)
        .expect("Failed to read content");

    let out: DataRef = from_bytes(&contents).unwrap();

    let rho = out.data2d.get(&String::from("rho")).unwrap();
    let p = out.data2d.get(&String::from("p_chck_cast")).unwrap();
    let ct = out.data2d.get(&String::from("CT_chck_cast")).unwrap();
    let sa_from_rho = out.data2d.get(&String::from("SA_from_rho")).unwrap();
    let tol = if cfg!(feature = "compat") || (f64::EPSILON > 1e-12) {
        f64::EPSILON
    } else {
        1e-12
    };
    for i in 0..3 {
        for j in 0..45 {
            if !sa_from_rho[i][j].is_nan() {
                assert!(
                    (gsw::volume::sa_from_rho(rho[i][j], ct[i][j], p[i][j]).unwrap()
                        - sa_from_rho[i][j])
                        .abs()
                        <= tol
                );
            }
        }
    }
}
