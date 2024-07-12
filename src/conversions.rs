//! Conversions
//!
//! Other conversions between temperatures, salinities, entropy, pressure
//! and height.

use crate::gsw_internal_const::{DB2PA, DEG2RAD, GAMMA, GSW_CP0, GSW_P0, GSW_SFAC, GSW_UPS};
use crate::gsw_internal_funcs::enthalpy_sso_0;
use crate::{Error, Result};

/*
/// Absolute Salinity Anomaly from Practical Salinity
///
/// # Arguments
///
/// * `sp`:
/// * `p`:
/// * `lon`:
/// * `lat`:
///
/// # Returns
///
/// # References
///
/// # Examples
///
pub fn deltasa_from_sp(sp: f64, p: f64, lon: f64, lat: f64) -> Result<f64> {
    // Remove out of range values
    if ((p < 100.0) & (sp > 120.0)) | ((p >= 100.0) & (sp > 42.0)) {
        if cfg!(feature = "invalidasnan") {
            return Ok(f64::NAN);
        } else {
            return Err(Error::Undefined);
        }
    }

    if (p < -1.5) | (p > 12000.0) | (lon < 0.0) | (lon > 360.0) | (lat < -90.0) | (lat > 90.0) {
        if cfg!(feature = "compat") {
            return Err(Error::Undefined);
        } else if cfg!(feature = "invalidasnan") {
            return Ok(f64::NAN);
        } else {
            return Err(Error::Undefined);
        }
    }

    let sp: f64 = if sp < 0.0 {
        if cfg!(feature = "compat") {
            0.0
        } else if cfg!(feature = "invalidasnan") {
            return Ok(f64::NAN);
        } else {
            return Err(Error::NegativeSalinity);
        }
    } else {
        sp
    };

    let sa = sa_from_sp(sp, p, lon, lat);
    let sr = sr_from_sp(sp);
    let dsa = sa - sr;
    Ok(dsa)
}
*/

/*
gsw_deltaSA_from_SP(gsw_sa_from_sp, gsw_sr_from_sp)
gsw_SA_Sstar_from_SP
*/

/// Reference Salinity ($S_R$) from Practical Salinity ($S_P$)
///
/// # Arguments
///
/// * `sp`: Practical salinity (PSS-78) \[unitless\]
///
/// # Returns
///
/// * `sr`: Reference salinity \[g kg-1\]
///
/// # Features
///
/// If compiled with 'compat', $u_{PS}$ is approximated to
/// 1.004715428571429, which can cause a minimal rounding error.
///
/// # References
///
/// Millero, F. J., R. Feistel, D. G. Wright, and T. J. McDougall, 2008: The
///   composition of Standard Seawater and the definition of the
///   Reference-Composition Salinity Scale, Deep-Sea Res. I, 55, 50-72.
///
/// # Examples
/// ```
/// use gsw::conversions::sr_from_sp;
///
/// let sr = sr_from_sp(32.0);
/// assert!((sr - 32.150893714285715).abs() <= f64::EPSILON);
/// ```
pub fn sr_from_sp(sp: f64) -> f64 {
    if cfg!(feature = "compat") {
        sp * 1.004715428571429
    } else {
        sp * GSW_UPS
    }
}

/// Practical Salinity ($S_P$) from Reference Salinity ($S_R$)
///
/// # Arguments
///
/// * `sr`: Reference salinity \[g kg-1\]
///
/// # Returns
///
/// * `sp`: Practical salinity (PSS-78) \[unitless\]
///
/// # Features
///
/// If compiled with 'compat', $1/u_{PS}$ is approximated to
/// 0.995306702338459, which can cause a minimal rounding error.
///
/// # References
///
/// Millero, F. J., R. Feistel, D. G. Wright, and T. J. McDougall, 2008: The
///   composition of Standard Seawater and the definition of the
///   Reference-Composition Salinity Scale, Deep-Sea Res. I, 55, 50-72.
///
/// # Examples
/// ```
/// use gsw::conversions::sp_from_sr;
///
/// let sp = sp_from_sr(32.0);
/// assert!((sp - 31.849814474830684).abs() <= f64::EPSILON);
/// ```
pub fn sp_from_sr(sr: f64) -> f64 {
    if cfg!(feature = "compat") {
        sr * 0.995306702338459
    } else {
        sr / GSW_UPS
    }
}

