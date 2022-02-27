//! Specific volume, density, and enthalpy (75-term polynomial approximation)
//!

use crate::gsw_internal_const::*;
use crate::gsw_internal_funcs::non_dimensional_p;
use crate::gsw_specvol_coefficients::*;
use crate::{Error, Result};

pub use crate::gsw_internal_funcs::specvol_sso_0;

#[inline]
/// Non-dimensional salinity
fn non_dimensional_sa(sa: f64) -> Result<f64> {
    // Other implementations force negative SA to be 0. That is dangerous
    // since it can hide error by processing unrealistic inputs
    let sa: f64 = if sa < 0.0 {
        if cfg!(feature = "compat") {
            0.0
        } else if cfg!(feature = "invalidasnan") {
            return Ok(f64::NAN);
        } else {
            return Err(Error::NegativeSalinity);
        }
    } else {
        sa
    };

    Ok(libm::sqrt(GSW_SFAC * sa + OFFSET))
}

/// Specific volume of sea water (75-term polynomial approximation)
///
/// Calculates specific volume from Absolute Salinity, Conservative
/// Temperature and pressure, using the computationally-efficient
/// polynomial expression for specific volume (Roquet et al., 2014).
///
/// # Arguments
///
/// * `sa`: Absolute Salinity \[ g kg-1 \]
/// * `ct`: Conservative Temperature (ITS-90) \[ deg C \]
/// * `p`: sea pressure \[ dbar \] (i.e. absolute pressure - 10.1325 dbar)
///
/// # Returns
///
/// * `specvol`: specific volume \[m3 kg-1\]
///
/// # Example:
/// ```
/// use gsw::volume::specvol;
/// let v = specvol(32.0, 10.0, 100.0).unwrap();
/// assert!((v - 0.0009756515980668401).abs() <= f64::EPSILON);
/// ```
///
/// Note that the coefficients v(i,j,k) follow the convention in the original
/// paper, which is different from the convention used in the C-library.
///
pub fn specvol(sa: f64, ct: f64, p: f64) -> Result<f64> {
    let xs: f64 = non_dimensional_sa(sa)?;
    let ys: f64 = ct / GSW_CTU;
    let z: f64 = non_dimensional_p(p);

    // Specific Volume
    Ok(V000
        + xs * (V100 + xs * (V200 + xs * (V300 + xs * (V400 + xs * (V500 + xs * V600)))))
        + ys * (V010
            + xs * (V110 + xs * (V210 + xs * (V310 + xs * (V410 + xs * V510))))
            + ys * (V020
                + xs * (V120 + xs * (V220 + xs * (V320 + xs * V420)))
                + ys * (V030
                    + xs * (V130 + xs * (V230 + xs * V330))
                    + ys * (V040
                        + xs * (V140 + xs * V240)
                        + ys * (V050 + xs * V150 + ys * V060)))))
        + z * (V001
            + xs * (V101 + xs * (V201 + xs * (V301 + xs * (V401 + xs * V501))))
            + ys * (V011
                + xs * (V111 + xs * (V211 + xs * (V311 + xs * V411)))
                + ys * (V021
                    + xs * (V121 + xs * (V221 + xs * V321))
                    + ys * (V031
                        + xs * (V131 + xs * V231)
                        + ys * (V041 + xs * V141 + ys * V051))))
            + z * (V002
                + xs * (V102 + xs * (V202 + xs * (V302 + xs * V402)))
                + ys * (V012
                    + xs * (V112 + xs * (V212 + xs * V312))
                    + ys * (V022
                        + xs * (V122 + xs * V222)
                        + ys * (V032 + xs * V132 + ys * V042)))
                + z * (V003
                    + xs * (V103 + xs * V203)
                    + ys * (V013 + xs * V113 + ys * V023)
                    + z * (V004 + xs * V104 + ys * V014 + z * (V005 + z * V006))))))
}

#[cfg(test)]
mod test_specvol {
    use super::specvol;

    #[test]
    // NaN input results in NaN output.
    // Other libraries using GSW-rs might rely on this behavior to propagate
    // and handle invalid elements.
    fn nan() {
        let v = specvol(f64::NAN, 1.0, 1.0);
        assert!(v.unwrap().is_nan());

        let v = specvol(1.0, f64::NAN, 1.0);
        assert!(v.unwrap().is_nan());

        let v = specvol(1.0, 1.0, f64::NAN);
        assert!(v.unwrap().is_nan());
    }

    #[test]
    // Test value from Roquet 2015 Appendix C.3, rounded to 9.732819628e-04
    fn roquet2015() {
        assert!((specvol(30., 10., 1000.0).unwrap() - 9.732819628e-04).abs() <= 5e-14);
    }

    #[allow(clippy::excessive_precision)]
    #[test]
    fn test_specvol() {
        if cfg!(feature = "compat") {
            // Test value from C library.
            assert!(
                (specvol(34.507499465692057, 27.994827331978655, 0.0).unwrap()
                    - 0.00097855432330275953)
                    .abs()
                    < f64::EPSILON
            );
        }
    }

    #[test]
    fn extreme_values_of_sa() {
        assert!((specvol(0., 10., 100.0).unwrap() - 9.997742842214592e-4).abs() <= f64::EPSILON);
        assert!((specvol(50., 10., 100.0).unwrap() - 9.625729186157327e-4).abs() <= f64::EPSILON);
    }
}

/// in-situ density
///
/// Calculates in-situ density from Absolute Salinity and Conservative
/// Temperature, using the computationally-efficient expression for
/// specific volume in terms of SA, CT and p (Roquet et al., 2014).
///
/// # Arguments
///
/// * `sa`: Absolute Salinity \[ g kg-1 \]
/// * `ct`: Conservative Temperature (ITS-90) \[ deg C \]
/// * `p`: sea pressure \[ dbar \] (i.e. absolute pressure - 10.1325 dbar)
///
/// # Returns
///
/// * `rho`: in-situ density \[ kg m-3 \]
///
/// # Example:
/// ```
/// use gsw::volume::rho;
/// let density = rho(33.0, 10.0, 100.0).unwrap();
/// assert!((density - 1025.72882658687).abs() <= f64::EPSILON);
/// ```
pub fn rho(sa: f64, ct: f64, p: f64) -> Result<f64> {
    Ok(1.0 / specvol(sa, ct, p)?)
}

#[cfg(test)]
mod test_rho {
    use super::rho;

    #[test]
    // NaN input results in NaN output.
    // Other libraries using GSW-rs might rely on this behavior to propagate
    // and handle invalid elements.
    fn nan() {
        let density = rho(f64::NAN, 1.0, 1.0);
        assert!(density.unwrap().is_nan());

        let density = rho(1.0, f64::NAN, 1.0);
        assert!(density.unwrap().is_nan());

        let density = rho(1.0, 1.0, f64::NAN);
        assert!(density.unwrap().is_nan());
    }
}
/// Thermal expansion coefficient with respect to Conservative Temperature
/// (75-term polynomial approximation)
///
/// # Arguments
///
/// * `sa`: Absolute Salinity \[ g kg-1 \]
/// * `ct`: Conservative Temperature (ITS-90) \[ deg C \]
/// * `p`: sea pressure \[ dbar \] (i.e. absolute pressure - 10.1325 dbar)
///
pub fn alpha(sa: f64, ct: f64, p: f64) -> Result<f64> {
    let xs: f64 = non_dimensional_sa(sa)?;
    let ys: f64 = ct / GSW_CTU;
    let z: f64 = non_dimensional_p(p);

    let v_ct: f64 = A000
        + xs * (A100 + xs * (A200 + xs * (A300 + xs * (A400 + A500 * xs))))
        + ys * (A010
            + xs * (A110 + xs * (A210 + xs * (A310 + A410 * xs)))
            + ys * (A020
                + xs * (A120 + xs * (A220 + A320 * xs))
                + ys * (A030 + xs * (A130 + A230 * xs) + ys * (A040 + A140 * xs + A050 * ys))))
        + z * (A001
            + xs * (A101 + xs * (A201 + xs * (A301 + A401 * xs)))
            + ys * (A011
                + xs * (A111 + xs * (A211 + A311 * xs))
                + ys * (A021 + xs * (A121 + A221 * xs) + ys * (A031 + A131 * xs + A041 * ys)))
            + z * (A002
                + xs * (A102 + xs * (A202 + A302 * xs))
                + ys * (A012 + xs * (A112 + A212 * xs) + ys * (A022 + A122 * xs + A032 * ys))
                + z * (A003 + A103 * xs + A013 * ys + A004 * z)));

    Ok(0.025 * v_ct / specvol(sa, ct, p)?)
}

/// Saline contraction coefficient at constant Conservative Temperature
/// (75-term polynomial approximation)
///
/// # Arguments
///
/// * `sa`: Absolute Salinity \[ g kg-1 \]
/// * `ct`: Conservative Temperature (ITS-90) \[ deg C \]
/// * `p`: sea pressure \[ dbar \] (i.e. absolute pressure - 10.1325 dbar)
///
pub fn beta(sa: f64, ct: f64, p: f64) -> Result<f64> {
    let xs: f64 = non_dimensional_sa(sa)?;
    let ys: f64 = ct / GSW_CTU;
    let z: f64 = non_dimensional_p(p);

    let v_sa: f64 = B000
        + xs * (B100 + xs * (B200 + xs * (B300 + xs * (B400 + B500 * xs))))
        + ys * (B010
            + xs * (B110 + xs * (B210 + xs * (B310 + B410 * xs)))
            + ys * (B020
                + xs * (B120 + xs * (B220 + B320 * xs))
                + ys * (B030 + xs * (B130 + B230 * xs) + ys * (B040 + B140 * xs + B050 * ys))))
        + z * (B001
            + xs * (B101 + xs * (B201 + xs * (B301 + B401 * xs)))
            + ys * (B011
                + xs * (B111 + xs * (B211 + B311 * xs))
                + ys * (B021 + xs * (B121 + B221 * xs) + ys * (B031 + B131 * xs + B041 * ys)))
            + z * (B002
                + xs * (B102 + xs * (B202 + B302 * xs))
                + ys * (B012 + xs * (B112 + B212 * xs) + ys * (B022 + B122 * xs + B032 * ys))
                + z * (B003 + B103 * xs + B013 * ys + B004 * z)));

    Ok(-v_sa * 0.5 * GSW_SFAC / (specvol(sa, ct, p)? * xs))
}

