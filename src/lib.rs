//! # Gibbs Sea Water
//!
//! Gibbs Sea Water Oceanographic Toolbox of TEOS-10 implemented in Rust.
//! version: 3.06.12
//!
//! http://www.teos-10.org
//!

////////////////////////////////////////////////////////////////////////////////

// Do not depend on the standard library
#![no_std]

/// cbindgen:ignore
mod gsw_internal_const;

/// cbindgen:ignore
mod gsw_specvol_coefficients;

#[cfg(feature = "capi")]
mod ffi;

use gsw_internal_const::*;
use gsw_specvol_coefficients::*;

/// Calculates specific volume of sea water
///
/// Calculates specific volume from Absolute Salinity, Conservative
/// Temperature and pressure, using the computationally-efficient
/// polynomial expression for specific volume (Roquet et al., 2014).
///
/// sa [g/kg] : Absolute Salinity
/// ct [deg C] : Conservative Temperature (ITS-90)
/// p [dbar] : sea pressure ( i.e. absolute pressure - 10.1325 dbar )
///
/// specvol [m^3/kg] : specific volume
///
/// Note that the coefficients v(i,j,k) follow the convention in the original
/// paper, which is different from the convention used in the C-library.
///
pub fn specvol(sa: f64, ct: f64, p: f64) -> f64 {
    // Other implementations force negative SA to be 0. That is dangerous
    // since it can hide error by processing unrealistic inputs
    let sa: f64 = if (sa > 0.0) {
        sa
    } else if cfg!(feature = "compat") {
        0.0
    } else {
        panic!("Negative SA");
    };

    let xs: f64 = libm::sqrt(GSW_SFAC * sa + OFFSET);
    let ys: f64 = ct / GSW_CTU;
    let z: f64 = p / GSW_PU;

    // Specific Volume
    V000 + xs * (V100 + xs * (V200 + xs * (V300 + xs * (V400 + xs * (V500 + xs * V600)))))
        + ys * (V010
            + xs * (V110 + xs * (V210 + xs * (V310 + xs * (V410 + xs * V510))))
            + ys * (V020
                + xs * (V120 + xs * (V220 + xs * (V320 + xs * V420)))
                + ys * (V030
                    + xs * (V130 + xs * (V230 + xs * V330))
                    + ys * (V040 + xs * (V140 + xs * V240) + ys * (V050 + xs * V150 + ys * V060)))))
        + z * (V001
            + xs * (V101 + xs * (V201 + xs * (V301 + xs * (V401 + xs * V501))))
            + ys * (V011
                + xs * (V111 + xs * (V211 + xs * (V311 + xs * V411)))
                + ys * (V021
                    + xs * (V121 + xs * (V221 + xs * V321))
                    + ys * (V031 + xs * (V131 + xs * V231) + ys * (V041 + xs * V141 + ys * V051))))
            + z * (V002
                + xs * (V102 + xs * (V202 + xs * (V302 + xs * V402)))
                + ys * (V012
                    + xs * (V112 + xs * (V212 + xs * V312))
                    + ys * (V022 + xs * (V122 + xs * V222) + ys * (V032 + xs * V132 + ys * V042)))
                + z * (V003
                    + xs * (V103 + xs * V203)
                    + ys * (V013 + xs * V113 + ys * V023)
                    + z * (V004 + xs * V104 + ys * V014 + z * (V005 + z * V006)))))
}

#[cfg(test)]
mod tests {
    use super::gsw_internal_const::*;
    use super::{
        alpha, beta, specvol, specvol_alpha_beta, specvol_anom_standard, specvol_sso_0, GSW_SFAC,
    };

    #[test]
    // Calculated SFAC is slightly different than the prescribed SFAC in other
    // packages. Prescribed ends in 4615 while the here calculated ends in
    // 461472. Which is the correct one?
    fn test_const_sfac() {
        if cfg!(feature = "compat") {
            assert_eq!(GSW_SFAC, 0.0248826675584615);
        } else {
            assert_eq!(GSW_SFAC, 0.024882667558461472);
        }
    }