/*
gsw_SP_from_SA(gsw_sp_from_sa_baltic)
gsw_Sstar_from_SA
gsw_SA_from_Sstar
gsw_SP_from_Sstar
gsw_pt_from_CT
gsw_t_from_CT
gsw_pt_from_CT(gsw_ct_from_pt, gsw_gibbs_pt0_pt0)
gsw_t_from_CT(gsw_ct_from_pt, gsw_pt_from_t)
*/

/// Conservative Temperature from potential temperature
///
/// # Arguments
///
/// * `sa`: Absolute salinity \[g kg-1\]
/// * `pt`: Potential temperature (ITS-90) \[deg C\]
///
/// # Returns
///
/// * `ct`: Conservative Temperature (ITS-90) \[deg C\]
///
/// # Features
///
/// * default: Negative salinity returns [Error::NegativeSalinity].
/// * compat:
///   * Negative salinity is assumed to be zero.
///   * S factor is approximated to 0.0248826675584615, which can cause a
///     minimal error.
/// * invalidasnan: Negative salinity results in NaN.
///
/// # References
///
/// # Examples
/// ```
/// use gsw::conversions::ct_from_pt;
///
/// let ct = ct_from_pt(32.0, 10.0).unwrap();
/// assert!((ct - 10.047455620469973).abs() <= f64::EPSILON);
/// ```
pub fn ct_from_pt(sa: f64, pt: f64) -> Result<f64> {
    // Doesn't apply the offset so can't use non_dimensional_sa
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

    let x2 = GSW_SFAC * sa;
    let x = libm::sqrt(x2);
    let y = pt * 0.025;
    let pot_enthalpy = 61.01362420681071e0
        + y * (168776.46138048015e0
            + y * (-2735.2785605119625e0
                + y * (2574.2164453821433e0
                    + y * (-1536.6644434977543e0
                        + y * (545.7340497931629e0
                            + (-50.91091728474331e0 - 18.30489878927802e0 * y) * y)))))
        + x2 * (268.5520265845071e0
            + y * (-12019.028203559312e0
                + y * (3734.858026725145e0
                    + y * (-2046.7671145057618e0
                        + y * (465.28655623826234e0
                            + (-0.6370820302376359e0 - 10.650848542359153e0 * y) * y))))
            + x * (937.2099110620707e0
                + y * (588.1802812170108e0
                    + y * (248.39476522971285e0
                        + (-3.871557904936333e0 - 2.6268019854268356e0 * y) * y))
                + x * (-1687.914374187449e0
                    + x * (246.9598888781377e0
                        + x * (123.59576582457964e0 - 48.5891069025409e0 * x))
                    + y * (936.3206544460336e0
                        + y * (-942.7827304544439e0
                            + y * (369.4389437509002e0
                                + (-33.83664947895248e0 - 9.987880382780322e0 * y) * y))))));

    Ok(pot_enthalpy / GSW_CP0)
}

/*
gsw_pot_enthalpy_from_pt
gsw_pt_from_t
gsw_pt0_from_t(gsw_entropy_part, gsw_entropy_part_zerop, gsw_gibbs_pt0_pt0)
gsw_t_from_pt0
*/

/// ITS-90 temperature from IPTS-48 temperature
///
/// # Arguments
///
/// * `t48`: Temperature  (IPTS-48) \[deg C\]
///
/// # Returns
///
/// * `t90`: Temperature ITS-90 \[deg C\]
///
/// # References
///
/// Saunders, P. 1990: The International Temperature Scale of 1990, ITS-90.
///   WOCE Newsletter 10, IOS, Wormley, UK.
///
/// # Examples
/// ```
/// use gsw::conversions::t90_from_t48;
///
/// let t90 = t90_from_t48(12.0);
/// assert!((t90 - 11.992475405902583).abs() <= f64::EPSILON);
/// ```
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
/// * `t68`: Temperature IPTS-68 \[deg C\]
///
/// # Returns
///
/// * `t90`: Temperature ITS-90 \[deg C\]
///
/// # Features
///
/// * default: Calculate precise $t_{68}$ / 1.00024
/// * compat: Approximate 1 / 1.00024 to 0.999760057586179, which can cause
///           a negligible error.
///
/// # Notes
///
/// * TEOS-10 manual recommends to continue using Saunders 1990 instead of
///   Rusby 1991.
///
/// # References
///
/// Saunders, P. 1990: The International Temperature Scale of 1990, ITS-90.
///   WOCE Newsletter 10, IOS, Wormley, UK.
///
/// # Examples
/// ```
/// use gsw::conversions::t90_from_t68;
///
/// let t90 = t90_from_t68(13.42);
/// assert!((t90 - 13.416779972806527).abs() <= f64::EPSILON);
/// ```
pub fn t90_from_t68(t68: f64) -> f64 {
    if cfg!(feature = "compat") {
        t68 * 0.999760057586179
    } else {
        t68 / 1.00024
    }
}