/// The raio alpha/beta (75-term polynomial approximation)
///
/// # Arguments
///
/// * `sa`: Absolute Salinity \[ g kg-1 \]
/// * `ct`: Conservative Temperature (ITS-90) \[ deg C \]
/// * `p`: sea pressure \[ dbar \] (i.e. absolute pressure - 10.1325 dbar)
///
/// # Example:
/// ```
/// use gsw::volume::alpha_on_beta;
/// let ratio = alpha_on_beta(33.0, 10.0, 100.0).unwrap();
/// assert!((ratio - 0.21698852133695548).abs() <= f64::EPSILON);
/// ```
pub fn alpha_on_beta(sa: f64, ct: f64, p: f64) -> Result<f64> {
    let s: f64 = non_dimensional_sa(sa)?;
    let tau: f64 = ct / GSW_CTU;
    let pi: f64 = non_dimensional_p(p);

    let v_ct: f64 = A000
        + s * (A100 + s * (A200 + s * (A300 + s * (A400 + A500 * s))))
        + tau
            * (A010
                + s * (A110 + s * (A210 + s * (A310 + A410 * s)))
                + tau
                    * (A020
                        + s * (A120 + s * (A220 + A320 * s))
                        + tau
                            * (A030
                                + s * (A130 + A230 * s)
                                + tau * (A040 + A140 * s + A050 * tau))))
        + pi * (A001
            + s * (A101 + s * (A201 + s * (A301 + A401 * s)))
            + tau
                * (A011
                    + s * (A111 + s * (A211 + A311 * s))
                    + tau * (A021 + s * (A121 + A221 * s) + tau * (A031 + A131 * s + A041 * tau)))
            + pi * (A002
                + s * (A102 + s * (A202 + A302 * s))
                + tau * (A012 + s * (A112 + A212 * s) + tau * (A022 + A122 * s + A032 * tau))
                + pi * (A003 + A103 * s + A013 * tau + A004 * pi)));

    let v_sa: f64 = B000
        + s * (B100 + s * (B200 + s * (B300 + s * (B400 + B500 * s))))
        + tau
            * (B010
                + s * (B110 + s * (B210 + s * (B310 + B410 * s)))
                + tau
                    * (B020
                        + s * (B120 + s * (B220 + B320 * s))
                        + tau
                            * (B030
                                + s * (B130 + B230 * s)
                                + tau * (B040 + B140 * s + B050 * tau))))
        + pi * (B001
            + s * (B101 + s * (B201 + s * (B301 + B401 * s)))
            + tau
                * (B011
                    + s * (B111 + s * (B211 + B311 * s))
                    + tau * (B021 + s * (B121 + B221 * s) + tau * (B031 + B131 * s + B041 * tau)))
            + pi * (B002
                + s * (B102 + s * (B202 + B302 * s))
                + tau * (B012 + s * (B112 + B212 * s) + tau * (B022 + B122 * s + B032 * tau))
                + pi * (B003 + B103 * s + B013 * tau + B004 * pi)));

    Ok(-v_ct * s / (20.0 * GSW_SFAC * v_sa))
}

#[cfg(test)]
mod test_alpha_on_beta {
    use super::{alpha_on_beta, Error};

    #[test]
    // NaN input results in NaN output.
    // Other libraries using GSW-rs might rely on this behavior to propagate
    // and handle invalid elements.
    fn nan() {
        let ratio = alpha_on_beta(f64::NAN, 1.0, 1.0).unwrap();
        assert!(ratio.is_nan());

        let ratio = alpha_on_beta(1.0, f64::NAN, 1.0).unwrap();
        assert!(ratio.is_nan());

        let ratio = alpha_on_beta(1.0, 1.0, f64::NAN).unwrap();
        assert!(ratio.is_nan());
    }

    #[test]
    fn negative_sa() {
        let ratio = alpha_on_beta(-0.1, 10.0, 100.0);

        if cfg!(feature = "compat") {
            assert!((ratio.unwrap() - 0.1016043030015299).abs() <= f64::EPSILON);
        } else if cfg!(feature = "invalidasnan") {
            assert!(ratio.unwrap().is_nan());
        } else {
            match ratio {
                Err(Error::NegativeSalinity) => (),
                _ => panic!("It should be Error::NegativeSalinity"),
            }
        }
    }
}

/// in-situ density, thermal expansion & saline contraction coefficients
/// (75-term polynomial approximation)
///
/// # Arguments
///
/// * `sa`: Absolute Salinity \[ g kg-1 \]
/// * `ct`: Conservative Temperature (ITS-90) \[ deg C \]
/// * `p`: sea pressure \[ dbar \] (i.e. absolute pressure - 10.1325 dbar)
///
pub fn rho_alpha_beta(sa: f64, ct: f64, p: f64) -> Result<(f64, f64, f64)> {
    let rho = rho(sa, ct, p)?;
    let alpha = alpha(sa, ct, p)?;
    let beta = beta(sa, ct, p)?;

    Ok((rho, alpha, beta))
}

///
/// # Arguments
///
/// * `sa`: Absolute Salinity \[ g kg-1 \]
/// * `ct`: Conservative Temperature (ITS-90) \[ deg C \]
/// * `p`: sea pressure \[ dbar \] (i.e. absolute pressure - 10.1325 dbar)
///
pub fn specvol_alpha_beta(sa: f64, ct: f64, p: f64) -> Result<(f64, f64, f64)> {
    let specvol = specvol(sa, ct, p)?;
    let alpha = alpha(sa, ct, p)?;
    let beta = beta(sa, ct, p)?;

    Ok((specvol, alpha, beta))
}

/// First order derivatives of specific volume
/// (75-term polynomial approximation)
///
/// # Arguments
///
/// * `sa`: Absolute Salinity \[ g kg-1 \]
/// * `ct`: Conservative Temperature (ITS-90) \[ deg C \]
/// * `p`: sea pressure \[ dbar \] (i.e. absolute pressure - 10.1325 dbar)
///
/// # Example:
/// ```
/// use gsw::volume::specvol_first_derivatives;
/// let (dvdsa, dvdct, dvdp) = specvol_first_derivatives(32.0, 10.0, 100.0).unwrap();
/// assert!((dvdsa + 7.355503539675526e-07).abs() <= f64::EPSILON);
/// assert!((dvdct - 1.5715643283187364e-07).abs() <= f64::EPSILON);
/// assert!((dvdp + 4.3021618664421085e-13).abs() <= f64::EPSILON);
/// ```
pub fn specvol_first_derivatives(sa: f64, ct: f64, p: f64) -> Result<(f64, f64, f64)> {
    let s: f64 = non_dimensional_sa(sa)?;
    let tau: f64 = ct / GSW_CTU;
    let pi: f64 = non_dimensional_p(p);

    let v_ct_part: f64 = A000
        + s * (A100 + s * (A200 + s * (A300 + s * (A400 + A500 * s))))
        + tau
            * (A010
                + s * (A110 + s * (A210 + s * (A310 + A410 * s)))
                + tau
                    * (A020
                        + s * (A120 + s * (A220 + A320 * s))
                        + tau
                            * (A030
                                + s * (A130 + A230 * s)
                                + tau * (A040 + A140 * s + A050 * tau))))
        + pi * (A001
            + s * (A101 + s * (A201 + s * (A301 + A401 * s)))
            + tau
                * (A011
                    + s * (A111 + s * (A211 + A311 * s))
                    + tau * (A021 + s * (A121 + A221 * s) + tau * (A031 + A131 * s + A041 * tau)))
            + pi * (A002
                + s * (A102 + s * (A202 + A302 * s))
                + tau * (A012 + s * (A112 + A212 * s) + tau * (A022 + A122 * s + A032 * tau))
                + pi * (A003 + A103 * s + A013 * tau + A004 * pi)));

    let v_ct = 0.025 * v_ct_part;

    let v_sa_part: f64 = B000
        + s * (B100 + s * (B200 + s * (B300 + s * (B400 + B500 * s))))
        + tau
            * (B010
                + s * (B110 + s * (B210 + s * (B310 + B410 * s)))
                + tau
                    * (B020
                        + s * (B120 + s * (B220 + B320 * s))
                        + tau
                            * (B030
                                + s * (B130 + B230 * s)
                                + tau * (B040 + B140 * s + B050 * tau))))
        + pi * (B001
            + s * (B101 + s * (B201 + s * (B301 + B401 * s)))
            + tau
                * (B011
                    + s * (B111 + s * (B211 + B311 * s))
                    + tau * (B021 + s * (B121 + B221 * s) + tau * (B031 + B131 * s + B041 * tau)))
            + pi * (B002
                + s * (B102 + s * (B202 + B302 * s))
                + tau * (B012 + s * (B112 + B212 * s) + tau * (B022 + B122 * s + B032 * tau))
                + pi * (B003 + B103 * s + B013 * tau + B004 * pi)));

    let v_sa = 0.5 * GSW_SFAC * v_sa_part / s;

    let v_p_part = C000
        + s * (C100 + s * (C200 + s * (C300 + s * (C400 + C500 * s))))
        + tau
            * (C010
                + s * (C110 + s * (C210 + s * (C310 + C410 * s)))
                + tau
                    * (C020
                        + s * (C120 + s * (C220 + C320 * s))
                        + tau
                            * (C030
                                + s * (C130 + C230 * s)
                                + tau * (C040 + C140 * s + C050 * tau))))
        + pi * (C001
            + s * (C101 + s * (C201 + s * (C301 + C401 * s)))
            + tau
                * (C011
                    + s * (C111 + s * (C211 + C311 * s))
                    + tau * (C021 + s * (C121 + C221 * s) + tau * (C031 + C131 * s + C041 * tau)))
            + pi * (C002
                + s * (C102 + C202 * s)
                + tau * (C012 + C112 * s + C022 * tau)
                + pi * (C003 + C103 * s + C013 * tau + pi * (C004 + C005 * pi))));

    let v_p = 1e-8 * v_p_part;

    Ok((v_sa, v_ct, v_p))
}

