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
#![allow(unused)]

/// cbindgen:ignore
mod gsw_internal_const;

/// cbindgen:ignore
mod gsw_specvol_coefficients;

#[cfg(feature = "capi")]
mod ffi;

mod volume;

use crate::gsw_internal_const::*;
use crate::gsw_specvol_coefficients::*;
pub use crate::volume::{specvol, specvol_sso_0};

#[cfg(test)]
mod tests {
    use super::gsw_internal_const::*;
    use super::{alpha, beta, specvol, specvol_alpha_beta, specvol_anom_standard, GSW_SFAC};

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
