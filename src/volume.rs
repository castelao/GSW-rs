//! Specific volume, density, and enthalpy (75-term polynomial approximation)
//!

use crate::gsw_internal_const::*;
use crate::gsw_internal_funcs::non_dimensional_p;
use crate::gsw_specvol_coefficients::*;
use crate::{Error, Result};

#[inline]
/// Non-dimensional salinity
fn non_dimensional_sa(sa: f64) -> Result<f64> {
    // Other implementations force negative SA to be 0. That is dangerous
    // since it can hide error by processing unrealistic inputs
    let sa: f64 = if sa < 0.0 {
        if cfg!(feature = "compat") {
            0.0
        } else if cfg!(feature = "invalidasnan") {
            return Ok(f64::NAN);
        } else {
            return Err(Error::NegativeSalinity);
        }
    } else {
        sa
    };

    Ok(libm::sqrt(GSW_SFAC * sa + OFFSET))
}

/// in-situ density
///
/// Calculates in-situ density from Absolute Salinity and Conservative
/// Temperature, using the computationally-efficient expression for
/// specific volume in terms of SA, CT and p (Roquet et al., 2014).
///
/// # Arguments
///
/// * `sa`: Absolute Salinity \[g kg-1\]
/// * `ct`: Conservative Temperature (ITS-90) \[deg C\]
/// * `p`: sea pressure \[dbar\] (i.e. absolute pressure - 10.1325 dbar)
///
/// # Returns
///
/// * `rho`: in-situ density \[kg m-3\]
///
pub fn rho(sa: f64, ct: f64, p: f64) -> Result<f64> {
    Ok(1.0 / specvol(sa, ct, p)?)
}

/// Thermal expansion coefficient with respect to Conservative Temperature
/// (75-term polynomial approximation)
///
/// # Arguments
///
/// * `sa`: Absolute Salinity \[ g kg-1 \]
/// * `ct`: Conservative Temperature (ITS-90) \[ deg C \]
/// * `p`: sea pressure \[ dbar \] (i.e. absolute pressure - 10.1325 dbar)
///
pub fn alpha(sa: f64, ct: f64, p: f64) -> Result<f64> {
    let xs: f64 = non_dimensional_sa(sa)?;
    let ys: f64 = ct / GSW_CTU;
    let z: f64 = non_dimensional_p(p);

    let v_ct: f64 = A000
        + xs * (A100 + xs * (A200 + xs * (A300 + xs * (A400 + A500 * xs))))
        + ys * (A010
            + xs * (A110 + xs * (A210 + xs * (A310 + A410 * xs)))
            + ys * (A020
                + xs * (A120 + xs * (A220 + A320 * xs))
                + ys * (A030 + xs * (A130 + A230 * xs) + ys * (A040 + A140 * xs + A050 * ys))))
        + z * (A001
            + xs * (A101 + xs * (A201 + xs * (A301 + A401 * xs)))
            + ys * (A011
                + xs * (A111 + xs * (A211 + A311 * xs))
                + ys * (A021 + xs * (A121 + A221 * xs) + ys * (A031 + A131 * xs + A041 * ys)))
            + z * (A002
                + xs * (A102 + xs * (A202 + A302 * xs))
                + ys * (A012 + xs * (A112 + A212 * xs) + ys * (A022 + A122 * xs + A032 * ys))
                + z * (A003 + A103 * xs + A013 * ys + A004 * z)));

    Ok(0.025 * v_ct / specvol(sa, ct, p)?)
}

