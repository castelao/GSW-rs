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
    let v000: f64 = 1.0769995862e-3;
    let v001: f64 = -6.0799143809e-5;
    let v002: f64 = 9.9856169219e-6;
    let v003: f64 = -1.1309361437e-6;
    let v004: f64 = 1.0531153080e-7;
    let v005: f64 = -1.2647261286e-8;
    let v006: f64 = 1.9613503930e-9;
    let v010: f64 = -1.5649734675e-5;
    let v011: f64 = 1.8505765429e-5;
    let v012: f64 = -1.1736386731e-6;
    let v013: f64 = -3.6527006553e-7;
    let v014: f64 = 3.1454099902e-7;
    let v020: f64 = 2.7762106484e-5;
    let v021: f64 = -1.1716606853e-5;
    let v022: f64 = 2.1305028740e-6;
    let v023: f64 = 2.8695905159e-7;
    let v030: f64 = -1.6521159259e-5;
    let v031: f64 = 7.9279656173e-6;
    let v032: f64 = -4.6132540037e-7;
    let v040: f64 = 6.9111322702e-6;
    let v041: f64 = -3.4102187482e-6;
    let v042: f64 = -6.3352916514e-8;
    let v050: f64 = -8.0539615540e-7;
    let v051: f64 = 5.0736766814e-7;
    let v060: f64 = 2.0543094268e-7;
    let v100: f64 = -3.1038981976e-4;
    let v101: f64 = 2.4262468747e-5;
    let v102: f64 = -5.8484432984e-7;
    let v103: f64 = 3.6310188515e-7;
    let v104: f64 = -1.1147125423e-7;
    let v110: f64 = 3.5009599764e-5;
    let v111: f64 = -9.5677088156e-6;
    let v112: f64 = -5.5699154557e-6;
    let v113: f64 = -2.7295696237e-7;
    let v120: f64 = -3.7435842344e-5;
    let v121: f64 = -2.3678308361e-7;
    let v122: f64 = 3.9137387080e-7;
    let v130: f64 = 2.4141479483e-5;
    let v131: f64 = -3.4558773655e-6;
    let v132: f64 = 7.7618888092e-9;
    let v140: f64 = -8.7595873154e-6;
    let v141: f64 = 1.2956717783e-6;
    let v150: f64 = -3.3052758900e-7;
    let v200: f64 = 6.6928067038e-4;
    let v201: f64 = -3.4792460974e-5;
    let v202: f64 = -4.8122251597e-6;
    let v203: f64 = 1.6746303780e-8;
    let v210: f64 = -4.3592678561e-5;
    let v211: f64 = 1.1100834765e-5;
    let v212: f64 = 5.4620748834e-6;
    let v220: f64 = 3.5907822760e-5;
    let v221: f64 = 2.9283346295e-6;
    let v222: f64 = -6.5731104067e-7;
    let v230: f64 = -1.4353633048e-5;
    let v231: f64 = 3.1655306078e-7;
    let v240: f64 = 4.3703680598e-6;
    let v300: f64 = -8.5047933937e-4;
    let v301: f64 = 3.7470777305e-5;
    let v302: f64 = 4.9263106998e-6;
    let v310: f64 = 3.4532461828e-5;
    let v311: f64 = -9.8447117844e-6;
    let v312: f64 = -1.3544185627e-6;
    let v320: f64 = -1.8698584187e-5;
    let v321: f64 = -4.8826139200e-7;
    let v330: f64 = 2.2863324556e-6;
    let v400: f64 = 5.8086069943e-4;
    let v401: f64 = -1.7322218612e-5;
    let v402: f64 = -1.7811974727e-6;
    let v410: f64 = -1.1959409788e-5;
    let v411: f64 = 2.5909225260e-6;
    let v420: f64 = 3.8595339244e-6;
    let v500: f64 = -2.1092370507e-4;
    let v501: f64 = 3.0927427253e-6;
    let v510: f64 = 1.3864594581e-6;
    let v600: f64 = 3.1932457305e-5;

    /// sfac  =  1/(40*gsw_ups)
    const GSW_SFAC: f64 = 0.0248826675584615;

    // deltaS = 24, offset = deltaS * gsw_sfac
    const OFFSET: f64 = 5.971840214030754e-1;

    let xs: f64 = (GSW_SFAC * sa + OFFSET).sqrt();
    let ys: f64 = ct * 0.025;
    let z: f64 = p * 1e-4;

    let value = v000
        + xs * (v100 + xs * (v200 + xs * (v300 + xs * (v400 + xs * (v500 + xs * v600)))))
        + ys * (v010
            + xs * (v110 + xs * (v210 + xs * (v310 + xs * (v410 + xs * v510))))
            + ys * (v020
                + xs * (v120 + xs * (v220 + xs * (v320 + xs * v420)))
                + ys * (v030
                    + xs * (v130 + xs * (v230 + xs * v330))
                    + ys * (v040
                        + xs * (v140 + xs * v240)
                        + ys * (v050 + xs * v150 + ys * v060)))))
        + z * (v001
            + xs * (v101 + xs * (v201 + xs * (v301 + xs * (v401 + xs * v501))))
            + ys * (v011
                + xs * (v111 + xs * (v211 + xs * (v311 + xs * v411)))
                + ys * (v021
                    + xs * (v121 + xs * (v221 + xs * v321))
                    + ys * (v031
                        + xs * (v131 + xs * v231)
                        + ys * (v041 + xs * v141 + ys * v051))))
            + z * (v002
                + xs * (v102 + xs * (v202 + xs * (v302 + xs * v402)))
                + ys * (v012
                    + xs * (v112 + xs * (v212 + xs * v312))
                    + ys * (v022
                        + xs * (v122 + xs * v222)
                        + ys * (v032 + xs * v132 + ys * v042)))
                + z * (v003
                    + xs * (v103 + xs * v203)
                    + ys * (v013 + xs * v113 + ys * v023)
                    + z * (v004 + xs * v104 + ys * v014 + z * (v005 + z * v006)))));

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
    let v005: f64 = -1.2647261286e-8;
    let v006: f64 = 1.9613503930e-9;

    let z = p * 1.0e-4;

    9.726613854843870e-04
        + z * (-4.505913211160929e-05
            + z * (7.130728965927127e-06
                + z * (-6.657179479768312e-07
                    + z * (-2.994054447232880e-08 + z * (v005 + v006 * z)))))
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