    /// specvol() at SSO & CT=0 should be identical to specvol_sso_0()
    #[test]
    fn test_specvol_vs_specvol_sso_0() {
        let p_to_test: [f64; 5] = [0., 10., 100., 1000., 5000.];
        for p in p_to_test.iter().cloned() {
            let specvol = specvol(GSW_SSO, 0., p);
            let specvol_sso_0 = specvol_sso_0(p);
            assert_eq!(specvol, specvol_sso_0);
        }
    }

    #[test]
    fn test_specvol() {
        // Test value from Roquet 2015
        //let specvol = specvol(30., 10., 1000.0);
        //#[cfg(not(feature = "compat"))]
        //assert_eq!(specvol, 9.732819627722664e-4);

        // Test value from C library.
        let specvol = specvol(34.507499465692057, 27.994827331978655, 0.0);
        #[cfg(feature = "compat")]
        assert_eq!(specvol, 0.00097855432330275953);
    }

    #[test]
    #[cfg(feature = "compat")]
    // If not compat, there is a residue of 1e-19
    fn test_specvol_anom_standard_at_standard() {
        assert_eq!(specvol_anom_standard(35.16504, 0.0, 1000.0), 0.0);
    }

    #[test]
    fn test_specvol_alpha_beta() {
        #[cfg(feature = "compat")]
        // Values from C implementation
        assert_eq!(
            specvol_alpha_beta(34.537484086977358, 27.793319825682374, 50.),
            (
                0.00097826888242888476,
                0.00031741177706767163,
                0.00071877529859646001
            )
        );
    }

    #[test]
    #[cfg(feature = "compat")]
    // If feature compatible is activated, negative sa will be replaced by 0.0
    fn test_negative_sa() {
        assert_eq!(specvol(-20.0, 10., 0.), specvol(0.0, 10., 0.));
        assert_eq!(alpha(-20.0, 10., 0.), alpha(0.0, 10., 0.));
        assert_eq!(beta(-20.0, 10., 0.), beta(0.0, 10., 0.));
    }
}

/// Specific Volume of Standard Ocean Salinity and CT=0
///
/// This function calculates specifc volume at the Standard Ocean Salinity,
/// SSO, and at a Conservative Temperature of zero degrees C, as a function
/// of pressure, p, in dbar, using a streamlined version of the 75-term CT
/// version of specific volume, that is, a streamlined version of the code
/// "specvol(SA,CT,p)".
///
/// version: 3.06.12
///
/// If using compat (truncated constants) there is a difference of O[1e-19],
/// which is negligible but enough to fail the validation tests.
pub fn specvol_sso_0(p: f64) -> f64 {
    const VXX0: f64 = if cfg!(feature = "compat") {
        9.726_613_854_843_87e-4
    } else {
        9.726_613_854_843_871e-4
    };

    const VXX1: f64 = if cfg!(feature = "compat") {
        -4.505_913_211_160_929e-5
    } else {
        -4.505_913_211_160_931e-5
    };

    const VXX2: f64 = if cfg!(feature = "compat") {
        7.130_728_965_927_127e-6
    } else {
        7.130_728_965_927_128e-6
    };

    const VXX3: f64 = if cfg!(feature = "compat") {
        -6.657_179_479_768_312e-7
    } else {
        -6.657_179_479_768_313e-7
    };
    const VXX4: f64 = if cfg!(feature = "compat") {
        -2.994_054_447_232_88e-8
    } else {
        -2.994_054_447_232_877_6e-8
    };

    let p = p / GSW_PU;

    VXX0 + p * (VXX1 + p * (VXX2 + p * (VXX3 + p * (VXX4 + p * (V005 + V006 * p)))))
}