/// Saline contraction coefficient at constant Conservative Temperature
/// (75-term polynomial approximation)
///
/// # Arguments
///
/// * `sa`: Absolute Salinity \[ g kg-1 \]
/// * `ct`: Conservative Temperature (ITS-90) \[ deg C \]
/// * `p`: sea pressure \[ dbar \] (i.e. absolute pressure - 10.1325 dbar)
///
pub fn beta(sa: f64, ct: f64, p: f64) -> Result<f64> {
    let xs: f64 = non_dimensional_sa(sa)?;
    let ys: f64 = ct / GSW_CTU;
    let z: f64 = non_dimensional_p(p);

    let v_sa: f64 = B000
        + xs * (B100 + xs * (B200 + xs * (B300 + xs * (B400 + B500 * xs))))
        + ys * (B010
            + xs * (B110 + xs * (B210 + xs * (B310 + B410 * xs)))
            + ys * (B020
                + xs * (B120 + xs * (B220 + B320 * xs))
                + ys * (B030 + xs * (B130 + B230 * xs) + ys * (B040 + B140 * xs + B050 * ys))))
        + z * (B001
            + xs * (B101 + xs * (B201 + xs * (B301 + B401 * xs)))
            + ys * (B011
                + xs * (B111 + xs * (B211 + B311 * xs))
                + ys * (B021 + xs * (B121 + B221 * xs) + ys * (B031 + B131 * xs + B041 * ys)))
            + z * (B002
                + xs * (B102 + xs * (B202 + B302 * xs))
                + ys * (B012 + xs * (B112 + B212 * xs) + ys * (B022 + B122 * xs + B032 * ys))
                + z * (B003 + B103 * xs + B013 * ys + B004 * z)));

    Ok(-v_sa * 0.5 * GSW_SFAC / (specvol(sa, ct, p)? * xs))
}

/// in-situ density, thermal expansion & saline contraction coefficients
/// (75-term polynomial approximation)
///
/// # Arguments
///
/// * `sa`: Absolute Salinity \[ g kg-1 \]
/// * `ct`: Conservative Temperature (ITS-90) \[ deg C \]
/// * `p`: sea pressure \[ dbar \] (i.e. absolute pressure - 10.1325 dbar)
///
pub fn rho_alpha_beta(sa: f64, ct: f64, p: f64) -> Result<(f64, f64, f64)> {
    let rho = rho(sa, ct, p)?;
    let alpha = alpha(sa, ct, p)?;
    let beta = beta(sa, ct, p)?;

    Ok((rho, alpha, beta))
}

///
/// # Arguments
///
/// * `sa`: Absolute Salinity \[ g kg-1 \]
/// * `ct`: Conservative Temperature (ITS-90) \[ deg C \]
/// * `p`: sea pressure \[ dbar \] (i.e. absolute pressure - 10.1325 dbar)
///
pub fn specvol_alpha_beta(sa: f64, ct: f64, p: f64) -> Result<(f64, f64, f64)> {
    let specvol = specvol(sa, ct, p)?;
    let alpha = alpha(sa, ct, p)?;
    let beta = beta(sa, ct, p)?;

    Ok((specvol, alpha, beta))
}

/// Specific volume of sea water (75-term polynomial approximation)
///
/// Calculates specific volume from Absolute Salinity, Conservative
/// Temperature and pressure, using the computationally-efficient
/// polynomial expression for specific volume (Roquet et al., 2014).
///
/// # Arguments
///
/// * `sa`: Absolute Salinity \[g kg-1\]
/// * `ct`: Conservative Temperature (ITS-90) \[deg C\]
/// * `p`: sea pressure \[dbar\] (i.e. absolute pressure - 10.1325 dbar)
///
/// # Returns
///
/// * `specvol`: specific volume \[m3 kg-1\]
///
/// Note that the coefficients v(i,j,k) follow the convention in the original
/// paper, which is different from the convention used in the C-library.
///
pub fn specvol(sa: f64, ct: f64, p: f64) -> Result<f64> {
    let xs: f64 = non_dimensional_sa(sa)?;
    let ys: f64 = ct / GSW_CTU;
    let z: f64 = non_dimensional_p(p);

    // Specific Volume
    Ok(V000
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
                    + z * (V004 + xs * V104 + ys * V014 + z * (V005 + z * V006))))))
}

#[cfg(test)]
mod test_specvol {
    use super::specvol;

    #[test]
    // NaN input results in NaN output.
    // Other libraries using GSW-rs might rely on this behavior to propagate
    // and handle invalid elements.
    fn nan() {
        let v = specvol(f64::NAN, 1.0, 1.0);
        assert!(v.unwrap().is_nan());

        let v = specvol(1.0, f64::NAN, 1.0);
        assert!(v.unwrap().is_nan());

        let v = specvol(1.0, 1.0, f64::NAN);
        assert!(v.unwrap().is_nan());
    }

    #[test]
    // Test value from Roquet 2015 Appendix C.3, rounded to 9.732819628e-04
    fn roquet2015() {
        assert!((specvol(30., 10., 1000.0).unwrap() - 9.732819628e-04).abs() <= 5e-14);
    }

