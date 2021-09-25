//! Planet Earth properties

use crate::conversions::z_from_p;
use crate::gsw_internal_const::{DEG2RAD, GAMMA};
use crate::{Error, Result};

/// Coriolis parameter (f)
///
/// # Example
/// ```
/// use gsw::earth::coriollis_parameter;
/// let f = coriollis_parameter(5.0).unwrap();
/// assert!((f - 1.2710993980526786e-05).abs() <= f64::EPSILON);
/// ```
pub fn coriollis_parameter(lat: f64) -> Result<f64> {
    if (lat < -90.0) | (lat > 90.0) {
        return Err(Error::InvalidValue);
    }

    let omega = 7.292115e-5;
    Ok(2.0 * omega * libm::sin(lat * DEG2RAD))
}

/// Gravity in the ocean
///
/// Calculates acceleration due to gravity as a function of latitude and as a
/// function of pressure in the ocean.
///
/// # Example:
/// ```
/// use gsw::earth::gravity;
/// let g = gravity(-18.0, 1250.0).unwrap();
/// assert!((g - 9.78799904835888).abs() <= f64::EPSILON);
/// ```
///
/// # Notes:
/// * Here we use the convention that the height z, corresponding to the given
///   pressure p, is negative in the ocean.
/// * If feature nodgdz is activated, GAMMA is zero, thus this can be
///   simplified.
pub fn gravity(lat: f64, p: f64) -> Result<f64> {
    if (lat < -90.0) | (lat > 90.0) {
        return Err(Error::InvalidValue);
    }

    let sinlat = libm::sin(lat * DEG2RAD);
    let sin2 = sinlat * sinlat;
    let gs = 9.780327 * (1.0 + (5.2792e-3 + (2.32e-5 * sin2)) * sin2);

    let z = z_from_p(p, lat, 0.0, 0.0);
    Ok(gs * (1.0 - GAMMA * z))
}