/// Specific Volume Anomaly of Standard Ocean Salinity and CT=0
///
/// Specific volume anomaly with reference of SA = SSO & CT = 0 (75-term equation)
///
///
/// sa [g/kg] : Absolute Salinity
/// ct [deg C] : Conservative Temperature (ITS-90)
/// p [dbar] : sea pressure ( i.e. absolute pressure - 10.1325 dbar )
///
/// specvol_anom : specific volume anomaly of seawater
///
pub fn specvol_anom_standard(sa: f64, ct: f64, p: f64) -> f64 {
    specvol(sa, ct, p) - specvol_sso_0(p)
}

pub fn enthalpy_sso_0(p: f64) -> f64 {
    const H006: f64 = -2.10787688100e-9;
    const H007: f64 = 2.80192913290e-10;

    let z = p * 1.0e-4;

    let dynamic_enthalpy_sso_0_p = z
        * (9.726_613_854_843_87e-4
            + z * (-2.252_956_605_630_465e-5
                + z * (2.376_909_655_387_404e-6
                    + z * (-1.664_294_869_986_011e-7
                        + z * (-5.988_108_894_465_758e-9 + z * (H006 + H007 * z))))));

    dynamic_enthalpy_sso_0_p * DB2PA * 1.0e4
}

pub fn specvol_alpha_beta(sa: f64, ct: f64, p: f64) -> (f64, f64, f64) {
    // What to do with negative SA? Matlab does "SA(SA < 0) = 0;", but maybe we shouldn't guess and fail with assert.

    let specvol = specvol(sa, ct, p);
    let alpha = alpha(sa, ct, p);
    let beta = beta(sa, ct, p);

    (specvol, alpha, beta)
}

/// in-situ density
///
/// Calculates in-situ density from Absolute Salinity and Conservative
/// Temperature, using the computationally-efficient expression for
/// specific volume in terms of SA, CT and p (Roquet et al., 2014).
///
/// sa [g/kg] : Absolute Salinity
/// ct [deg C] : Conservative Temperature (ITS-90)
/// p [dbar] : sea pressure ( i.e. absolute pressure - 10.1325 dbar )
///
/// rho  [kg/m] : in-situ density
///
pub fn rho(sa: f64, ct: f64, p: f64) -> f64 {
    1.0 / specvol(sa, ct, p)
}

/// Height from pressure
///
/// Calculates the height z from pressure p
///
/// p [dbar] : sea pressure ( i.e. absolute pressure - 10.1325 dbar )
/// lat [deg] : latitude
/// geo_strf_dyn_height [m^2/s^2] : dynamic height anomaly
///
/// Note that the reference pressure, p_ref, of geo_strf_dyn_height must
/// be zero (0) dbar.
/// sea_surface_geopotential [m^2/s^2] : geopotential at zero sea pressure
///
pub fn z_from_p(
    press: f64,
    lat: f64,
    geo_strf_dyn_height: f64,
    sea_surface_geopotential: f64,
) -> f64 {
    let x = libm::sin(lat * DEG2RAD);
    let sin2 = x * x;
    let b = 9.780327 * (1.0 + (5.2792e-3 + (2.32e-5 * sin2)) * sin2);
    let a = -0.5 * GAMMA * b;
    let c = enthalpy_sso_0(press) - (geo_strf_dyn_height + sea_surface_geopotential);

    // Depth z
    -2.0 * c / (b + libm::sqrt(b * b - 4.0 * a * c))
}

fn alpha(sa: f64, ct: f64, p: f64) -> f64 {
    // Other implementations force negative SA to be 0. That is dangerous
    // since it can hide error by processing unrealistic inputs
    let sa: f64 = if (sa > 0.0) {
        sa
    } else if cfg!(feature = "compat") {
        0.0
    } else {
        panic!("Negative SA");
    };

    let xs: f64 = libm::sqrt(GSW_SFAC * sa + OFFSET);
    let ys: f64 = ct / GSW_CTU;
    let z: f64 = p / GSW_PU;

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

    0.025 * v_ct / specvol(sa, ct, p)
}