    #[allow(clippy::excessive_precision)]
    #[test]
    fn test_specvol() {
        if cfg!(feature = "compat") {
            // Test value from C library.
            assert!(
                (specvol(34.507499465692057, 27.994827331978655, 0.0).unwrap()
                    - 0.00097855432330275953)
                    .abs()
                    < f64::EPSILON
            );
        }
    }
}

/// Specific volume anomaly (75-term polynomial approximation)
///
/// # Arguments
///
/// * `sa`: Absolute Salinity \[ g kg-1 \]
/// * `ct`: Conservative Temperature (ITS-90) \[ deg C \]
/// * `p`: sea pressure \[ dbar \] (i.e. absolute pressure - 10.1325 dbar)
/// * `sa_ref`:
/// * `ct_ref`:
///
fn specvol_anom(sa: f64, ct: f64, p: f64, sa_ref: Option<f64>, ct_ref: Option<f64>) -> Result<f64> {
    let xs: f64 = non_dimensional_sa(sa)?;
    let ys: f64 = ct / GSW_CTU;
    let z: f64 = non_dimensional_p(p);

    unimplemented!();

    // let xs_ref: f64 = non_dimensional_sa(sa_ref)?;
    // let ys: f64 = ct_ref / GSW_CTU;
}

/// Specific Volume Anomaly of Standard Ocean Salinity and CT=0
///
/// Specific volume anomaly with reference of SA = SSO & CT = 0 (75-term equation)
///
/// # Arguments
///
/// * `sa`: Absolute Salinity \[g kg-1\]
/// * `ct`: Conservative Temperature (ITS-90) \[deg C\]
/// * `p`: sea pressure \[dbar\] (i.e. absolute pressure - 10.1325 dbar)
///
/// # Returns
///
/// * `specvol_anom`: specific volume anomaly of seawater [m3 kg-1]
///
pub fn specvol_anom_standard(sa: f64, ct: f64, p: f64) -> Result<f64> {
    Ok(specvol(sa, ct, p)? - crate::gsw_internal_funcs::specvol_sso_0(p))
}

/// Potential density anomaly with reference to sea pressure of 0 dbar
/// (75-term polynomial approximation)
///
/// # Arguments
///
/// * `sa`: Absolute Salinity \[g kg-1\]
/// * `ct`: Conservative Temperature (ITS-90) \[deg C\]
/// * `p`: sea pressure \[dbar\] (i.e. absolute pressure - 10.1325 dbar)
pub fn sigma0(sa: f64, ct: f64) -> Result<f64> {
    let xs: f64 = non_dimensional_sa(sa)?;
    let ys: f64 = ct / GSW_CTU;

    // Specific Volume
    let v = V000
        + xs * (V100 + xs * (V200 + xs * (V300 + xs * (V400 + xs * (V500 + xs * V600)))))
        + ys * (V010
            + xs * (V110 + xs * (V210 + xs * (V310 + xs * (V410 + xs * V510))))
            + ys * (V020
                + xs * (V120 + xs * (V220 + xs * (V320 + xs * V420)))
                + ys * (V030
                    + xs * (V130 + xs * (V230 + xs * V330))
                    + ys * (V040
                        + xs * (V140 + xs * V240)
                        + ys * (V050 + xs * V150 + ys * V060)))));

    Ok(1.0 / v - 1000.0)
}

/// Potential density anomaly with reference to sea pressure of 1000 dbar
/// (75-term polynomial approximation)
///
/// # Arguments
///
/// * `sa`: Absolute Salinity \[g kg-1\]
/// * `ct`: Conservative Temperature (ITS-90) \[deg C\]
/// * `p`: sea pressure \[dbar\] (i.e. absolute pressure - 10.1325 dbar)
pub fn sigma1(sa: f64, ct: f64) -> Result<f64> {
    Ok(rho(sa, ct, 1000.0)? - 1000.0)
}

/// Potential density anomaly with reference to sea pressure of 2000 dbar
/// (75-term polynomial approximation)
///
/// # Arguments
///
/// * `sa`: Absolute Salinity \[g kg-1\]
/// * `ct`: Conservative Temperature (ITS-90) \[deg C\]
/// * `p`: sea pressure \[dbar\] (i.e. absolute pressure - 10.1325 dbar)
pub fn sigma2(sa: f64, ct: f64) -> Result<f64> {
    Ok(rho(sa, ct, 2000.0)? - 1000.0)
}