#[cfg(test)]
mod test_specvol_first_derivatives {
    use super::{specvol_first_derivatives, Error};

    #[test]
    // NaN input results in NaN output.
    // Other libraries using GSW-rs might rely on this behavior to propagate
    // and handle invalid elements.
    fn nan() {
        let (dsa, dct, dp) = specvol_first_derivatives(f64::NAN, 1.0, 1.0).unwrap();
        assert!(dsa.is_nan());
        assert!(dct.is_nan());
        assert!(dp.is_nan());

        let (dsa, dct, dp) = specvol_first_derivatives(1.0, f64::NAN, 1.0).unwrap();
        assert!(dsa.is_nan());
        assert!(dct.is_nan());
        assert!(dp.is_nan());

        let (dsa, dct, dp) = specvol_first_derivatives(1.0, 1.0, f64::NAN).unwrap();
        assert!(dsa.is_nan());
        assert!(dct.is_nan());
        assert!(dp.is_nan());
    }

    #[test]
    fn negative_sa() {
        let ans = specvol_first_derivatives(-0.1, 10.0, 100.0);

        if cfg!(feature = "compat") {
            let (dsa, dct, dp) = ans.unwrap();
            assert!((dsa + 7.820648830135304e-7).abs() <= f64::EPSILON);
            assert!((dct - 7.946115734056278e-8).abs() <= f64::EPSILON);
            assert!((dp + 4.774798120959369e-13).abs() <= f64::EPSILON);
        } else if cfg!(feature = "invalidasnan") {
            let (dsa, dct, dp) = ans.unwrap();
            assert!(dsa.is_nan());
            assert!(dct.is_nan());
            assert!(dp.is_nan());
        } else {
            match ans {
                Err(Error::NegativeSalinity) => (),
                _ => panic!("It should be Error::NegativeSalinity"),
            }
        }
    }
}

/// Second order derivatives of specific volume
/// (75-term polynomial approximation)
///
/// # Arguments
///
/// * `sa`: Absolute Salinity \[ g kg-1 \]
/// * `ct`: Conservative Temperature (ITS-90) \[ deg C \]
/// * `p`: sea pressure \[ dbar \] (i.e. absolute pressure - 10.1325 dbar)
pub fn specvol_second_derivatives(sa: f64, ct: f64, p: f64) -> Result<(f64, f64, f64, f64, f64)> {
    // Same procedure of non_dimensional_sa, but here we also want s2.
    let sa: f64 = if sa < 0.0 {
        if cfg!(feature = "compat") {
            0.0
        } else if cfg!(feature = "invalidasnan") {
            return Ok((f64::NAN, f64::NAN, f64::NAN, f64::NAN, f64::NAN));
        } else {
            return Err(Error::NegativeSalinity);
        }
    } else {
        sa
    };

    let s2 = GSW_SFAC * sa + OFFSET;
    let s = libm::sqrt(s2);

    let tau: f64 = ct / GSW_CTU;
    let pi: f64 = non_dimensional_p(p);

    let v_sa_sa_part = (-B000
        + s2 * (B200 + s * (2.0 * B300 + s * (3.0 * B400 + 4.0 * B500 * s)))
        + tau
            * (-B010
                + s2 * (B210 + s * (2.0 * B310 + 3.0 * B410 * s))
                + tau
                    * (-B020
                        + s2 * (B220 + 2.0 * B320 * s)
                        + tau * (-B030 + B230 * s2 + tau * (-B040 - B050 * tau))))
        + pi * (-B001
            + s2 * (B201 + s * (2.0 * B301 + 3.0 * B401 * s))
            + tau
                * (-B011
                    + s2 * (B211 + 2.0 * B311 * s)
                    + tau * (-B021 + B221 * s2 + tau * (-B031 - B041 * tau)))
            + pi * (-B002
                + s2 * (B202 + 2.0 * B302 * s)
                + tau * (-B012 + B212 * s2 + tau * (-B022 - B032 * tau))
                + pi * (-B003 - B013 * tau - B004 * pi))))
        / s2;

    let v_sa_sa = 0.25 * GSW_SFAC * GSW_SFAC * v_sa_sa_part / s;

    let v_sa_ct_part = (B010
        + s * (B110 + s * (B210 + s * (B310 + B410 * s)))
        + tau
            * (2.0 * (B020 + s * (B120 + s * (B220 + B320 * s)))
                + tau
                    * (3.0 * (B030 + s * (B130 + B230 * s))
                        + tau * (4.0 * (B040 + B140 * s) + 5.0 * B050 * tau)))
        + pi * (B011
            + s * (B111 + s * (B211 + B311 * s))
            + tau
                * (2.0 * (B021 + s * (B121 + B221 * s))
                    + tau * (3.0 * (B031 + B131 * s) + 4.0 * B041 * tau))
            + pi * (B012
                + s * (B112 + B212 * s)
                + tau * (2.0 * (B022 + B122 * s) + 3.0 * B032 * tau)
                + B013 * pi)))
        / s;

    let v_sa_ct = 0.025 * 0.5 * GSW_SFAC * v_sa_ct_part;

    let v_ct_ct_part = A010
        + s * (A110 + s * (A210 + s * (A310 + A410 * s)))
        + tau
            * (2.0 * (A020 + s * (A120 + s * (A220 + A320 * s)))
                + tau
                    * (3.0 * (A030 + s * (A130 + A230 * s))
                        + tau * (4.0 * (A040 + A140 * s) + 5.0 * A050 * tau)))
        + pi * (A011
            + s * (A111 + s * (A211 + A311 * s))
            + tau
                * (2.0 * (A021 + s * (A121 + A221 * s))
                    + tau * (3.0 * (A031 + A131 * s) + 4.0 * A041 * tau))
            + pi * (A012
                + s * (A112 + A212 * s)
                + tau * (2.0 * (A022 + A122 * s) + 3.0 * A032 * tau)
                + A013 * pi));

    let v_ct_ct = 0.025 * 0.025 * v_ct_ct_part;

    let v_sa_p_part = B001
        + s * (B101 + s * (B201 + s * (B301 + B401 * s)))
        + tau
            * (B011
                + s * (B111 + s * (B211 + B311 * s))
                + tau * (B021 + s * (B121 + B221 * s) + tau * (B031 + B131 * s + B041 * tau)))
        + pi * (2.0
            * (B002
                + s * (B102 + s * (B202 + B302 * s))
                + tau * (B012 + s * (B112 + B212 * s) + tau * (B022 + B122 * s + B032 * tau)))
            + pi * (3.0 * (B003 + B103 * s + B013 * tau) + 4.0 * B004 * pi));

    let v_sa_p = 1e-8 * 0.5 * GSW_SFAC * v_sa_p_part;

    let v_ct_p_part = A001
        + s * (A101 + s * (A201 + s * (A301 + A401 * s)))
        + tau
            * (A011
                + s * (A111 + s * (A211 + A311 * s))
                + tau * (A021 + s * (A121 + A221 * s) + tau * (A031 + A131 * s + A041 * tau)))
        + pi * (2.0
            * (A002
                + s * (A102 + s * (A202 + A302 * s))
                + tau * (A012 + s * (A112 + A212 * s) + tau * (A022 + A122 * s + A032 * tau)))
            + pi * (3.0 * (A003 + A103 * s + A013 * tau) + 4.0 * A004 * pi));

    let v_ct_p = 1e-8 * 0.025 * v_ct_p_part;

    Ok((v_sa_sa, v_sa_ct, v_ct_ct, v_sa_p, v_ct_p))
}

#[cfg(test)]
mod test_specvol_second_derivatives {
    use super::{specvol_second_derivatives, Error};

    #[test]
    // NaN input results in NaN output.
    // Other libraries using GSW-rs might rely on this behavior to propagate
    // and handle invalid elements.
    fn nan() {
        let (dsds, dsdt, dtdt, dsdp, dtdp) =
            specvol_second_derivatives(f64::NAN, 1.0, 1.0).unwrap();
        assert!(dsds.is_nan());
        assert!(dsdt.is_nan());
        assert!(dtdt.is_nan());
        assert!(dsdp.is_nan());
        assert!(dtdp.is_nan());

        let (dsds, dsdt, dtdt, dsdp, dtdp) =
            specvol_second_derivatives(1.0, f64::NAN, 1.0).unwrap();
        assert!(dsds.is_nan());
        assert!(dsdt.is_nan());
        assert!(dtdt.is_nan());
        assert!(dsdp.is_nan());
        assert!(dtdp.is_nan());

        let (dsds, dsdt, dtdt, dsdp, dtdp) =
            specvol_second_derivatives(1.0, 1.0, f64::NAN).unwrap();
        assert!(dsds.is_nan());
        assert!(dsdt.is_nan());
        assert!(dtdt.is_nan());
        assert!(dsdp.is_nan());
        assert!(dtdp.is_nan());
    }