/// IPTS-68 temperature from ITS-90 temperature
///
/// # Arguments
///
/// * `t90`: Temperature ITS-90 \[deg C\]
///
/// # Returns
///
/// * `t68`: Temperature IPTS-68 \[deg C\]
///
/// # References
///
/// Saunders, P. 1990: The International Temperature Scale of 1990, ITS-90.
///   WOCE Newsletter 10, IOS, Wormley, UK.
///
/// # Examples
/// ```
/// use gsw::conversions::t68_from_t90;
///
/// let t = t68_from_t90(13.42);
/// assert!((t - 13.4232208).abs() <= f64::EPSILON);
/// ```
pub fn t68_from_t90(t90: f64) -> f64 {
    t90 * 1.00024
}

/// Height from pressure
///
/// Calculates the height z from pressure p
///
/// # Arguments
/// * `pressure`: Sea Pressure \[dbar\], i.e. absolute pressure - 10.1325 dbar.
/// * `lat`: Latitude \[deg\]
/// * `geo_strf_dyn_height`: Dynamic height anomaly \[m2 s-2\]
/// * `sea_surface_geopotential`: Geopotential at zero sea pressure \[m2 s-2\]
///
/// # Returns
///
/// * `z`: Height \[m\], where z points upward and is zero at the sea level,
///        thus it is negative in the ocean.
///
/// # Features
///
/// * default: $gamma$ = 2.26e-7.
/// * nodgdz: $gamma$ = 0, i.e. gravity doesn't depend on depth.
///
/// # References
///
/// # Notes
///
/// If geo_strf_dyn_height was obtained from geo_strf_dyn_height(), reference
/// pressure (p_ref) must be zero dbar.
///
/// # Examples
/// ```
/// use gsw::conversions::z_from_p;
///
/// let z = z_from_p(6131., 11.0, 0.0, 0.0);
/// assert!((z - (-6010.854959777581)).abs() <= f64::EPSILON);
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

#[cfg(test)]
mod test_z_from_p {
    use super::{p_from_z, z_from_p};

    #[test]
    fn surface() {
        assert!((0.0 - z_from_p(0.0, 33.3482, 0.0, 0.0)).abs() < f64::EPSILON);
        // assert_eq!(-6010.85496, z_from_p(6131.0, 11.0, 0.0, 0.0));
    }

    #[test]
    fn roundtrip() {
        let p1 = 6131.0;
        let lat = 11.0;
        let p2 = p_from_z(z_from_p(p1, lat, 0.0, 0.0), lat, Some(0.0), Some(0.0)).unwrap();
        // Precision defined by (McDougall and Wotherspoon, 2013)
        assert!((p1 - p2).abs() <= 1.6e-10);
    }

    #[test]
    fn roundtrip_with_extras() {
        let p1 = 6131.0;
        let lat = 11.0;
        for dh in [0.0, 1.0] {
            for gp in [0.0, 1.0] {
                let p2 = p_from_z(z_from_p(p1, lat, dh, gp), lat, Some(dh), Some(gp)).unwrap();
                // Precision defined by (McDougall and Wotherspoon, 2013)
                assert!((p1 - p2).abs() <= 1.6e-10);
            }
        }
    }
}