/// Potential density anomaly with reference to sea pressure of 3000 dbar
/// (75-term polynomial approximation)
///
/// # Arguments
///
/// * `sa`: Absolute Salinity \[g kg-1\]
/// * `ct`: Conservative Temperature (ITS-90) \[deg C\]
/// * `p`: sea pressure \[dbar\] (i.e. absolute pressure - 10.1325 dbar)
pub fn sigma3(sa: f64, ct: f64) -> Result<f64> {
    Ok(rho(sa, ct, 3000.0)? - 1000.0)
}

/// Potential density anomaly with reference to sea pressure of 4000 dbar
/// (75-term polynomial approximation)
///
/// # Arguments
///
/// * `sa`: Absolute Salinity \[g kg-1\]
/// * `ct`: Conservative Temperature (ITS-90) \[deg C\]
/// * `p`: sea pressure \[dbar\] (i.e. absolute pressure - 10.1325 dbar)
pub fn sigma4(sa: f64, ct: f64) -> Result<f64> {
    Ok(rho(sa, ct, 4000.0)? - 1000.0)
}

/// Sound speed in seawater (75-term polynomial approximation)
///
/// # Arguments
///
/// * `sa`: Absolute Salinity \[g kg-1\]
/// * `ct`: Conservative Temperature (ITS-90) \[deg C\]
/// * `p`: sea pressure \[dbar\] (i.e. absolute pressure - 10.1325 dbar)
///
/// # Returns
///
/// * `c`: Sound speed \[m s-1\]
///
/// # Notes
///
/// * Pressure is implicitly converted to Pa to obtain speed in m s-1;
///
/// # References
///
/// * IOC, SCOR and IAPSO, 2010: The international thermodynamic equation of
///   seawater - 2010: Calculation and use of thermodynamic properties.
///   Intergovernmental Oceanographic Commission, Manuals and Guides No. 56,
///   UNESCO (English), 196 pp.  Available from <http://www.TEOS-10.org>
///   See Appendix K of this TEOS-10 Manual.
///
/// * McDougall, T.J., D.R. Jackett, D.G. Wright and R. Feistel, 2003:
///   Accurate and computationally efficient algorithms for potential
///   temperature and density of seawater.  J. Atmosph. Ocean. Tech., 20,
///   pp. 730-741.
///
/// * Roquet, F., G. Madec, T.J. McDougall, P.M. Barker, 2015: Accurate
///   polynomial expressions for the density and specifc volume of seawater
///   using the TEOS-10 standard. Ocean Modelling., 90, pp. 29-43.
///
pub fn sound_speed(sa: f64, ct: f64, p: f64) -> Result<f64> {
    let xs: f64 = non_dimensional_sa(sa)?;
    let ys: f64 = ct / GSW_CTU;
    //
    let z: f64 = non_dimensional_p(p);

    // Specific Volume
    let v = V000
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

    let v_p = C000
        + xs * (C100 + xs * (C200 + xs * (C300 + xs * (C400 + C500 * xs))))
        + ys * (C010
            + xs * (C110 + xs * (C210 + xs * (C310 + C410 * xs)))
            + ys * (C020
                + xs * (C120 + xs * (C220 + C320 * xs))
                + ys * (C030 + xs * (C130 + C230 * xs) + ys * (C040 + C140 * xs + C050 * ys))))
        + z * (C001
            + xs * (C101 + xs * (C201 + xs * (C301 + C401 * xs)))
            + ys * (C011
                + xs * (C111 + xs * (C211 + C311 * xs))
                + ys * (C021 + xs * (C121 + C221 * xs) + ys * (C031 + C131 * xs + C041 * ys)))
            + z * (C002
                + xs * (C102 + C202 * xs)
                + ys * (C012 + C112 * xs + C022 * ys)
                + z * (C003 + C103 * xs + C013 * ys + z * (C004 + C005 * z))));

    Ok(10_000.0 * libm::sqrt(-v * v / v_p))
}