    #[test]
    fn negative_sa() {
        let ans = specvol_second_derivatives(-0.1, 10.0, 100.0);

        if cfg!(feature = "compat") {
            let (dsds, dsdt, dtdt, dsdp, dtdp) = ans.unwrap();
            assert!((dsds - 4.171687573107866e-9).abs() <= f64::EPSILON);
            assert!((dsdt - 2.6980316664661234e-9).abs() <= f64::EPSILON);
            assert!((dtdt - 1.224276720857283e-8).abs() <= f64::EPSILON);
            assert!((dsdp - 1.2399041600177509e-15).abs() <= f64::EPSILON);
            assert!((dtdp - 2.4440494320176846e-15).abs() <= f64::EPSILON);
        } else if cfg!(feature = "invalidasnan") {
            let (dsds, dsdt, dtdt, dsdp, dtdp) = ans.unwrap();
            assert!(dsds.is_nan());
            assert!(dsdt.is_nan());
            assert!(dtdt.is_nan());
            assert!(dsdp.is_nan());
            assert!(dtdp.is_nan());
        } else {
            match ans {
                Err(Error::NegativeSalinity) => (),
                _ => panic!("It should be Error::NegativeSalinity"),
            }
        }
    }
}

/// Specific volume anomaly (75-term polynomial approximation)
///
/// # Arguments
///
/// * `sa`: Absolute Salinity \[ g kg-1 \]
/// * `ct`: Conservative Temperature (ITS-90) \[ deg C \]
/// * `p`: sea pressure \[ dbar \] (i.e. absolute pressure - 10.1325 dbar)
/// * `sa_ref`:
/// * `ct_ref`:
///
// fn specvol_anom(sa: f64, ct: f64, p: f64, sa_ref: Option<f64>, ct_ref: Option<f64>) -> Result<f64> {
pub fn specvol_anom(sa: f64, ct: f64, p: f64, sa_ref: f64, ct_ref: f64) -> Result<f64> {
    let xs: f64 = non_dimensional_sa(sa)?;
    let ys: f64 = ct / GSW_CTU;
    let z: f64 = non_dimensional_p(p);

    let xs_ref: f64 = non_dimensional_sa(sa_ref)?;
    let ys_ref: f64 = ct_ref / GSW_CTU;

    let xy_part_0 = xs * (V100 + xs * (V200 + xs * (V300 + xs * (V400 + xs * (V500 + V600 * xs)))))
        + ys * (V010
            + xs * (V110 + xs * (V210 + xs * (V310 + xs * (V410 + V510 * xs))))
            + ys * (V020
                + xs * (V120 + xs * (V220 + xs * (V320 + V420 * xs)))
                + ys * (V030
                    + xs * (V130 + xs * (V230 + V330 * xs))
                    + ys * (V040
                        + xs * (V140 + V240 * xs)
                        + ys * (V050 + V150 * xs + V060 * ys)))));

    let xy_part_1 = xs * (V101 + xs * (V201 + xs * (V301 + xs * (V401 + V501 * xs))))
        + ys * (V011
            + xs * (V111 + xs * (V211 + xs * (V311 + V411 * xs)))
            + ys * (V021
                + xs * (V121 + xs * (V221 + V321 * xs))
                + ys * (V031 + xs * (V131 + V231 * xs) + ys * (V041 + V141 * xs + V051 * ys))));

    let xy_part_2 = xs * (V102 + xs * (V202 + xs * (V302 + V402 * xs)))
        + ys * (V012
            + xs * (V112 + xs * (V212 + V312 * xs))
            + ys * (V022 + xs * (V122 + V222 * xs) + ys * (V032 + V132 * xs + V042 * ys)));

    let xy_part_3 = xs * (V103 + V203 * xs) + ys * (V013 + V113 * xs + V023 * ys);

    let xy_part_0_ref = xs_ref
        * (V100
            + xs_ref
                * (V200 + xs_ref * (V300 + xs_ref * (V400 + xs_ref * (V500 + V600 * xs_ref)))))
        + ys_ref
            * (V010
                + xs_ref
                    * (V110 + xs_ref * (V210 + xs_ref * (V310 + xs_ref * (V410 + V510 * xs_ref))))
                + ys_ref
                    * (V020
                        + xs_ref * (V120 + xs_ref * (V220 + xs_ref * (V320 + V420 * xs_ref)))
                        + ys_ref
                            * (V030
                                + xs_ref * (V130 + xs_ref * (V230 + V330 * xs_ref))
                                + ys_ref
                                    * (V040
                                        + xs_ref * (V140 + V240 * xs_ref)
                                        + ys_ref * (V050 + V150 * xs_ref + V060 * ys_ref)))));

    let xy_part_1_ref = xs_ref
        * (V101 + xs_ref * (V201 + xs_ref * (V301 + xs_ref * (V401 + V501 * xs_ref))))
        + ys_ref
            * (V011
                + xs_ref * (V111 + xs_ref * (V211 + xs_ref * (V311 + V411 * xs_ref)))
                + ys_ref
                    * (V021
                        + xs_ref * (V121 + xs_ref * (V221 + V321 * xs_ref))
                        + ys_ref
                            * (V031
                                + xs_ref * (V131 + V231 * xs_ref)
                                + ys_ref * (V041 + V141 * xs_ref + V051 * ys_ref))));

    let xy_part_2_ref = xs_ref * (V102 + xs_ref * (V202 + xs_ref * (V302 + V402 * xs_ref)))
        + ys_ref
            * (V012
                + xs_ref * (V112 + xs_ref * (V212 + V312 * xs_ref))
                + ys_ref
                    * (V022
                        + xs_ref * (V122 + V222 * xs_ref)
                        + ys_ref * (V032 + V132 * xs_ref + V042 * ys_ref)));

    let xy_part_3_ref =
        xs_ref * (V103 + V203 * xs_ref) + ys_ref * (V013 + V113 * xs_ref + V023 * ys_ref);

    let xy_part_4_diff = V104 * (xs - xs_ref) + V014 * (ys - ys_ref);

    Ok(xy_part_0 - xy_part_0_ref
        + z * (xy_part_1 - xy_part_1_ref
            + z * (xy_part_2 - xy_part_2_ref
                + z * (xy_part_3 - xy_part_3_ref + z * (xy_part_4_diff)))))
}

/// Specific Volume Anomaly of Standard Ocean Salinity and CT=0
///
/// Specific volume anomaly with reference of SA = SSO & CT = 0 (75-term equation)
///
/// # Arguments
///
/// * `sa`: Absolute Salinity \[g kg-1\]
/// * `ct`: Conservative Temperature (ITS-90) \[deg C\]
/// * `p`: sea pressure \[dbar\] (i.e. absolute pressure - 10.1325 dbar)
///
/// # Returns
///
/// * `specvol_anom`: specific volume anomaly of seawater [m3 kg-1]
///
pub fn specvol_anom_standard(sa: f64, ct: f64, p: f64) -> Result<f64> {
    Ok(specvol(sa, ct, p)? - crate::gsw_internal_funcs::specvol_sso_0(p))
}

#[cfg(test)]
mod test_specvol_anom_standard {
    use super::{specvol_anom_standard, GSW_SSO};

    #[test]
    /// This anomaly should be zero for SSO & CT=0 at any depth
    fn test_specvol_anom_standard_at_standard() {
        let p_to_test: [f64; 5] = [0., 10., 100., 1000., 5000.];
        for p in p_to_test.iter().cloned() {
            assert!((specvol_anom_standard(GSW_SSO, 0.0, p).unwrap() - 0.0).abs() < f64::EPSILON);
        }
    }
}

