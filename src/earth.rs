//! Planet Earth properties

use crate::gsw_internal_const::DEG2RAD;
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