///
/// # Arguments
///
/// * `sa`: Absolute Salinity \[g kg-1\]
/// * `ct`: Conservative Temperature (ITS-90) \[deg C\]
/// * `p`: sea pressure \[dbar\] (i.e. absolute pressure - 10.1325 dbar)
///
fn internal_energy(sa: f64, ct: f64, p: f64) -> Result<f64> {
    unimplemented!()
}

///
/// # Arguments
///
/// * `sa`: Absolute Salinity \[g kg-1\]
/// * `ct`: Conservative Temperature (ITS-90) \[deg C\]
/// * `p`: sea pressure \[dbar\] (i.e. absolute pressure - 10.1325 dbar)
///
fn enthalpy(sa: f64, ct: f64, p: f64) -> Result<f64> {
    unimplemented!()
}

///
/// # Arguments
///
/// * `sa`: Absolute Salinity \[g kg-1\]
/// * `ct`: Conservative Temperature (ITS-90) \[deg C\]
/// * `p`: sea pressure \[dbar\] (i.e. absolute pressure - 10.1325 dbar)
///
fn enthalpy_diff(sa: f64, ct: f64, p: f64) -> Result<f64> {
    unimplemented!()
}

/// Dynamic enthalpy of seawater (75-term polynomial approximation)
///
/// # Arguments
///
/// * `sa`: Absolute Salinity \[g kg-1\]
/// * `ct`: Conservative Temperature (ITS-90) \[deg C\]
/// * `p`: sea pressure \[dbar\] (i.e. absolute pressure - 10.1325 dbar)
///
/// # Returns
///
/// * Dynamic enthalpy \[ J kg-1 \]
///
/// # References
///
/// # Example:
/// ```
/// use gsw::volume::dynamic_enthalpy;
/// let h_hat = dynamic_enthalpy(32.0, 10.0, 100.0).unwrap();
/// assert!((h_hat - 975.8669302190772).abs() <= f64::EPSILON);
/// ```
pub fn dynamic_enthalpy(sa: f64, ct: f64, p: f64) -> Result<f64> {
    let xs: f64 = non_dimensional_sa(sa)?;
    let ys: f64 = ct / GSW_CTU;
    let z: f64 = non_dimensional_p(p);

    let dynamic_enthalpy_part = z
        * (H001
            + xs * (H101 + xs * (H201 + xs * (H301 + xs * (H401 + xs * (H501 + H601 * xs)))))
            + ys * (H011
                + xs * (H111 + xs * (H211 + xs * (H311 + xs * (H411 + H511 * xs))))
                + ys * (H021
                    + xs * (H121 + xs * (H221 + xs * (H321 + H421 * xs)))
                    + ys * (H031
                        + xs * (H131 + xs * (H231 + H331 * xs))
                        + ys * (H041
                            + xs * (H141 + H241 * xs)
                            + ys * (H051 + H151 * xs + H061 * ys)))))
            + z * (H002
                + xs * (H102 + xs * (H202 + xs * (H302 + xs * (H402 + H502 * xs))))
                + ys * (H012
                    + xs * (H112 + xs * (H212 + xs * (H312 + H412 * xs)))
                    + ys * (H022
                        + xs * (H122 + xs * (H222 + H322 * xs))
                        + ys * (H032
                            + xs * (H132 + H232 * xs)
                            + ys * (H042 + H142 * xs + H052 * ys))))
                + z * (H003
                    + xs * (H103 + xs * (H203 + xs * (H303 + H403 * xs)))
                    + ys * (H013
                        + xs * (H113 + xs * (H213 + H313 * xs))
                        + ys * (H023
                            + xs * (H123 + H223 * xs)
                            + ys * (H033 + H133 * xs + H043 * ys)))
                    + z * (H004
                        + xs * (H104 + H204 * xs)
                        + ys * (H014 + H114 * xs + H024 * ys)
                        + z * (H005 + H105 * xs + H015 * ys + z * (H006 + H007 * z))))));

    Ok(dynamic_enthalpy_part * DB2PA * 1e4)
}

#[cfg(test)]
mod test_dynamic_enthalpy {
    use super::dynamic_enthalpy;

