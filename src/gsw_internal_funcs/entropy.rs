//! Internal functions related to entropy

use super::sanitize_salinity;
use crate::gsw_internal_const::GSW_SFAC;
use crate::Result;

#[inline]
#[allow(dead_code)]
/// Entropy part evaluated at the sea surface (zero pressure)
///
/// Calculates entropy at zero sea pressure, but omits terms that depend solely
/// on Absolute Salinity. Excluding these terms avoids unnecessary computation
/// (including a natural logarithm evaluation), and is valid because they are
/// not required when deriving potential temperature from in-situ temperature.
///
/// # Arguments
///
/// * `sa`: Absolute Salinity \[g kg⁻¹\]
/// * `pt0`: Potential temperature with reference sea pressure of zero
///   dbar (ITS-90) \[deg C\]
///
/// # Returns
///
/// * `entropy_part_zerop`: Entropy part at zero pressure \[J/(kg·K)\]
///
/// # Notes
///
/// - Compatible with GSW-Matlab version: 3.06.12 (25th May, 2020)
///
/// # Features
///
/// * default: Negative salinity returns [Error::NegativeSalinity].
/// * compat:
///   * Negative salinity is assumed to be zero.
///   * S factor is approximated to 0.0248826675584615, which can cause a
///     minimal difference.
/// * invalidasnan: Negative salinity results in NaN.
///
/// # References
///
/// * IOC, SCOR and IAPSO, 2010: The international thermodynamic equation of
///   seawater - 2010: Calculation and use of thermodynamic properties.
///   Intergovernmental Oceanographic Commission, Manuals and Guides No. 56,
///   UNESCO (English), 196 pp. Available from <http://www.TEOS-10.org>
///
/// # Example
/// ```ignore
/// use crate::gsw_internal_funcs::entropy_part_zerop;
/// let entropy = entropy_part_zerop(35.0, 10.0).unwrap();
/// ```
pub(crate) fn entropy_part_zerop(sa: f64, pt0: f64) -> Result<f64> {
    let sa = sanitize_salinity(sa)?;

    let x2 = GSW_SFAC * sa;
    let x = libm::sqrt(x2);
    let y = pt0 * 0.025;

    // Temperature-dependent part of the Gibbs function
    let g03 = y
        * (-24715.571866078
            + y * (2210.2236124548363
                + y * (-592.743745734632
                    + y * (290.12956292128547
                        + y * (-113.90630790850321 + y * 21.35571525415769)))));

    // Salinity-temperature interaction part of the Gibbs function
    let g08 = x2
        * (x * (x
            * (y * (-137.1145018408982
                + y * (148.10030845687618 + y * (-68.5590309679152 + 12.4848504784754 * y))))
            + y * (-86.1329351956084 + y * (-30.0682112585625 + y * 3.50240264723578)))
            + y * (1760.062705994408
                + y * (-675.802947790203
                    + y * (365.7041791005036
                        + y * (-108.30162043765552 + 12.78101825083098 * y)))));

    Ok(-(g03 + g08) * 0.025)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_values() {
        let entropy = entropy_part_zerop(35.0, 10.0).unwrap();
        assert!((entropy - 143.4171178988772).abs() <= f64::EPSILON);
    }

    #[test]
    fn test_zero_salinity() {
        let entropy = entropy_part_zerop(0.0, 10.0).unwrap();
        assert!((entropy - 151.22470789985408).abs() <= f64::EPSILON);
    }

    #[test]
    #[cfg(not(any(feature = "compat", feature = "invalidasnan")))]
    // Confirming that negative salinity returns an error by default
    fn test_negative_salinity_error() {
        let entropy = entropy_part_zerop(-1_000.0, 10.0);
        assert!(matches!(entropy, Err(crate::Error::NegativeSalinity)));
    }

    #[test]
    #[cfg(feature = "compat")]
    // Any negative salinity is equal to 0.0 salinity
    fn test_negative_salinity_compat() {
        let entropy = entropy_part_zerop(-1_000.0, 10.0).unwrap();
        let expected = entropy_part_zerop(0.0, 10.0).unwrap();
        assert!((entropy - expected).abs() < f64::EPSILON);
    }

    #[test]
    #[cfg(all(not(feature = "compat"), feature = "invalidasnan"))]
    // Confirming the `invalidasnan` behavior for negative salinity
    fn test_negative_salinity_nan() {
        let result = entropy_part_zerop(-1e-4, 10.0).unwrap();
        assert!(result.is_nan());
    }

    #[test]
    // NaN input should result in NaN output
    fn test_nan_propagation() {
        let result = entropy_part_zerop(f64::NAN, 10.0).unwrap();
        assert!(result.is_nan());

        let result = entropy_part_zerop(35.0, f64::NAN).unwrap();
        assert!(result.is_nan());

        let result = entropy_part_zerop(f64::NAN, f64::NAN).unwrap();
        assert!(result.is_nan());
    }

    #[test]
    fn test_extreme_values() {
        // Test with extreme but valid oceanographic values
        let result = entropy_part_zerop(0.0, -2.0).unwrap();
        assert!(result.is_finite());

        let result = entropy_part_zerop(42.0, 30.0).unwrap();
        assert!(result.is_finite());

        let result = entropy_part_zerop(50.0, 40.0).unwrap();
        assert!(result.is_finite());
    }
}

/*
#[cfg(test)]
mod benchmark_tests {
    use super::*;

    #[test]
    fn test_performance_check() {
        // Simple performance check - should complete quickly
        let start = std::time::Instant::now();
        for i in 0..1000 {
            let sa = 30.0 + (i as f64) * 0.01;
            let pt0 = 5.0 + (i as f64) * 0.01;
            let _ = entropy_part_zerop(sa, pt0).unwrap();
        }
        let duration = start.elapsed();
        // Should complete 1000 iterations in well under 1 second
        assert!(duration.as_millis() < 100);
    }
}
*/
