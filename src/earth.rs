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
    if !(-90.0..=90.0).contains(&lat) {
        return Err(Error::Undefined);
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
/// * If feature nodgdz is activated, GAMMA is zero, thus this can be
///   simplified.
pub fn gravity(lat: f64, p: f64) -> Result<f64> {
    if !(-90.0..=90.0).contains(&lat) {
        return Err(Error::Undefined);
    }

    let sinlat = libm::sin(lat * DEG2RAD);
    let sin2 = sinlat * sinlat;
    let gs = 9.780327 * (1.0 + (5.2792e-3 + (2.32e-5 * sin2)) * sin2);

    // Here we use the convention that the height z, corresponding to the given
    // pressure p, is negative in the ocean.
    let z = z_from_p(p, lat, 0.0, 0.0);
    Ok(gs * (1.0 - GAMMA * z))
}

/// Distance between two coordinates on Earth
///
/// # Arguments
///  * `lon1`: longitude of the first waypoint \[decimal degrees\]
///  * `lat1`: latitude of the first waypoint \[decimal degrees\]
///  * `p1`: sea pressure of the first waypoint \[dbar\] (i.e. absolute
///          pressure - 10.1325 dbar)
///  * `lon2`: longitude of the second waypoint \[decimal degrees\]
///  * `lat2`: latitude of the second waypoint \[decimal degrees\]
///  * `p2`: sea pressure of the second waypoint \[dbar\] (i.e. absolute
///          pressure - 10.1325 dbar)
///
/// # Returns
/// * The 'horizontal' distance between two coordinates \[m\]
///
/// # Example
/// ```
/// use gsw::earth::distance;
/// let d = distance(-38.0, 15.0, 100.0, -38.0, 12.0, 105.0).unwrap();
/// assert_eq!(d.round(), 333579.0);
/// ```
pub fn distance(lon1: f64, lat1: f64, p1: f64, lon2: f64, lat2: f64, p2: f64) -> Result<f64> {
    if !(-90.0..=90.0).contains(&lat1) | !(-90.0..=90.0).contains(&lat2) {
        return Err(Error::Undefined);
    }

    // Earth's radius in metres
    let earth_radius = 6371000.0;

    let dlon = (lon2 - lon1) * DEG2RAD;
    let dlat = (lat2 - lat1) * DEG2RAD;
    let sdlat = libm::sin(0.5 * dlat);
    let sdlon = libm::sin(0.5 * dlon);

    let a =
        (sdlat * sdlat) + libm::cos(lat1 * DEG2RAD) * libm::cos(lat2 * DEG2RAD) * (sdlon * sdlon);
    let angles = 2.0 * libm::atan2(libm::sqrt(a), libm::sqrt(1.0 - a));

    let p_mid = 0.5 * (p1 + p2);
    let lat_mid = 0.5 * (lat1 + lat2);
    // Here we use the convention that the height z, corresponding to the given
    // pressure p, is negative in the ocean.
    let z = z_from_p(p_mid, lat_mid, 0.0, 0.0);

    Ok((earth_radius + z) * angles)
}

#[cfg(test)]
mod test_distance {
    use super::distance;

    #[test]
    fn distance_along_equator() {
        let d = distance(0.0, 0.0, 0.0, 1.0, 0.0, 0.0).unwrap();
        assert!((d - 111194.92664455874).abs() <= f64::EPSILON);
    }

    #[test]
    fn distance_high_latitude() {
        let d = distance(0.0, 75.0, 0.0, 1.0, 75.0, 0.0).unwrap();
        assert!((d - 28779.023924964815).abs() <= f64::EPSILON);
    }

    #[test]
    fn distance_with_depth() {
        let d = distance(0.0, 0.0, 0.0, 0.0, 1.0, 1000.0).unwrap();
        assert!((d - 111186.2584129753).abs() <= f64::EPSILON);
    }

    #[test]
    // Vertical distance is not included
    fn distance_on_depth() {
        let d = distance(0.0, 0.0, 0.0, 0.0, 0.0, 1000.0).unwrap();
        assert!((d - 0.0).abs() <= f64::EPSILON);
    }

    #[test]
    // Distance reduces with depth (average depth between p1 & p2)
    fn distance_depth_effect() {
        let d_surf = distance(0.0, 0.0, 0.0, 0.0, 0.1, 0.0).unwrap();
        let d_deep = distance(0.0, 0.0, 0.0, 0.0, 0.1, 4000.0).unwrap();
        let diff = d_surf - d_deep;
        assert!((diff - 3.4549677662762406).abs() <= f64::EPSILON);
    }
}