/// First order derivatives of density
/// (75-term polynomial approximation)
///
/// # Arguments
///
/// * `sa`: Absolute Salinity \[ g kg-1 \]
/// * `ct`: Conservative Temperature (ITS-90) \[ deg C \]
/// * `p`: sea pressure \[ dbar \] (i.e. absolute pressure - 10.1325 dbar)
/// # Example:
/// ```
/// use gsw::volume::rho_first_derivatives;
/// let (drho_dsa, drho_dct, drho_dp) = rho_first_derivatives(32.0, 10.0, 100.0).unwrap();
/// assert!((drho_dsa - 0.7727213082442861).abs() <= f64::EPSILON);
/// assert!((drho_dct + 0.16509831546108913).abs() <= f64::EPSILON);
/// assert!((drho_dp - 4.519571131716803e-07 ).abs() <= f64::EPSILON);
/// ```
pub fn rho_first_derivatives(sa: f64, ct: f64, p: f64) -> Result<(f64, f64, f64)> {
    let s: f64 = non_dimensional_sa(sa)?;
    let tau: f64 = ct / GSW_CTU;
    let pi: f64 = non_dimensional_p(p);

    // Specific Volume
    let v = V000
        + s * (V100 + s * (V200 + s * (V300 + s * (V400 + s * (V500 + s * V600)))))
        + tau
            * (V010
                + s * (V110 + s * (V210 + s * (V310 + s * (V410 + s * V510))))
                + tau
                    * (V020
                        + s * (V120 + s * (V220 + s * (V320 + s * V420)))
                        + tau
                            * (V030
                                + s * (V130 + s * (V230 + s * V330))
                                + tau
                                    * (V040
                                        + s * (V140 + s * V240)
                                        + tau * (V050 + s * V150 + tau * V060)))))
        + pi * (V001
            + s * (V101 + s * (V201 + s * (V301 + s * (V401 + s * V501))))
            + tau
                * (V011
                    + s * (V111 + s * (V211 + s * (V311 + s * V411)))
                    + tau
                        * (V021
                            + s * (V121 + s * (V221 + s * V321))
                            + tau
                                * (V031
                                    + s * (V131 + s * V231)
                                    + tau * (V041 + s * V141 + tau * V051))))
            + pi * (V002
                + s * (V102 + s * (V202 + s * (V302 + s * V402)))
                + tau
                    * (V012
                        + s * (V112 + s * (V212 + s * V312))
                        + tau
                            * (V022
                                + s * (V122 + s * V222)
                                + tau * (V032 + s * V132 + tau * V042)))
                + pi * (V003
                    + s * (V103 + s * V203)
                    + tau * (V013 + s * V113 + tau * V023)
                    + pi * (V004 + s * V104 + tau * V014 + pi * (V005 + pi * V006)))));

    let rho2 = 1.0 / (v * v);

    let v_sa = B000
        + s * (B100 + s * (B200 + s * (B300 + s * (B400 + B500 * s))))
        + tau
            * (B010
                + s * (B110 + s * (B210 + s * (B310 + B410 * s)))
                + tau
                    * (B020
                        + s * (B120 + s * (B220 + B320 * s))
                        + tau
                            * (B030
                                + s * (B130 + B230 * s)
                                + tau * (B040 + B140 * s + B050 * tau))))
        + pi * (B001
            + s * (B101 + s * (B201 + s * (B301 + B401 * s)))
            + tau
                * (B011
                    + s * (B111 + s * (B211 + B311 * s))
                    + tau * (B021 + s * (B121 + B221 * s) + tau * (B031 + B131 * s + B041 * tau)))
            + pi * (B002
                + s * (B102 + s * (B202 + B302 * s))
                + tau * (B012 + s * (B112 + B212 * s) + tau * (B022 + B122 * s + B032 * tau))
                + pi * (B003 + B103 * s + B013 * tau + B004 * pi)));

    let drho_dsa = -rho2 * 0.5 * GSW_SFAC * v_sa / s;

    let v_ct = A000
        + s * (A100 + s * (A200 + s * (A300 + s * (A400 + A500 * s))))
        + tau
            * (A010
                + s * (A110 + s * (A210 + s * (A310 + A410 * s)))
                + tau
                    * (A020
                        + s * (A120 + s * (A220 + A320 * s))
                        + tau
                            * (A030
                                + s * (A130 + A230 * s)
                                + tau * (A040 + A140 * s + A050 * tau))))
        + pi * (A001
            + s * (A101 + s * (A201 + s * (A301 + A401 * s)))
            + tau
                * (A011
                    + s * (A111 + s * (A211 + A311 * s))
                    + tau * (A021 + s * (A121 + A221 * s) + tau * (A031 + A131 * s + A041 * tau)))
            + pi * (A002
                + s * (A102 + s * (A202 + A302 * s))
                + tau * (A012 + s * (A112 + A212 * s) + tau * (A022 + A122 * s + A032 * tau))
                + pi * (A003 + A103 * s + A013 * tau + A004 * pi)));

    let drho_dct = -rho2 * 0.025 * v_ct;

    let v_p = C000
        + s * (C100 + s * (C200 + s * (C300 + s * (C400 + C500 * s))))
        + tau
            * (C010
                + s * (C110 + s * (C210 + s * (C310 + C410 * s)))
                + tau
                    * (C020
                        + s * (C120 + s * (C220 + C320 * s))
                        + tau
                            * (C030
                                + s * (C130 + C230 * s)
                                + tau * (C040 + C140 * s + C050 * tau))))
        + pi * (C001
            + s * (C101 + s * (C201 + s * (C301 + C401 * s)))
            + tau
                * (C011
                    + s * (C111 + s * (C211 + C311 * s))
                    + tau * (C021 + s * (C121 + C221 * s) + tau * (C031 + C131 * s + C041 * tau)))
            + pi * (C002
                + s * (C102 + C202 * s)
                + tau * (C012 + C112 * s + C022 * tau)
                + pi * (C003 + C103 * s + C013 * tau + pi * (C004 + C005 * pi))));

    let drho_dp = -rho2 * 1e-4 * PA2DB * v_p;

    Ok((drho_dsa, drho_dct, drho_dp))
}

#[cfg(test)]
mod test_rho_first_derivatives {
    use super::{rho_first_derivatives, Error};

    #[test]
    // NaN input results in NaN output.
    // Other libraries using GSW-rs might rely on this behavior to propagate
    // and handle invalid elements.
    fn nan() {
        let (drho_dsa, drho_dct, drho_dp) = rho_first_derivatives(f64::NAN, 1.0, 1.0).unwrap();
        assert!(drho_dsa.is_nan());
        assert!(drho_dct.is_nan());
        assert!(drho_dp.is_nan());

        let (drho_dsa, drho_dct, drho_dp) = rho_first_derivatives(1.0, f64::NAN, 1.0).unwrap();
        assert!(drho_dsa.is_nan());
        assert!(drho_dct.is_nan());
        assert!(drho_dp.is_nan());

        let (drho_dsa, drho_dct, drho_dp) = rho_first_derivatives(1.0, 1.0, f64::NAN).unwrap();
        assert!(drho_dsa.is_nan());
        assert!(drho_dct.is_nan());
        assert!(drho_dp.is_nan());
    }

    #[test]
    fn negative_sa() {
        let ans = rho_first_derivatives(-0.1, 10.0, 100.0);

        if cfg!(feature = "compat") {
            let (drho_dsa, drho_dct, drho_dp) = ans.unwrap();
            assert!((drho_dsa - 0.7824180513504084).abs() <= f64::EPSILON);
            assert!((drho_dct + 0.07949704076327348).abs() <= f64::EPSILON);
            assert!((drho_dp - 4.776954345523257e-7).abs() <= f64::EPSILON);
        } else if cfg!(feature = "invalidasnan") {
            let (drho_dsa, drho_dct, drho_dp) = ans.unwrap();
            assert!(drho_dsa.is_nan());
            assert!(drho_dct.is_nan());
            assert!(drho_dp.is_nan());
        } else {
            match ans {
                Err(Error::NegativeSalinity) => (),
                _ => panic!(),
            }
        }
    }
}

/// Potential density anomaly with reference to sea pressure of 0 dbar
/// (75-term polynomial approximation)
///
/// # Arguments
///
/// * `sa`: Absolute Salinity \[g kg-1\]
/// * `ct`: Conservative Temperature (ITS-90) \[deg C\]
/// * `p`: sea pressure \[dbar\] (i.e. absolute pressure - 10.1325 dbar)
pub fn sigma0(sa: f64, ct: f64) -> Result<f64> {
    let xs: f64 = non_dimensional_sa(sa)?;
    let ys: f64 = ct / GSW_CTU;

    // Specific Volume
    let v = V000
        + xs * (V100 + xs * (V200 + xs * (V300 + xs * (V400 + xs * (V500 + xs * V600)))))
        + ys * (V010
            + xs * (V110 + xs * (V210 + xs * (V310 + xs * (V410 + xs * V510))))
            + ys * (V020
                + xs * (V120 + xs * (V220 + xs * (V320 + xs * V420)))
                + ys * (V030
                    + xs * (V130 + xs * (V230 + xs * V330))
                    + ys * (V040
                        + xs * (V140 + xs * V240)
                        + ys * (V050 + xs * V150 + ys * V060)))));

    Ok(1.0 / v - 1000.0)
}

/// Potential density anomaly with reference to sea pressure of 1000 dbar
/// (75-term polynomial approximation)
///
/// # Arguments
///
/// * `sa`: Absolute Salinity \[g kg-1\]
/// * `ct`: Conservative Temperature (ITS-90) \[deg C\]
/// * `p`: sea pressure \[dbar\] (i.e. absolute pressure - 10.1325 dbar)
pub fn sigma1(sa: f64, ct: f64) -> Result<f64> {
    Ok(rho(sa, ct, 1000.0)? - 1000.0)
}

/// Potential density anomaly with reference to sea pressure of 2000 dbar
/// (75-term polynomial approximation)
///
/// # Arguments
///
/// * `sa`: Absolute Salinity \[g kg-1\]
/// * `ct`: Conservative Temperature (ITS-90) \[deg C\]
/// * `p`: sea pressure \[dbar\] (i.e. absolute pressure - 10.1325 dbar)
pub fn sigma2(sa: f64, ct: f64) -> Result<f64> {
    Ok(rho(sa, ct, 2000.0)? - 1000.0)
}

/// Potential density anomaly with reference to sea pressure of 3000 dbar
/// (75-term polynomial approximation)
///
/// # Arguments
///
/// * `sa`: Absolute Salinity \[g kg-1\]
/// * `ct`: Conservative Temperature (ITS-90) \[deg C\]
/// * `p`: sea pressure \[dbar\] (i.e. absolute pressure - 10.1325 dbar)
pub fn sigma3(sa: f64, ct: f64) -> Result<f64> {
    Ok(rho(sa, ct, 3000.0)? - 1000.0)
}

/// Potential density anomaly with reference to sea pressure of 4000 dbar
/// (75-term polynomial approximation)
///
/// # Arguments
///
/// * `sa`: Absolute Salinity \[g kg-1\]
/// * `ct`: Conservative Temperature (ITS-90) \[deg C\]
/// * `p`: sea pressure \[dbar\] (i.e. absolute pressure - 10.1325 dbar)
pub fn sigma4(sa: f64, ct: f64) -> Result<f64> {
    Ok(rho(sa, ct, 4000.0)? - 1000.0)
}

/// Cabbeling coefficient (75-term polynomial approximation)
///
/// # Arguments
///
/// * `sa`: Absolute Salinity \[ g kg-1 \]
/// * `ct`: Conservative Temperature (ITS-90) \[ deg C \]
/// * `p`: sea pressure \[ dbar \] (i.e. absolute pressure - 10.1325 dbar)
/// # Example:
/// ```
/// use gsw::volume::cabbeling;
/// let c_b = cabbeling(33.0, 10.0, 100.0).unwrap();
/// assert!((c_b - 1.111181295081108e-5).abs() <= f64::EPSILON);
/// ```
pub fn cabbeling(sa: f64, ct: f64, p: f64) -> Result<f64> {
    let (v_sa, v_ct, _) = specvol_first_derivatives(sa, ct, p)?;
    let (v_sa_sa, v_sa_ct, v_ct_ct, _, _) = specvol_second_derivatives(sa, ct, p)?;

    let rho = rho(sa, ct, p)?;

    let alpha_ct = rho * (v_ct_ct - rho * v_ct * v_ct);
    let alpha_sa = rho * (v_sa_ct - rho * v_sa * v_ct);
    let beta_sa = -rho * (v_sa_sa - rho * v_sa * v_sa);

    let alpha_on_beta = alpha_on_beta(sa, ct, p)?;

    Ok(alpha_ct + alpha_on_beta * (2.0 * alpha_sa - alpha_on_beta * beta_sa))
}

