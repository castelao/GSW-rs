//! Internal Functions
//!
//! Functions not intended to be used outside this library

use crate::gsw_internal_const::{DB2PA, GSW_PU};
use crate::gsw_specvol_coefficients::{V005, V006};

const A0: f64 = 0.008_0;
const A1: f64 = -0.169_2;
const A2: f64 = 25.385_1;
const A3: f64 = 14.094_1;
const A4: f64 = -7.026_1;
const A5: f64 = 2.708_1;

const B0: f64 = 0.000_5;
const B1: f64 = -0.005_6;
const B2: f64 = -0.006_6;
const B3: f64 = -0.037_5;
const B4: f64 = 0.063_6;
const B5: f64 = -0.014_4;

// Consider rename K to something different
const K: f64 = 0.016_2;

const G0: f64 = 2.641_463_563_366_498e-1;
const G1: f64 = 2.007_883_247_811_176e-4;
const G2: f64 = -4.107_694_432_853_053e-6;
const G3: f64 = 8.401_670_882_091_225e-8;
const G4: f64 = -1.711_392_021_989_210e-9;
const G5: f64 = 3.374_193_893_377_380e-11;
const G6: f64 = -5.923_731_174_730_784e-13;
const G7: f64 = 8.057_771_569_962_299e-15;
const G8: f64 = -7.054_313_817_447_962e-17;
const G9: f64 = 2.859_992_717_347_235e-19;

#[inline]
/// Non-dimensional pressure
///
/// The polynomial approximation solutions proposed by Roquet (2015) are based
/// on non-dimensional salinity, temperature, and pressure. Here we scale
/// pressure by p_u (1e4 [dbar]) to obtain the non-dimensional quantity \pi
/// (TEOS-10 Manual appendix K, or \zeta on Roquet (2015)).
///
/// # Argument
///
/// * `p`: sea pressure \[dbar\] (i.e. absolute pressure - 10.1325 dbar)
///
/// # Returns
///
/// * `\pi`: Non-dimensional pressure
///
/// # Notes
///
/// * The original formulation is a scaling of p by p_u. The MatLab and C
///   implementations of GSW operate as the product with 1e-4, which does make
///   sense since it is a lighter operation than a division is for computers.
///   The issue here is on the inhability of f64 to fully represent certain
///   fractions. For instance, while 3812.0 can be perfectly represented,
///   0.3812 is rounded to 0.381200000000000038813. Simmilarly 1e4 is fine but
///   1e-4 on f64 is rounded to 0.000100000000000000004792, thus 3812/1e4 is
///   diffrent than 3812*1e-4.
///
/// # Example
/// ```
/// let p = 3812;
/// assert!((3812.0 * 1e-4) > (3812.0 / 1e4))
/// ```
pub(crate) fn non_dimensional_p(p: f64) -> f64 {
    if cfg!(feature = "compat") {
        p * 1e-4
    } else {
        p / GSW_PU
    }
}

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

    let p = non_dimensional_p(p);
    VXX0 + p * (VXX1 + p * (VXX2 + p * (VXX3 + p * (VXX4 + p * (V005 + V006 * p)))))
}

#[cfg(test)]
mod test_specvol_sso_0 {
    use super::specvol_sso_0;
    use crate::gsw_internal_const::GSW_SSO;
    use crate::volume::specvol;

    /// specvol() at SSO & CT=0 should be identical to specvol_sso_0()
    #[test]
    fn specvol_vs_specvol_sso_0() {
        let p_to_test: [f64; 5] = [0., 10., 100., 1000., 5000.];
        for p in p_to_test.iter().cloned() {
            let specvol = specvol(GSW_SSO, 0., p).unwrap();
            let specvol_sso_0 = specvol_sso_0(p);
            assert!((specvol - specvol_sso_0).abs() < f64::EPSILON);
        }
    }
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

pub(crate) fn hill_ratio_at_sp2(t: f64) -> f64 {
    let sp2 = 2.0;

    let t68: f64 = t * 1.00024;
    let ft68: f64 = (t68 - 15.0) / (1.0 + K * (t68 - 15.0));

    // Find the initial estimates of Rtx (Rtx0) and of the derivative dSP_dRtx
    // at SP = 2.
    let rtx0: f64 = G0
        + t68
            * (G1
                + t68
                    * (G2
                        + t68
                            * (G3
                                + t68
                                    * (G4
                                        + t68
                                            * (G5
                                                + t68
                                                    * (G6
                                                        + t68 * (G7 + t68 * (G8 + t68 * G9))))))));

    let dsp_drtx: f64 = A1
        + (2.0 * A2 + (3.0 * A3 + (4.0 * A4 + 5.0 * A5 * rtx0) * rtx0) * rtx0) * rtx0
        + ft68 * (B1 + (2.0 * B2 + (3.0 * B3 + (4.0 * B4 + 5.0 * B5 * rtx0) * rtx0) * rtx0) * rtx0);

    //  Begin a single modified Newton-Raphson iteration (McDougall and
    //  Wotherspoon, 2013) to find Rt at SP = 2.

    let sp_est: f64 = A0
        + (A1 + (A2 + (A3 + (A4 + A5 * rtx0) * rtx0) * rtx0) * rtx0) * rtx0
        + ft68 * (B0 + (B1 + (B2 + (B3 + (B4 + B5 * rtx0) * rtx0) * rtx0) * rtx0) * rtx0);
    let rtx: f64 = rtx0 - (sp_est - sp2) / dsp_drtx;
    let rtxm: f64 = 0.5 * (rtx + rtx0);
    let dsp_drtx: f64 = A1
        + (2.0 * A2 + (3.0 * A3 + (4.0 * A4 + 5.0 * A5 * rtxm) * rtxm) * rtxm) * rtxm
        + ft68 * (B1 + (2.0 * B2 + (3.0 * B3 + (4.0 * B4 + 5.0 * B5 * rtxm) * rtxm) * rtxm) * rtxm);
    let rtx: f64 = rtx0 - (sp_est - sp2) / dsp_drtx;

    // This is the end of one full iteration of the modified Newton-Raphson
    // iterative equation solver.  The error in Rtx at this point is
    // equivalent to an error in SP of 9e-16 psu.

    let x: f64 = 400.0 * rtx * rtx;
    let sqrty: f64 = 10.0 * rtx;
    let part1: f64 = 1.0 + x * (1.5 + x);
    let part2: f64 = 1.0 + sqrty * (1.0 + sqrty * (1.0 + sqrty));
    let sp_hill_raw_at_sp2: f64 = sp2 - A0 / part1 - B0 * ft68 / part2;

    2.0 / sp_hill_raw_at_sp2
}

#[cfg(test)]
mod test_hill_ratio_at_sp2 {
    use super::hill_ratio_at_sp2;

    #[test]
    fn example_values() {
        let ratio = hill_ratio_at_sp2(10.);
        assert!((ratio - 0.9999586761759697).abs() <= f64::EPSILON);

        let ratio = hill_ratio_at_sp2(25.);
        assert!((ratio - 1.0000764830968472).abs() <= f64::EPSILON);
    }
}