fn beta(sa: f64, ct: f64, p: f64) -> f64 {
    // Other implementations force negative SA to be 0. That is dangerous
    // since it can hide error by processing unrealistic inputs
    let sa: f64 = if (sa > 0.0) {
        sa
    } else if cfg!(feature = "compat") {
        0.0
    } else {
        panic!("Negative SA");
    };

    let xs: f64 = libm::sqrt(GSW_SFAC * sa + OFFSET);
    let ys: f64 = ct / GSW_CTU;
    let z: f64 = p / GSW_PU;

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

    -v_sa * 0.5 * GSW_SFAC / (specvol(sa, ct, p) * xs)
}

fn specvol_first_derivatives(sa: f64, ct: f64, p: f64) -> (f64, f64, f64) {
    // Other implementations force negative SA to be 0. That is dangerous
    // since it can hide error by processing unrealistic inputs
    let sa: f64 = if (sa > 0.0) {
        sa
    } else if cfg!(feature = "compat") {
        0.0
    } else {
        panic!("Negative SA");
    };

    let xs: f64 = libm::sqrt(GSW_SFAC * sa + OFFSET);
    let ys: f64 = ct / GSW_CTU;
    let z: f64 = p / GSW_PU;

    let v_ct_part: f64 = A000
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

    let v_ct = 0.025 * v_ct_part;

    let v_sa_part: f64 = B000
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

    let v_sa = 0.5 * GSW_SFAC * v_sa_part / xs;

    let v_p_part = C000
        + xs * (C100 + xs * (C200 + xs * (C300 + xs * (C400 + C500 * xs))))
        + ys * (C010
            + xs * (C110 + xs * (C210 + xs * (C310 + C410 * xs)))
            + ys * (C020
                + xs * (C120 + xs * (C220 + C320 * xs))
                + ys * (C030 + xs * (C130 + C230 * xs) + ys * (C040 + C140 * xs + C050 * ys))))
        + z * (C001
            + xs * (C101 + xs * (C201 + xs * (C301 + C401 * xs)))
            + ys * (C011
                + xs * (C111 + xs * (C211 + C311 * xs))
                + ys * (C021 + xs * (C121 + C221 * xs) + ys * (C031 + C131 * xs + C041 * ys)))
            + z * (C002
                + xs * (C102 + C202 * xs)
                + ys * (C012 + C112 * xs + C022 * ys)
                + z * (C003 + C103 * xs + C013 * ys + z * (C004 + C005 * z))));

    let v_p = 1e-8 * v_p_part;

    (v_sa, v_ct, v_p)
}

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

fn specvol_t_exact(sa: f64, t: f64, p: f64) {
    unimplemented!()
}

fn specvol_ct_exact(sa: f64, ct: f64, p: f64) {
    unimplemented!()
}
fn specvol_alpha_beta_ct_exact() {
    unimplemented!()
}
fn specvol_anom(sa: f64, ct: f64, p: f64) {
    unimplemented!()
}
fn specvol_anom_ct_exact() {
    unimplemented!()
}
fn specvol_anom_standard_ct_exact() {
    unimplemented!()
}
fn specvol_diff() {
    unimplemented!()
}
fn specvol_diff_ct_exact() {
    unimplemented!()
}
fn specvol_first_derivatives_ct_exact() {
    unimplemented!()
}
fn specvol_first_derivatives_wrt_enthalpy_ct_exact() {
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
fn specvol_second_derivatives_ct_exact() {
    unimplemented!()
}
fn specvol_second_derivatives_wrt_enthalpy_ct_exact() {
    unimplemented!()
}
fn specvol_anom_standard_t_exact() {
    unimplemented!()
}
fn specvol_anom_t_exact() {
    unimplemented!()
}

fn t90_from_t48(t48: f64) -> f64 {
    (t48 - (4.4e-6) * t48 * (100. - t48)) / 1.00024
}

fn t90_from_t68(t68: f64) -> f64 {
    #[cfg(feature = "compat")]
    let t90: f64 = t68 * 0.999760057586179;

    #[cfg(not(feature = "compat"))]
    let t90: f64 = t68 / 1.00024;

    t90
}

fn abs_pressure_from_p(p: f64) -> f64 {
    p * DB2PA + GSW_P0
}
