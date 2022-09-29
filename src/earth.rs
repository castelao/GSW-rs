//! Planet Earth properties

use crate::conversions::z_from_p;
use crate::gsw_internal_const::{DEG2RAD, GAMMA};
use crate::{Error, Result};

/// Coriolis parameter (f)
///
/// # Arguments
///
/// * `lat`: latitude \[decimal degrees\] (positive northward)
///
/// # Returns
///
/// * `f`: coriolis parameter \[s-1\]
///
/// # References
///
/// Gill, A. E., & Adrian, E. (1982). Atmosphere-ocean dynamics (Vol. 30).
/// Academic press.
///
/// Groten, E., 2004: Fundamental Parameters and Current (2004) Best
/// Estimates of the Parameters of Common Relevance to Astronomy, Geodesy,
/// and Geodynamics. Journal of Geodesy, 77, pp. 724-797.
///
/// # Example
/// ```
/// use gsw::earth::coriollis_parameter;
/// let f = coriollis_parameter(5.0).unwrap();
/// assert!((f - 1.2710993980526786e-05).abs() <= f64::EPSILON);
/// ```
pub fn coriollis_parameter(lat: f64) -> Result<f64> {
    if !(-90.0..=90.0).contains(&lat) {
        if lat.is_nan() {
            return Ok(f64::NAN);
        } else {
            return Err(Error::OutOfBounds);
        }
    }

    let omega = 7.292115e-5;
    Ok(2.0 * omega * libm::sin(lat * DEG2RAD))
}

#[cfg(test)]
mod test_coriollis_parameter {
    use super::{coriollis_parameter, Error};

    #[test]
    fn poles() {
        // North pole
        let f = coriollis_parameter(90.0).unwrap();
        assert!((f - 0.0001458423).abs() <= f64::EPSILON);

        // South pole
        let f = coriollis_parameter(-90.0).unwrap();
        assert!((f + 0.0001458423).abs() <= f64::EPSILON);
    }

    #[test]
    fn equator() {
        let f = coriollis_parameter(0.0).unwrap();
        assert!((f).abs() <= f64::EPSILON);
    }

    #[test]
    fn nan() {
        let f = coriollis_parameter(f64::NAN).unwrap();
        assert!(f.is_nan());
    }

    #[test]
    fn out_of_limites() {
        for lat in [-91.0, 91.0] {
            let f = coriollis_parameter(lat);
            match f {
                Err(Error::OutOfBounds) => (),
                _ => panic!("Out of limits. It should be Error::OutOfBounds"),
            }
        }
    }
}

/// Gravity in the ocean
///
/// Calculates acceleration due to gravity as a function of latitude and
/// pressure in the ocean.
///
/// # Arguments
///
/// * `lat`: latitude \[degree\] (positive northward)
/// * `p`: sea pressure \[ dbar \] (i.e. absolute pressure - 10.1325 dbar)
///
/// # Returns
///
/// * `g`: gravitational acceleration \[m s-2\]
///
/// # References
///
/// Moritz, H. (2000). Geodetic reference system 1980. Journal of Geodesy,
/// 74(1), pp. 128-133.
///
/// Saunders, P. M., & Fofonoff, N. P. (1976, January). Conversion of pressure
/// to depth in the ocean. In Deep Sea Research and Oceanographic Abstracts
/// (Vol. 23, No. 1, pp. 109-111). Elsevier.
///
/// # Example:
/// ```
/// use gsw::earth::gravity;
/// let g = gravity(-18.0, 1250.0).unwrap();
/// assert!((g - 9.78799904835888).abs() <= f64::EPSILON);
/// ```
pub fn gravity(lat: f64, p: f64) -> Result<f64> {
    if !(-90.0..=90.0).contains(&lat) {
        return Err(Error::OutOfBounds);
    }

    let sinlat = libm::sin(lat * DEG2RAD);
    let sin2 = sinlat * sinlat;
    let gs = 9.780327 * (1.0 + (5.2792e-3 + (2.32e-5 * sin2)) * sin2);

    // If nodgdz is activate, GAMMA is zero, thus this function can be simplified.
    if cfg!(feature = "nodgdz") {
        return Ok(gs);
    }

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