#[cfg(test)]
mod test_cabbeling {
    use super::{cabbeling, Error};

    #[test]
    // NaN input results in NaN output.
    // Other libraries using GSW-rs might rely on this behavior to propagate
    // and handle invalid elements.
    fn nan() {
        let c_b = cabbeling(f64::NAN, 1.0, 1.0).unwrap();
        assert!(c_b.is_nan());

        let c_b = cabbeling(1.0, f64::NAN, 1.0).unwrap();
        assert!(c_b.is_nan());

        let c_b = cabbeling(1.0, 1.0, f64::NAN).unwrap();
        assert!(c_b.is_nan());
    }

    #[test]
    fn negative_sa() {
        let c_b = cabbeling(-0.1, 10.0, 100.0);

        if cfg!(feature = "compat") {
            assert!((c_b.unwrap() - 1.283699411753888e-5).abs() <= f64::EPSILON);
        } else if cfg!(feature = "invalidasnan") {
            assert!(c_b.unwrap().is_nan());
        } else {
            match c_b {
                Err(Error::NegativeSalinity) => (),
                _ => panic!("It should be Error::NegativeSalinity"),
            }
        }
    }
}

/// Thermobaric coefficient (75-term polynomial approximation)
///
/// # Arguments
///
/// * `sa`: Absolute Salinity \[ g kg-1 \]
/// * `ct`: Conservative Temperature (ITS-90) \[ deg C \]
/// * `p`: sea pressure \[ dbar \] (i.e. absolute pressure - 10.1325 dbar)
///
/// # Example:
/// ```
/// use gsw::volume::thermobaric;
/// let t_b = thermobaric(33.0, 10.0, 100.0).unwrap();
/// assert!((t_b - 2.3569403670861866e-12).abs() <= f64::EPSILON);
/// ```
pub fn thermobaric(sa: f64, ct: f64, p: f64) -> Result<f64> {
    let (v_sa, v_ct, _) = specvol_first_derivatives(sa, ct, p)?;
    let (_, _, _, v_sa_p, v_ct_p) = specvol_second_derivatives(sa, ct, p)?;

    Ok(rho(sa, ct, p)? * (v_ct_p - (v_ct / v_sa) * v_sa_p))
}

#[cfg(test)]
mod test_thermobaric {
    use super::{thermobaric, Error};

    #[test]
    // NaN input results in NaN output.
    // Other libraries using GSW-rs might rely on this behavior to propagate
    // and handle invalid elements.
    fn nan() {
        let t_b = thermobaric(f64::NAN, 1.0, 1.0).unwrap();
        assert!(t_b.is_nan());

        let t_b = thermobaric(1.0, f64::NAN, 1.0).unwrap();
        assert!(t_b.is_nan());

        let t_b = thermobaric(1.0, 1.0, f64::NAN).unwrap();
        assert!(t_b.is_nan());
    }

    #[test]
    fn negative_sa() {
        let t_b = thermobaric(-0.1, 10.0, 100.0);

        if cfg!(feature = "compat") {
            assert!((t_b.unwrap() - 1.283699411753888e-5).abs() <= f64::EPSILON);
        } else if cfg!(feature = "invalidasnan") {
            assert!(t_b.unwrap().is_nan());
        } else {
            match t_b {
                Err(Error::NegativeSalinity) => (),
                _ => panic!("It should be Error::NegativeSalinity"),
            }
        }
    }
}

/// Specific enthalpy of seawater (75-term polynomial approximation)
///
/// # Arguments
///
/// * `sa`: Absolute Salinity \[g kg-1\]
/// * `ct`: Conservative Temperature (ITS-90) \[deg C\]
/// * `p`: sea pressure \[dbar\] (i.e. absolute pressure - 10.1325 dbar)
///
///
/// ```
/// use gsw::volume::enthalpy;
/// let h = enthalpy(32.0, 10.0, 100.0).unwrap();
/// assert!((h - 40894.546501415374).abs() <= f64::EPSILON);
/// ```
pub fn enthalpy(sa: f64, ct: f64, p: f64) -> Result<f64> {
    Ok(GSW_CP0 * ct + dynamic_enthalpy(sa, ct, p)?)
}

/// Difference of enthalpy between two pressures
/// (75-term polynomial approximation)
///
/// # Arguments
///
/// * `sa`: Absolute Salinity \[ g kg-1 \]
/// * `ct`: Conservative Temperature (ITS-90) \[ deg C \]
/// * `p_shallow`: Upper sea pressure \[ dbar \] (i.e. absolute pressure - 10.1325 dbar)
/// * `p_deep`: Lower sea pressure \[ dbar \] (i.e. absolute pressure - 10.1325 dbar)
///
/// # Example:
/// ```
/// use gsw::volume::enthalpy_diff;
/// let h_diff = enthalpy_diff(33.0, 10.0, 0.0, 100.0).unwrap();
/// assert!((h_diff - 975.1311809188919).abs() <= f64::EPSILON);
/// ```
pub fn enthalpy_diff(sa: f64, ct: f64, p_shallow: f64, p_deep: f64) -> Result<f64> {
    Ok(enthalpy(sa, ct, p_deep)? - enthalpy(sa, ct, p_shallow)?)

    /*
    let s: f64 = non_dimensional_sa(sa)?;
    let tau: f64 = ct / GSW_CTU;
    let pi_shallow: f64 = non_dimensional_p(p_shallow);
    let pi_deep: f64 = non_dimensional_p(p_deep);

    // Must confirm/check below here:
    let part_1 = H001
        + s * (H101 + s * (H201 + s * (H301 + s * (H401 + s * (H501 + H601 * s)))))
        + tau
            * (H011
                + s * (H111 + s * (H211 + s * (H311 + s * (H411 + H511 * s))))
                + tau
                    * (H021
                        + s * (H121 + s * (H221 + s * (H321 + H421 * s)))
                        + tau
                            * (H031
                                + s * (H131 + s * (H231 + H331 * s))
                                + tau
                                    * (H041
                                        + s * (H141 + H241 * s)
                                        + tau * (H051 + H151 * s + H061 * tau)))));

    let part_2 = H002
        + s * (H102 + s * (H202 + s * (H302 + s * (H402 + H502 * s))))
        + tau
            * (H012
                + s * (H112 + s * (H212 + s * (H312 + H412 * s)))
                + tau
                    * (H022
                        + s * (H122 + s * (H222 + H322 * s))
                        + tau
                            * (H032
                                + s * (H132 + H232 * s)
                                + tau * (H042 + H142 * s + H052 * tau))));

    let part_3 = H003
        + s * (H103 + s * (H203 + s * (H303 + H403 * s)))
        + tau
            * (H013
                + s * (H113 + s * (H213 + H313 * s))
                + tau * (H023 + s * (H123 + H223 * s) + tau * (H033 + H133 * s + H043 * tau)));

    let part_4 = H004 + s * (H104 + H204 * s) + tau * (H014 + H114 * s + H024 * tau);

    let part_5 = H005 + H105 * s + H015 * tau;

    let h_hat_shallow = pi_shallow
        * (part_1
            + pi_shallow
                * (part_2
                    + pi_shallow
                        * (part_3
                            + pi_shallow
                                * (part_4
                                    + pi_shallow
                                        * (part_5 + pi_shallow * (H006 + H007 * pi_shallow))))));

    let h_hat_deep = pi_deep
        * (part_1
            + pi_deep
                * (part_2
                    + pi_deep
                        * (part_3
                            + pi_deep
                                * (part_4
                                    + pi_deep * (part_5 + pi_deep * (H006 + H007 * pi_deep))))));

    Ok((h_hat_deep - h_hat_shallow) * DB2PA * 1e4)
    */
}

#[cfg(test)]
mod test_enthalpy_diff {
    use super::{enthalpy, enthalpy_diff, Error};

    #[test]
    // NaN input results in NaN output.
    // Other libraries using GSW-rs might rely on this behavior to propagate
    // and handle invalid elements.
    fn nan() {
        let h_diff = enthalpy_diff(f64::NAN, 1.0, 1.0, 1.0);
        assert!(h_diff.unwrap().is_nan());

        let h_diff = enthalpy_diff(1.0, f64::NAN, 1.0, 1.0);
        assert!(h_diff.unwrap().is_nan());

        let h_diff = enthalpy_diff(1.0, 1.0, f64::NAN, 1.0);
        assert!(h_diff.unwrap().is_nan());

        let h_diff = enthalpy_diff(1.0, 1.0, 1.0, f64::NAN);
        assert!(h_diff.unwrap().is_nan());
    }

    #[test]
    fn negative_sa() {
        let h_diff = enthalpy_diff(-0.1, 10.0, 0.0, 100.0);

        if cfg!(feature = "compat") {
            assert!((h_diff.unwrap() - 1000.0132803364188).abs() <= f64::EPSILON);
        } else if cfg!(feature = "invalidasnan") {
            assert!(h_diff.unwrap().is_nan());
        } else {
            match h_diff {
                Err(Error::NegativeSalinity) => (),
                _ => panic!("It should be Error::NegativeSalinity"),
            }
        }
    }

    #[test]
    fn compare_with_explicit_differente() {
        let h_diff = enthalpy(32.0, 10.0, 100.0).unwrap() - enthalpy(32.0, 10.0, 0.0).unwrap();
        assert_eq!(h_diff, enthalpy_diff(32.0, 10.0, 0.0, 100.0).unwrap());
    }
}

