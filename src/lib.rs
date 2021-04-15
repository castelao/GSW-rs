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