/// Pressure from height (75-term polynomial approximation)
///
/// # Arguments
///
/// * `z`: Height \[m\], where z points upward and is zero at the sea level,
///        thus it is negative in the ocean.
/// * `lat`: Latitude \[deg\]
/// * `geo_strf_dyn_height`: Dynamic height anomaly \[m2 s-2\]. Optional
///   argument, thus can use 'None' to assume it zero.
/// * `sea_surface_geopotential`: Geopotential at zero sea pressure \[m2 s-2\].
///   Optional argument, thus can use 'None' to assume it zero.
///
/// # Returns
///
/// * `pressure`: Sea Pressure \[dbar\], i.e. absolute pressure - 10.1325 dbar.
///
/// # Notes
///
/// * The GSW-C does not allow dynamic height anomaly or geopotential at zero
///   sea pressure.
/// * Optional arguments might change in the future to require instead an
///   explicit value.
///
/// # References
///
/// # Examples
/// ```
/// use gsw::conversions::p_from_z;
///
/// let z = p_from_z(-1000.0, 15., None, None).unwrap();
/// assert!((z - 1008.321764487538).abs() <= f64::EPSILON);
/// ```
pub fn p_from_z(
    z: f64,
    lat: f64,
    geo_strf_dyn_height: Option<f64>,
    sea_surface_geopotental: Option<f64>,
) -> Result<f64> {
    if z > 5.0 {
        return Err(Error::Undefined);
    }

    let sinlat = libm::sin(lat * DEG2RAD);
    let sin2 = sinlat * sinlat;
    let gs = 9.780327 * (1.0 + (5.2792e-3 + (2.32e-5 * sin2)) * sin2);

    // get the first estimate of p from Saunders (1981)
    let c1 = 5.25e-3 * sin2 + 5.92e-3;
    let p = -2.0 * z / ((1.0 - c1) + libm::sqrt((1.0 - c1) * (1.0 - c1) + 8.84e-6 * z));
    // end of the first estimate from Saunders (1981)

    // initial value of the derivative of f
    let df_dp = DB2PA * crate::gsw_internal_funcs::specvol_sso_0(p);

    let f = crate::gsw_internal_funcs::enthalpy_sso_0(p) + gs * (z - 0.5 * GAMMA * (z * z))
        - (geo_strf_dyn_height.unwrap_or(0.0) + sea_surface_geopotental.unwrap_or(0.0));
    let p_old = p;
    let p = p_old - f / df_dp;
    let p_mid = 0.5 * (p + p_old);
    let df_dp = DB2PA * crate::gsw_internal_funcs::specvol_sso_0(p_mid);
    let p = p_old - f / df_dp;

    Ok(p)
}

/*
gsw_z_from_depth
gsw_depth_from_z
*/

/// Absolute Pressure, P, from sea pressure, p
///
/// # Arguments
///
/// * `pressure`: Sea Pressure \[dbar\], i.e. absolute pressure - 10.1325 dbar.
///
/// # Returns
///
/// * absolute_pressure: Absolute pressure \[Pa\]
///
/// # References
///
/// # Examples
/// ```
/// use gsw::conversions::abs_pressure_from_p;
///
/// let P = abs_pressure_from_p(100.0);
/// assert!((P - 1101325.0).abs() <= f64::EPSILON);
/// ```
pub fn abs_pressure_from_p(p: f64) -> f64 {
    p * DB2PA + GSW_P0
}

/// Pressure from absolute pressure
///
/// # Arguments
/// * absolute_pressure: Absolute pressure \[Pa\]
///
/// # Returns
///
/// * `pressure`: Sea Pressure \[dbar\], i.e. absolute pressure - 10.1325 dbar.
///
/// # References
///
/// # Examples
/// ```
/// use gsw::conversions::p_from_abs_pressure;
///
/// let p = p_from_abs_pressure(1101325.0);
/// assert!((p - 100.0).abs() <= f64::EPSILON);
/// ```
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
    use super::t90_from_t68;

    #[test]
    fn test_t90_from_t68() {
        assert!((0.0 - t90_from_t68(0.0)).abs() < f64::EPSILON);
        if cfg!(feature = "compat") {
            assert!((9.999999999999996 - t90_from_t68(10.0024)).abs() < f64::EPSILON);
        } else {
            assert!((10.0 - t90_from_t68(10.0024)).abs() < f64::EPSILON);
        }
    }
}
