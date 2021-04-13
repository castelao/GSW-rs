use crate::gsw_internal_const::*;
use crate::gsw_specvol_coefficients::*;

/// Calculates specific volume of sea water
///
/// Calculates specific volume from Absolute Salinity, Conservative
/// Temperature and pressure, using the computationally-efficient
/// polynomial expression for specific volume (Roquet et al., 2014).
///
/// sa [g/kg] : Absolute Salinity
/// ct [deg C] : Conservative Temperature (ITS-90)
/// p [dbar] : sea pressure ( i.e. absolute pressure - 10.1325 dbar )
///
/// specvol [m^3/kg] : specific volume
///
/// Note that the coefficients v(i,j,k) follow the convention in the original
/// paper, which is different from the convention used in the C-library.
///
pub fn specvol(sa: f64, ct: f64, p: f64) -> f64 {
    // Other implementations force negative SA to be 0. That is dangerous
    // since it can hide error by processing unrealistic inputs
    let sa: f64 = if (sa > 0.0) {
        sa
    } else if cfg!(feature = "compat") {
        0.0
    } else {
        panic!("Negative SA");
    };

    let xs: f64 = libm::sqrt(GSW_SFAC * sa + OFFSET);
    let ys: f64 = ct / GSW_CTU;
    let z: f64 = p / GSW_PU;

    // Specific Volume
    V000 + xs * (V100 + xs * (V200 + xs * (V300 + xs * (V400 + xs * (V500 + xs * V600)))))
        + ys * (V010
            + xs * (V110 + xs * (V210 + xs * (V310 + xs * (V410 + xs * V510))))
            + ys * (V020
                + xs * (V120 + xs * (V220 + xs * (V320 + xs * V420)))
                + ys * (V030
                    + xs * (V130 + xs * (V230 + xs * V330))
                    + ys * (V040 + xs * (V140 + xs * V240) + ys * (V050 + xs * V150 + ys * V060)))))
        + z * (V001
            + xs * (V101 + xs * (V201 + xs * (V301 + xs * (V401 + xs * V501))))
            + ys * (V011
                + xs * (V111 + xs * (V211 + xs * (V311 + xs * V411)))
                + ys * (V021
                    + xs * (V121 + xs * (V221 + xs * V321))
                    + ys * (V031 + xs * (V131 + xs * V231) + ys * (V041 + xs * V141 + ys * V051))))
            + z * (V002
                + xs * (V102 + xs * (V202 + xs * (V302 + xs * V402)))
                + ys * (V012
                    + xs * (V112 + xs * (V212 + xs * V312))
                    + ys * (V022 + xs * (V122 + xs * V222) + ys * (V032 + xs * V132 + ys * V042)))
                + z * (V003
                    + xs * (V103 + xs * V203)
                    + ys * (V013 + xs * V113 + ys * V023)
                    + z * (V004 + xs * V104 + ys * V014 + z * (V005 + z * V006)))))
}

/// Specific Volume of Standard Ocean Salinity and CT=0
///
/// This function calculates specifc volume at the Standard Ocean Salinity,
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

    let p = p / GSW_PU;

    VXX0 + p * (VXX1 + p * (VXX2 + p * (VXX3 + p * (VXX4 + p * (V005 + V006 * p)))))
}

#[cfg(test)]
mod tests {
    use super::{specvol, specvol_sso_0, GSW_SSO};

    #[test]
    // Test value from Roquet 2015, Appendix C.3
    // rounded to 9.732819628e-04
    fn test_specvol_roquet2015() {
        assert!((specvol(30., 10., 1000.0) - 9.732819628e-04).abs() <= 5e-14);
    }

    #[test]
    fn test_specvol() {
        if cfg!(feature = "compat") {
            // Test value from C library.
            assert_eq!(
                specvol(34.507499465692057, 27.994827331978655, 0.0),
                0.00097855432330275953
            );
        }
    }

    /// specvol() at SSO & CT=0 should be identical to specvol_sso_0()
    #[test]
    fn test_specvol_vs_specvol_sso_0() {
        let p_to_test: [f64; 5] = [0., 10., 100., 1000., 5000.];
        for p in p_to_test.iter().cloned() {
            let specvol = specvol(GSW_SSO, 0., p);
            let specvol_sso_0 = specvol_sso_0(p);
            assert_eq!(specvol, specvol_sso_0);
        }
    }
}
