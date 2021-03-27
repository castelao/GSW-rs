//! Gibbs Sea Water Oceanographic Toolbox of TEOS-10
//!

/// function gsw_specvol(sa,ct,p)
///
/// Calculates specific volume from Absolute Salinity, Conservative
/// Temperature and pressure, using the computationally-efficient
/// polynomial expression for specific volume (Roquet et al., 2014).
///
/// sa     : Absolute Salinity                               [g/kg]
/// ct     : Conservative Temperature (ITS-90)               [deg C]
/// p      : sea pressure                                    [dbar]
///          ( i.e. absolute pressure - 10.1325 dbar )
///
/// specvol: specific volume                                 [m^3/kg]
fn gsw_specvol(sa: f64, ct: f64, p: f64) -> f64 {
    /// sfac  =  1/(40*gsw_ups)
    const GSW_SFAC: f64 = 0.0248826675584615;

    // deltaS = 24, offset = deltaS * gsw_sfac
    const OFFSET: f64 = 5.971840214030754e-1;

    let xs: f64 = (GSW_SFAC * sa + OFFSET).sqrt();
    let ys: f64 = ct * 0.025;
    let z: f64 = p * 1e-4;

    let value = V000
        + xs * (V100 + xs * (V200 + xs * (V300 + xs * (V400 + xs * (V500 + xs * V600)))))
        + ys * (V010
            + xs * (V110 + xs * (V210 + xs * (V310 + xs * (V410 + xs * V510))))
            + ys * (V020
                + xs * (V120 + xs * (V220 + xs * (V320 + xs * V420)))
                + ys * (V030
                    + xs * (V130 + xs * (V230 + xs * V330))
                    + ys * (V040
                        + xs * (V140 + xs * V240)
                        + ys * (V050 + xs * V150 + ys * V060)))))
        + z * (V001
            + xs * (V101 + xs * (V201 + xs * (V301 + xs * (V401 + xs * V501))))
            + ys * (V011
                + xs * (V111 + xs * (V211 + xs * (V311 + xs * V411)))
                + ys * (V021
                    + xs * (V121 + xs * (V221 + xs * V321))
                    + ys * (V031
                        + xs * (V131 + xs * V231)
                        + ys * (V041 + xs * V141 + ys * V051))))
            + z * (V002
                + xs * (V102 + xs * (V202 + xs * (V302 + xs * V402)))
                + ys * (V012
                    + xs * (V112 + xs * (V212 + xs * V312))
                    + ys * (V022
                        + xs * (V122 + xs * V222)
                        + ys * (V032 + xs * V132 + ys * V042)))
                + z * (V003
                    + xs * (V103 + xs * V203)
                    + ys * (V013 + xs * V113 + ys * V023)
                    + z * (V004 + xs * V104 + ys * V014 + z * (V005 + z * V006)))));

    return value;
}

#[cfg(test)]
mod tests {
    use super::gsw_specvol;

    #[test]
    fn test_gsw_specvol() {
        // Test value from Roquet 2015
        let specvol = gsw_specvol(30., 10., 1000.0);
        assert_eq!(specvol, 0.0009732819627722662);

        // Test value from C library.
        let specvol = gsw_specvol(34.507499465692057, 27.994827331978655, 0.0);
        assert_eq!(specvol, 0.00097855432330275953);
    }
}

/// Specific Volume of Standard Ocean Salinity
///
/// This function calculates specifc volume at the Standard Ocean Salinity,
/// SSO, and at a Conservative Temperature of zero degrees C, as a function
/// of pressure, p, in dbar, using a streamlined version of the 75-term CT
/// version of specific volume, that is, a streamlined version of the code
/// "gsw_specvol(SA,CT,p)".
///
/// version: 3.06.12
///
fn gsw_specvol_sso_0(p: f64) -> f64 {
    let z = p * 1.0e-4;

    9.726613854843870e-04
        + z * (-4.505913211160929e-05
            + z * (7.130728965927127e-06
                + z * (-6.657179479768312e-07
                    + z * (-2.994054447232880e-08 + z * (V005 + V006 * z)))))
}

fn gsw_enthalpy_sso_0(p: f64) -> f64 {
    let h006 = -2.10787688100e-9;
    let h007 = 2.80192913290e-10;
    let db2pa = 1.0e4;

    let z = p * 1.0e-4;

    let dynamic_enthalpy_sso_0_p = z
        * (9.726613854843870e-4
            + z * (-2.252956605630465e-5
                + z * (2.376909655387404e-6
                    + z * (-1.664294869986011e-7
                        + z * (-5.988108894465758e-9 + z * (h006 + h007 * z))))));

    return dynamic_enthalpy_sso_0_p * db2pa * 1.0e4;
}