    #[test]
    // NaN input results in NaN output.
    // Other libraries using GSW-rs might rely on this behavior to propagate
    // and handle invalid elements.
    fn nan() {
        let h_hat = dynamic_enthalpy(f64::NAN, 1.0, 1.0);
        assert!(h_hat.unwrap().is_nan());

        let h_hat = dynamic_enthalpy(1.0, f64::NAN, 1.0);
        assert!(h_hat.unwrap().is_nan());

        let h_hat = dynamic_enthalpy(1.0, 1.0, f64::NAN);
        assert!(h_hat.unwrap().is_nan());
    }
}

/// Absolute salinity of seawater from given density, Conservative
/// Temperature, and pressure.
///
/// # Notes:
///
/// - According to the Matlab GSW toolbox, after two iterations of this
/// modified Newton-Raphson iteration, the error in SA is no larger than
/// 8e-13 g/kg, which is machine precision for this calculation.
pub fn sa_from_rho(rho: f64, ct: f64, p: f64) -> Result<f64> {
    let v_lab = 1.0 / rho;
    let v_0 = specvol(0.0, ct, p)?;
    let v_50 = specvol(50.0, ct, p)?;

    // First guess, a linear ratio
    let sa = 50.0 * (v_lab - v_0) / (v_50 - v_0);

    if (sa < 0.0) || (sa > 50.0) {
        if cfg!(feature = "invalidasnan") {
            return Ok(f64::NAN);
        } else {
            return Err(Error::Undefined);
        }
    }

    // First guess of dv/dSA
    let v_sa: f64 = (v_50 - v_0) / 50.0;

    // Modified Newton-Raphson iterative optimization
    for _ in 0..2 {
        let sa_old = sa;
        let delta_v = specvol(sa_old, ct, p)? - v_lab;
        // this is half way through the modified N-R method (McDougall and
        // Wotherspoon, 2012, appud Matlab GSW implementation)
        let sa = sa_old - delta_v / v_sa;
        let sa_mean = 0.5 * (sa + sa_old);
        let (v_sa, _, _) = specvol_first_derivatives(sa_mean, ct, p)?;
        let sa = sa_old - delta_v / v_sa;
        if (sa < 0.0) || (sa > 50.0) {
            if cfg!(feature = "invalidasnan") {
                return Ok(f64::NAN);
            } else {
                return Err(Error::Undefined);
            }
        }
    }
    Ok(sa)
}

fn ct_from_rho() {
    unimplemented!()
}

fn ct_maxdensity() {
    unimplemented!()
}

#[cfg(test)]
mod tests {
    use super::{alpha, beta, specvol, specvol_anom_standard, GSW_SSO};

    #[test]
    /// This anomaly should be zero for SSO & CT=0 at any depth
    fn test_specvol_anom_standard_at_standard() {
        let p_to_test: [f64; 5] = [0., 10., 100., 1000., 5000.];
        for p in p_to_test.iter().cloned() {
            assert!((specvol_anom_standard(GSW_SSO, 0.0, p).unwrap() - 0.0).abs() < f64::EPSILON);
        }
    }

    #[test]
    fn test_negative_sa() {
        let p_to_test: [f64; 5] = [0., 10., 100., 1000., 5000.];
        let ct_to_test: [f64; 5] = [0., 10., 20., 30., 40.];
        for p in p_to_test.iter() {
            for ct in ct_to_test.iter() {
                if cfg!(feature = "compat") {
                    // If feature compatible is activated, negative sa will be replaced by 0.0
                    assert!(
                        (specvol(-20.0, *ct, *p).unwrap() - specvol(0.0, *ct, *p).unwrap()).abs()
                            < f64::EPSILON
                    );
                    assert!(
                        (alpha(-20.0, *ct, *p).unwrap() - alpha(0.0, *ct, *p).unwrap()).abs()
                            < f64::EPSILON
                    );
                    assert!(
                        (beta(-20.0, *ct, *p).unwrap() - beta(0.0, *ct, *p).unwrap()).abs()
                            < f64::EPSILON
                    );
                } else {
                    // It should return an error if not compat

                    assert!(matches!(
                        alpha(-20.0, *ct, *p),
                        Err(crate::Error::NegativeSalinity)
                    ));
                }
            }
        }
    }
}

