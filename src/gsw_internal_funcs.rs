//! Internal Functions
//!
//! Functions not intended to be used outside this library

use crate::gsw_internal_const::DB2PA;

/// Specific Volume of Standard Ocean Salinity and CT=0
///
/// This function calculates specific volume at the Standard Ocean Salinity,
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

    let p = crate::volume::non_dimensional_p(p);

    VXX0 + p
        * (VXX1
            + p * (VXX2
                + p * (VXX3
                    + p * (VXX4
                        + p * (crate::gsw_specvol_coefficients::V005
                            + crate::gsw_specvol_coefficients::V006 * p)))))
}

pub(crate) fn enthalpy_sso_0(p: f64) -> f64 {
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

#[cfg(test)]
mod tests {
    use super::specvol_sso_0;
    use crate::gsw_internal_const::GSW_SSO;
    use crate::volume::specvol;

    /// specvol() at SSO & CT=0 should be identical to specvol_sso_0()
    #[test]
    fn test_specvol_vs_specvol_sso_0() {
        let p_to_test: [f64; 5] = [0., 10., 100., 1000., 5000.];
        for p in p_to_test.iter().cloned() {
            let specvol = specvol(GSW_SSO, 0., p).unwrap();
            let specvol_sso_0 = specvol_sso_0(p);
            assert!((specvol - specvol_sso_0).abs() < f64::EPSILON);
        }
    }
}
