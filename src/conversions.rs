//! Conversions
//!
//! Other conversions between temperatures, salinities, entropy, pressure
//! and height.

use crate::gsw_internal_const::{DB2PA, DEG2RAD, GAMMA, GSW_P0};
use crate::gsw_internal_funcs::enthalpy_sso_0;

/*
gsw_deltaSA_from_SP
gsw_SA_Sstar_from_SP
gsw_SR_from_SP
gsw_SP_from_SR
gsw_SP_from_SA
gsw_Sstar_from_SA
gsw_SA_from_Sstar
gsw_SP_from_Sstar
gsw_pt_from_CT
gsw_t_from_CT
gsw_CT_from_pt
gsw_pot_enthalpy_from_pt
gsw_pt_from_t
gsw_pt0_from_t
gsw_t_from_pt0
*/

/// ITS-90 temperature from IPTS-48 temperature
///
pub fn t90_from_t48(t48: f64) -> f64 {
    (t48 - (4.4e-6) * t48 * (100. - t48)) / 1.00024
}

/// ITS-90 temperature from IPTS-68 temperature
///
/// Converts IPTS-68 temperature to International Temperature Scale 1990
/// (ITS-90) temperature according to Saunders 1990 appud TEOS-10 manual
/// (Appendix A.1.4).
///
/// # Arguments
///
/// * `t68` - Temperature IPTS-68 [deg C]
///
/// # Returns
///
/// * `t90` - Temperature ITS-90 [deg C]
///
/// # Examples
/// ```
/// use gsw::conversions::t90_from_t68;
///
/// let t90 = t90_from_t68(13.42);
/// ```
pub fn t90_from_t68(t68: f64) -> f64 {
    if cfg!(feature = "compat") {
        t68 * 0.999760057586179
    } else {
        t68 / 1.00024
    }
}

/// Height from pressure
///
/// Calculates the height z from pressure p
///
/// # Arguments
/// * `pressure` [dbar] - Sea Pressure, i.e. absolute pressure - 10.1325 dbar.
/// * `lat` [deg] - Latitude.
/// * `geo_strf_dyn_height` [m^2 s^-2] - Dynamic height anomaly.
/// * `sea_surface_geopotential` [m^2 s^-2] - Geopotential at zero sea pressure.
///
/// # Returns
///
/// * `z` [m] - Height, where z points upward and is zero at the sea level,
///   thus it is negative in the ocean.
///
/// # Notes
/// If geo_strf_dyn_height was obtained from geo_strf_dyn_height(), reference
/// pressure (p_ref) must be zero dbar.
///
/// # Examples
/// ```
/// use gsw::conversions::z_from_p;
///
/// let z = z_from_p(100.0, -60.250, 0.0, 0.0);
/// ```
pub fn z_from_p(
    pressure: f64,
    lat: f64,
    geo_strf_dyn_height: f64,
    sea_surface_geopotential: f64,
) -> f64 {
    let x = libm::sin(lat * DEG2RAD);
    let sin2 = x * x;
    let b = 9.780327 * (1.0 + (5.2792e-3 + (2.32e-5 * sin2)) * sin2);
    let a = -0.5 * GAMMA * b;
    let c = enthalpy_sso_0(pressure) - (geo_strf_dyn_height + sea_surface_geopotential);

    // Depth z
    -2.0 * c / (b + libm::sqrt(b * b - 4.0 * a * c))
}

/*
gsw_p_from_z
gsw_z_from_depth
gsw_depth_from_z
*/

/// Absolute Pressure, P, from sea pressure, p
///
pub fn abs_pressure_from_p(p: f64) -> f64 {
    p * DB2PA + GSW_P0
}

/// Pressure from absolute pressure
///
/// # Arguments
/// * absolute_pressure [Pa] - Absolute pressure
///
pub fn p_from_abs_pressure(absolute_pressure: f64) -> f64 {
    (absolute_pressure - GSW_P0) / DB2PA
}

/*
gsw_entropy_from_CT
gsw_CT_from_entropy
gsw_entropy_from_pt
gsw_pt_from_entropy
gsw_entropy_from_t
gsw_t_from_entropy
gsw_adiabatic_lapse_rate_from_CT
gsw_adiabatic_lapse_rate_from_t
gsw_molality_from_SA
gsw_ionic_strength_from_SA
*/

#[cfg(test)]
mod tests {
    use super::{t90_from_t68, z_from_p};

    #[test]
    fn test_t90_from_t68() {
        assert!((0.0 - t90_from_t68(0.0)).abs() < f64::EPSILON);
        if cfg!(feature = "compat") {
            assert!((9.999999999999996 - t90_from_t68(10.0024)).abs() < f64::EPSILON);
        } else {
            assert!((10.0 - t90_from_t68(10.0024)).abs() < f64::EPSILON);
        }
    }

    #[test]
    fn test_z_from_p() {
        assert!((0.0 - z_from_p(0.0, 33.3482, 0.0, 0.0)).abs() < f64::EPSILON);
    }
}