/// Sound speed in seawater (75-term polynomial approximation)
///
/// # Arguments
///
/// * `sa`: Absolute Salinity \[g kg-1\]
/// * `ct`: Conservative Temperature (ITS-90) \[deg C\]
/// * `p`: sea pressure \[dbar\] (i.e. absolute pressure - 10.1325 dbar)
///
/// # Returns
///
/// * `c`: Sound speed \[m s-1\]
///
/// # Notes
///
/// * Pressure is implicitly converted to Pa to obtain speed in m s-1;
///
/// # References
///
/// * IOC, SCOR and IAPSO, 2010: The international thermodynamic equation of
///   seawater - 2010: Calculation and use of thermodynamic properties.
///   Intergovernmental Oceanographic Commission, Manuals and Guides No. 56,
///   UNESCO (English), 196 pp.  Available from <http://www.TEOS-10.org>
///   See Appendix K of this TEOS-10 Manual.
///
/// * McDougall, T.J., D.R. Jackett, D.G. Wright and R. Feistel, 2003:
///   Accurate and computationally efficient algorithms for potential
///   temperature and density of seawater.  J. Atmosph. Ocean. Tech., 20,
///   pp. 730-741.
///
/// * Roquet, F., G. Madec, T.J. McDougall, P.M. Barker, 2015: Accurate
///   polynomial expressions for the density and specifc volume of seawater
///   using the TEOS-10 standard. Ocean Modelling., 90, pp. 29-43.
///
pub fn sound_speed(sa: f64, ct: f64, p: f64) -> Result<f64> {
    let s: f64 = non_dimensional_sa(sa)?;
    let tau: f64 = ct / GSW_CTU;
    //
    let pi: f64 = non_dimensional_p(p);

    // Specific Volume
    let v = V000
        + s * (V100 + s * (V200 + s * (V300 + s * (V400 + s * (V500 + s * V600)))))
        + tau
            * (V010
                + s * (V110 + s * (V210 + s * (V310 + s * (V410 + s * V510))))
                + tau
                    * (V020
                        + s * (V120 + s * (V220 + s * (V320 + s * V420)))
                        + tau
                            * (V030
                                + s * (V130 + s * (V230 + s * V330))
                                + tau
                                    * (V040
                                        + s * (V140 + s * V240)
                                        + tau * (V050 + s * V150 + tau * V060)))))
        + pi * (V001
            + s * (V101 + s * (V201 + s * (V301 + s * (V401 + s * V501))))
            + tau
                * (V011
                    + s * (V111 + s * (V211 + s * (V311 + s * V411)))
                    + tau
                        * (V021
                            + s * (V121 + s * (V221 + s * V321))
                            + tau
                                * (V031
                                    + s * (V131 + s * V231)
                                    + tau * (V041 + s * V141 + tau * V051))))
            + pi * (V002
                + s * (V102 + s * (V202 + s * (V302 + s * V402)))
                + tau
                    * (V012
                        + s * (V112 + s * (V212 + s * V312))
                        + tau
                            * (V022
                                + s * (V122 + s * V222)
                                + tau * (V032 + s * V132 + tau * V042)))
                + pi * (V003
                    + s * (V103 + s * V203)
                    + tau * (V013 + s * V113 + tau * V023)
                    + pi * (V004 + s * V104 + tau * V014 + pi * (V005 + pi * V006)))));

    let v_p = C000
        + s * (C100 + s * (C200 + s * (C300 + s * (C400 + C500 * s))))
        + tau
            * (C010
                + s * (C110 + s * (C210 + s * (C310 + C410 * s)))
                + tau
                    * (C020
                        + s * (C120 + s * (C220 + C320 * s))
                        + tau
                            * (C030
                                + s * (C130 + C230 * s)
                                + tau * (C040 + C140 * s + C050 * tau))))
        + pi * (C001
            + s * (C101 + s * (C201 + s * (C301 + C401 * s)))
            + tau
                * (C011
                    + s * (C111 + s * (C211 + C311 * s))
                    + tau * (C021 + s * (C121 + C221 * s) + tau * (C031 + C131 * s + C041 * tau)))
            + pi * (C002
                + s * (C102 + C202 * s)
                + tau * (C012 + C112 * s + C022 * tau)
                + pi * (C003 + C103 * s + C013 * tau + pi * (C004 + C005 * pi))));

    Ok(10_000.0 * libm::sqrt(-v * v / v_p))
}

pub fn kappa(sa: f64, ct: f64, p: f64) -> Result<f64> {
    let s: f64 = non_dimensional_sa(sa)?;
    let tau: f64 = ct / GSW_CTU;
    let pi: f64 = non_dimensional_p(p);

    let v = V000
        + s * (V010 + s * (V020 + s * (V030 + s * (V040 + s * (V050 + V060 * s)))))
        + tau
            * (V100
                + s * (V110 + s * (V120 + s * (V130 + s * (V140 + V150 * s))))
                + tau
                    * (V200
                        + s * (V210 + s * (V220 + s * (V230 + V240 * s)))
                        + tau
                            * (V300
                                + s * (V310 + s * (V320 + V330 * s))
                                + tau
                                    * (V400
                                        + s * (V410 + V420 * s)
                                        + tau * (V500 + V510 * s + V600 * tau)))))
        + pi * (V001
            + s * (V011 + s * (V021 + s * (V031 + s * (V041 + V051 * s))))
            + tau
                * (V101
                    + s * (V111 + s * (V121 + s * (V131 + V141 * s)))
                    + tau
                        * (V201
                            + s * (V211 + s * (V221 + V231 * s))
                            + tau
                                * (V301
                                    + s * (V311 + V321 * s)
                                    + tau * (V401 + V411 * s + V501 * tau))))
            + pi * (V002
                + s * (V012 + s * (V022 + s * (V032 + V042 * s)))
                + tau
                    * (V102
                        + s * (V112 + s * (V122 + V132 * s))
                        + tau
                            * (V202
                                + s * (V212 + V222 * s)
                                + tau * (V302 + V312 * s + V402 * tau)))
                + pi * (V003
                    + s * (V013 + V023 * s)
                    + tau * (V103 + V113 * s + V203 * tau)
                    + pi * (V004 + V014 * s + V104 * tau + pi * (V005 + V006 * pi)))));

    let v_p = C000
        + s * (C100 + s * (C200 + s * (C300 + s * (C400 + C500 * s))))
        + tau
            * (C010
                + s * (C110 + s * (C210 + s * (C310 + C410 * s)))
                + tau
                    * (C020
                        + s * (C120 + s * (C220 + C320 * s))
                        + tau
                            * (C030
                                + s * (C130 + C230 * s)
                                + tau * (C040 + C140 * s + C050 * tau))))
        + pi * (C001
            + s * (C101 + s * (C201 + s * (C301 + C401 * s)))
            + tau
                * (C011
                    + s * (C111 + s * (C211 + C311 * s))
                    + tau * (C021 + s * (C121 + C221 * s) + tau * (C031 + C131 * s + C041 * tau)))
            + pi * (C002
                + s * (C102 + C202 * s)
                + tau * (C012 + C112 * s + C022 * tau)
                + pi * (C003 + C103 * s + C013 * tau + pi * (C004 + C005 * pi))));

    Ok(-1e-8 * v_p / v)
}

#[cfg(test)]
mod test_kappa {
    use super::{kappa, Error};

    #[test]
    // NaN input results in NaN output.
    // Other libraries using GSW-rs might rely on this behavior to propagate
    // and handle invalid elements.
    fn nan() {
        let k = kappa(f64::NAN, 1.0, 1.0).unwrap();
        assert!(k.is_nan());

        let k = kappa(1.0, f64::NAN, 1.0).unwrap();
        assert!(k.is_nan());

        let k = kappa(1.0, 1.0, f64::NAN).unwrap();
        assert!(k.is_nan());
    }

    #[test]
    fn negative_sa() {
        let k = kappa(-0.1, 10.0, 100.0);

        if cfg!(feature = "compat") {
            assert!((k.unwrap() - 4.631281402194529e-10).abs() <= f64::EPSILON);
        } else if cfg!(feature = "invalidasnan") {
            assert!(k.unwrap().is_nan());
        } else {
            match k {
                Err(Error::NegativeSalinity) => (),
                _ => panic!("It should be Error::NegativeSalinity"),
            }
        }
    }
}

/// Specific interal energy of seawater (75-term polynomial approximation)
///
/// # Arguments
///
/// * `sa`: Absolute Salinity \[ g kg-1 \]
/// * `ct`: Conservative Temperature (ITS-90) \[ deg C \]
/// * `p`: sea pressure \[ dbar \] (i.e. absolute pressure - 10.1325 dbar)
///
/// # Example:
/// ```
/// use gsw::volume::internal_energy;
/// let U = internal_energy(32.0, 10.0, 100.0).unwrap();
/// assert!((U - 39820.03700517441).abs() <= f64::EPSILON);
/// ```
pub fn internal_energy(sa: f64, ct: f64, p: f64) -> Result<f64> {
    Ok(enthalpy(sa, ct, p)? - (GSW_P0 + DB2PA * p) * specvol(sa, ct, p)?)
}

#[cfg(test)]
mod test_internal_energy {
    use super::{internal_energy, Error};

    #[test]
    // NaN input results in NaN output.
    // Other libraries using GSW-rs might rely on this behavior to propagate
    // and handle invalid elements.
    fn nan() {
        let u = internal_energy(f64::NAN, 1.0, 1.0).unwrap();
        assert!(u.is_nan());

        let u = internal_energy(1.0, f64::NAN, 1.0).unwrap();
        assert!(u.is_nan());

        let u = internal_energy(1.0, 1.0, f64::NAN).unwrap();
        assert!(u.is_nan());
    }

    #[test]
    fn negative_sa() {
        let u = internal_energy(-0.1, 10.0, 100.0);

        if cfg!(feature = "compat") {
            assert!((u.unwrap() - 39817.61643796252).abs() <= f64::EPSILON);
        } else if cfg!(feature = "invalidasnan") {
            assert!(u.unwrap().is_nan());
        } else {
            match u {
                Err(Error::NegativeSalinity) => (),
                _ => panic!("It should be Error::NegativeSalinity"),
            }
        }
    }
}