pub fn specvol_first_derivatives(sa: f64, ct: f64, p: f64) -> Result<(f64, f64, f64)> {
    let xs: f64 = non_dimensional_sa(sa)?;
    let ys: f64 = ct / GSW_CTU;
    let z: f64 = non_dimensional_p(p);

    let v_ct_part: f64 = A000
        + xs * (A100 + xs * (A200 + xs * (A300 + xs * (A400 + A500 * xs))))
        + ys * (A010
            + xs * (A110 + xs * (A210 + xs * (A310 + A410 * xs)))
            + ys * (A020
                + xs * (A120 + xs * (A220 + A320 * xs))
                + ys * (A030 + xs * (A130 + A230 * xs) + ys * (A040 + A140 * xs + A050 * ys))))
        + z * (A001
            + xs * (A101 + xs * (A201 + xs * (A301 + A401 * xs)))
            + ys * (A011
                + xs * (A111 + xs * (A211 + A311 * xs))
                + ys * (A021 + xs * (A121 + A221 * xs) + ys * (A031 + A131 * xs + A041 * ys)))
            + z * (A002
                + xs * (A102 + xs * (A202 + A302 * xs))
                + ys * (A012 + xs * (A112 + A212 * xs) + ys * (A022 + A122 * xs + A032 * ys))
                + z * (A003 + A103 * xs + A013 * ys + A004 * z)));

    let v_ct = 0.025 * v_ct_part;

    let v_sa_part: f64 = B000
        + xs * (B100 + xs * (B200 + xs * (B300 + xs * (B400 + B500 * xs))))
        + ys * (B010
            + xs * (B110 + xs * (B210 + xs * (B310 + B410 * xs)))
            + ys * (B020
                + xs * (B120 + xs * (B220 + B320 * xs))
                + ys * (B030 + xs * (B130 + B230 * xs) + ys * (B040 + B140 * xs + B050 * ys))))
        + z * (B001
            + xs * (B101 + xs * (B201 + xs * (B301 + B401 * xs)))
            + ys * (B011
                + xs * (B111 + xs * (B211 + B311 * xs))
                + ys * (B021 + xs * (B121 + B221 * xs) + ys * (B031 + B131 * xs + B041 * ys)))
            + z * (B002
                + xs * (B102 + xs * (B202 + B302 * xs))
                + ys * (B012 + xs * (B112 + B212 * xs) + ys * (B022 + B122 * xs + B032 * ys))
                + z * (B003 + B103 * xs + B013 * ys + B004 * z)));

    let v_sa = 0.5 * GSW_SFAC * v_sa_part / xs;

    let v_p_part = C000
        + xs * (C100 + xs * (C200 + xs * (C300 + xs * (C400 + C500 * xs))))
        + ys * (C010
            + xs * (C110 + xs * (C210 + xs * (C310 + C410 * xs)))
            + ys * (C020
                + xs * (C120 + xs * (C220 + C320 * xs))
                + ys * (C030 + xs * (C130 + C230 * xs) + ys * (C040 + C140 * xs + C050 * ys))))
        + z * (C001
            + xs * (C101 + xs * (C201 + xs * (C301 + C401 * xs)))
            + ys * (C011
                + xs * (C111 + xs * (C211 + C311 * xs))
                + ys * (C021 + xs * (C121 + C221 * xs) + ys * (C031 + C131 * xs + C041 * ys)))
            + z * (C002
                + xs * (C102 + C202 * xs)
                + ys * (C012 + C112 * xs + C022 * ys)
                + z * (C003 + C103 * xs + C013 * ys + z * (C004 + C005 * z))));

    let v_p = 1e-8 * v_p_part;

    Ok((v_sa, v_ct, v_p))
}

/*
fn specvol_first_derivatives_wrt_enthalpy(sa: f64, ct: f64, p: f64) -> (f64, f64) {
    unimplemented!()
}

fn specvol_ice(t: f64, p: f64) {
    unimplemented!()
}

fn specvol_second_derivatives(sa: f64, ct: f64, p: f64) -> (f64, f64, f64, f64, f64) {
    unimplemented!()
}

fn specvol_second_derivatives_wrt_enthalpy(sa: f64, ct: f64, p: f64) -> (f64, f64, f64) {
    unimplemented!()
}

fn specvol_diff() {
    unimplemented!()
}
fn specvol_from_pot_enthalpy_ice() {
    unimplemented!()
}
fn specvol_from_pot_enthalpy_ice_poly() {
    unimplemented!()
}
fn specvol_p_parts() {
    unimplemented!()
}
*/