/// Dynamic enthalpy of seawater (75-term polynomial approximation)
///
/// # Arguments
///
/// * `sa`: Absolute Salinity \[g kg-1\]
/// * `ct`: Conservative Temperature (ITS-90) \[deg C\]
/// * `p`: sea pressure \[dbar\] (i.e. absolute pressure - 10.1325 dbar)
///
/// # Returns
///
/// * Dynamic enthalpy \[ J kg-1 \]
///
/// # References
///
/// * Young, W. R. (2010). Dynamic Enthalpy, Conservative Temperature, and the
///   Seawater Boussinesq Approximation, Journal of Physical Oceanography,
///   40(2), 394-400. doi: 10.1175/2009JPO4294.1
///
/// # Example:
/// ```
/// use gsw::volume::dynamic_enthalpy;
/// let h_hat = dynamic_enthalpy(32.0, 10.0, 100.0).unwrap();
/// assert!((h_hat - 975.8669302190772).abs() <= f64::EPSILON);
/// ```
pub fn dynamic_enthalpy(sa: f64, ct: f64, p: f64) -> Result<f64> {
    let s: f64 = non_dimensional_sa(sa)?;
    let tau: f64 = ct / GSW_CTU;
    let pi: f64 = non_dimensional_p(p);

    let dynamic_enthalpy_part = pi
        * (H001
            + s * (H101 + s * (H201 + s * (H301 + s * (H401 + s * (H501 + H601 * s)))))
            + tau
                * (H011
                    + s * (H111 + s * (H211 + s * (H311 + s * (H411 + H511 * s))))
                    + tau
                        * (H021
                            + s * (H121 + s * (H221 + s * (H321 + H421 * s)))
                            + tau
                                * (H031
                                    + s * (H131 + s * (H231 + H331 * s))
                                    + tau
                                        * (H041
                                            + s * (H141 + H241 * s)
                                            + tau * (H051 + H151 * s + H061 * tau)))))
            + pi * (H002
                + s * (H102 + s * (H202 + s * (H302 + s * (H402 + H502 * s))))
                + tau
                    * (H012
                        + s * (H112 + s * (H212 + s * (H312 + H412 * s)))
                        + tau
                            * (H022
                                + s * (H122 + s * (H222 + H322 * s))
                                + tau
                                    * (H032
                                        + s * (H132 + H232 * s)
                                        + tau * (H042 + H142 * s + H052 * tau))))
                + pi * (H003
                    + s * (H103 + s * (H203 + s * (H303 + H403 * s)))
                    + tau
                        * (H013
                            + s * (H113 + s * (H213 + H313 * s))
                            + tau
                                * (H023
                                    + s * (H123 + H223 * s)
                                    + tau * (H033 + H133 * s + H043 * tau)))
                    + pi * (H004
                        + s * (H104 + H204 * s)
                        + tau * (H014 + H114 * s + H024 * tau)
                        + pi * (H005 + H105 * s + H015 * tau + pi * (H006 + H007 * pi))))));

    Ok(dynamic_enthalpy_part * DB2PA * 1e4)
}

#[cfg(test)]
mod test_dynamic_enthalpy {
    use super::{dynamic_enthalpy, Error};

    #[test]
    // NaN input results in NaN output.
    // Other libraries using GSW-rs might rely on this behavior to propagate
    // and handle invalid elements.
    fn nan() {
        let h_hat = dynamic_enthalpy(f64::NAN, 1.0, 1.0);
        assert!(h_hat.unwrap().is_nan());

        let h_hat = dynamic_enthalpy(1.0, f64::NAN, 1.0);
        assert!(h_hat.unwrap().is_nan());

        let h_hat = dynamic_enthalpy(1.0, 1.0, f64::NAN);
        assert!(h_hat.unwrap().is_nan());
    }

    #[test]
    fn negative_sa() {
        let h_hat = dynamic_enthalpy(-0.1, 10.0, 100.0);

        if cfg!(feature = "compat") {
            assert!((h_hat.unwrap() - 1000.0132803364188).abs() <= f64::EPSILON);
        } else if cfg!(feature = "invalidasnan") {
            assert!(h_hat.unwrap().is_nan());
        } else {
            match h_hat {
                Err(Error::NegativeSalinity) => (),
                _ => panic!("It should be Error::NegativeSalinity"),
            }
        }
    }
}

/// Absolute salinity of seawater from given density, Conservative
/// Temperature, and pressure.
///
/// # Notes:
///
/// - According to the Matlab GSW toolbox, after two iterations of this
/// modified Newton-Raphson iteration, the error in SA is no larger than
/// 8e-13 g/kg, which is machine precision for this calculation.
pub fn sa_from_rho(rho: f64, ct: f64, p: f64) -> Result<f64> {
    let v_lab = 1.0 / rho;
    let v_0 = specvol(0.0, ct, p)?;
    let v_50 = specvol(50.0, ct, p)?;

    // First guess, a linear ratio
    let sa = 50.0 * (v_lab - v_0) / (v_50 - v_0);

    if (sa < 0.0) | (sa > 50.0) {
        if cfg!(feature = "invalidasnan") {
            return Ok(f64::NAN);
        } else {
            return Err(Error::Undefined);
        }
    }

    // First guess of dv/dSA
    let v_sa: f64 = (v_50 - v_0) / 50.0;

    // Modified Newton-Raphson iterative optimization
    for _ in 0..2 {
        let sa_old = sa;
        let delta_v = specvol(sa_old, ct, p)? - v_lab;
        // this is half way through the modified N-R method (McDougall and
        // Wotherspoon, 2012, appud Matlab GSW implementation)
        let sa = sa_old - delta_v / v_sa;
        let sa_mean = 0.5 * (sa + sa_old);
        let (v_sa, _, _) = specvol_first_derivatives(sa_mean, ct, p)?;
        let sa = sa_old - delta_v / v_sa;
        if (sa < 0.0) | (sa > 50.0) {
            if cfg!(feature = "invalidasnan") {
                return Ok(f64::NAN);
            } else {
                return Err(Error::Undefined);
            }
        }
    }
    Ok(sa)
}

#[cfg(test)]
mod test_sa_from_rho {
    use super::sa_from_rho;

    #[test]
    // NaN input results in NaN output.
    // Other libraries using GSW-rs might rely on this behavior to propagate
    // and handle invalid elements.
    fn nan() {
        let v = sa_from_rho(f64::NAN, 1.0, 1.0);
        assert!(v.unwrap().is_nan());

        let v = sa_from_rho(1.0, f64::NAN, 1.0);
        assert!(v.unwrap().is_nan());

        let v = sa_from_rho(1.0, 1.0, f64::NAN);
        assert!(v.unwrap().is_nan());
    }
}

/// Conservative Temperature of maximum density of seawater
/// (75-term polynomial approximation)
///
/// # Example
/// ```
/// use gsw::volume::ct_maxdensity;
/// let ct = ct_maxdensity(32.0, 100.0).unwrap();
/// assert!((ct - (-3.337428439202098)).abs() <= f64::EPSILON);
/// ```
///
/// # Notes
/// * After three iterations of this modified Newton-Raphson (McDougall and
///   Wotherspoon, 2012) iteration, the error in CT_maxdensity is typically
///   no larger than 1x10^-15 degress C.
pub fn ct_maxdensity(sa: f64, p: f64) -> Result<f64> {
    let number_of_iterations = 3;
    // Conservative Temperature delta
    let dct = 0.001;
    // Initial guess of Conservative Temperature
    let mut ct = 3.978 - 0.22072 * sa;
    // Initial guess for d alpha / d CT
    let dalpha_dct = 1.1e-5;

    for _ in 0..number_of_iterations {
        let ct_old = ct;
        let a = alpha(sa, ct_old, p)?;
        ct = ct_old - a / dalpha_dct;
        let ct_mean = 0.5 * (ct + ct_old);
        let dalpha_dct =
            (alpha(sa, ct_mean + dct, p)? - alpha(sa, ct_mean - dct, p)?) / (2.0 * dct);
        ct = ct_old - a / dalpha_dct;
    }

    Ok(ct)
}

#[cfg(test)]
mod tests {
    use super::{alpha, beta, specvol};

    #[test]
    fn test_negative_sa() {
        let p_to_test: [f64; 5] = [0., 10., 100., 1000., 5000.];
        let ct_to_test: [f64; 5] = [0., 10., 20., 30., 40.];
        for p in p_to_test.iter() {
            for ct in ct_to_test.iter() {
                if cfg!(feature = "compat") {
                    // If feature compatible is activated, negative sa will be replaced by 0.0
                    assert!(
                        (specvol(-20.0, *ct, *p).unwrap() - specvol(0.0, *ct, *p).unwrap()).abs()
                            < f64::EPSILON
                    );
                    assert!(
                        (alpha(-20.0, *ct, *p).unwrap() - alpha(0.0, *ct, *p).unwrap()).abs()
                            < f64::EPSILON
                    );
                    assert!(
                        (beta(-20.0, *ct, *p).unwrap() - beta(0.0, *ct, *p).unwrap()).abs()
                            < f64::EPSILON
                    );
                } else {
                    // It should return an error if not compat

                    assert!(matches!(
                        alpha(-20.0, *ct, *p),
                        Err(crate::Error::NegativeSalinity)
                    ));
                }
            }
        }
    }
}
/*
fn specvol_first_derivatives_wrt_enthalpy(sa: f64, ct: f64, p: f64) -> (f64, f64) {
    unimplemented!()
}

fn specvol_ice(t: f64, p: f64) {
    unimplemented!()
}

fn specvol_second_derivatives(sa: f64, ct: f64, p: f64) -> (f64, f64, f64, f64, f64) {
    unimplemented!()
}

fn specvol_second_derivatives_wrt_enthalpy(sa: f64, ct: f64, p: f64) -> (f64, f64, f64) {
    unimplemented!()
}

fn specvol_diff() {
    unimplemented!()
}
fn specvol_from_pot_enthalpy_ice() {
    unimplemented!()
}
fn specvol_from_pot_enthalpy_ice_poly() {
    unimplemented!()
}
fn specvol_p_parts() {
    unimplemented!()
}
*/
