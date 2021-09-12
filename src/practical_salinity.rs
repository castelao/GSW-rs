//! Practical Salinity
//!

use crate::gsw_internal_const::*;
use crate::gsw_internal_funcs::*;
// use crate::gsw_specvol_coefficients::*;
// use crate::{Error, Result};
use crate::Result;

/// Practical Salinity from conductivity
///
/// # Example:
/// ```
/// use gsw::practical_salinity::sp_from_c;
/// let sp = sp_from_c(38.0, 10.0, 100.0).unwrap();
/// assert_eq!(sp, 34.8618423333713);
/// ```
///
/// # Notes:
///
/// - Return Ok(NaN) or an Error? Maybe a new error ivalid range?
pub fn sp_from_c(cndc: f64, t: f64, p: f64) -> Result<f64> {
    // Once other functions from the module are implemented, verify
    // constants and move outside functions for common use.
    const A0: f64 = 0.0080;
    const A1: f64 = -0.1692;
    const A2: f64 = 25.3851;
    const A3: f64 = 14.0941;
    const A4: f64 = -7.0261;
    const A5: f64 = 2.7081;

    const B0: f64 = 0.0005;
    const B1: f64 = -0.0056;
    const B2: f64 = -0.0066;
    const B3: f64 = -0.0375;
    const B4: f64 = 0.0636;
    const B5: f64 = -0.0144;

    const C0: f64 = 0.6766097;
    const C1: f64 = 2.00564e-2;
    const C2: f64 = 1.104259e-4;
    const C3: f64 = -6.9698e-7;
    const C4: f64 = 1.0031e-9;

    const D1: f64 = 3.426e-2;
    const D2: f64 = 4.464e-4;
    const D3: f64 = 4.215e-1;
    const D4: f64 = -3.107e-3;

    const E1: f64 = 2.070e-5;
    const E2: f64 = -6.370e-10;
    const E3: f64 = 3.989e-15;

    const K: f64 = 0.0162;

    let t68 = t * 1.00024;
    let ft68 = (t68 - 15.0) / (1.0 + K * (t68 - 15.0));

    /*
    The dimensionless conductivity ratio, R, is the conductivity input, C,
    divided by the present estimate of C(SP=35, t_68=15, p=0) which is
    42.9140 mS/cm (=4.29140 S/m), (Culkin and Smith, 1980).
    */

    // Matlab only. C didn't follow Matlab here.
    let r = if cfg!(feature = "compat") {
        cndc * 0.023302418791070513
    } else {
        cndc / GSW_C3515
    };

    /*rt_lc corresponds to rt as defined in the UNESCO 44 (1983) routines.*/
    let rt_lc = C0 + (C1 + (C2 + (C3 + C4 * t68) * t68) * t68) * t68;
    let rp = 1.0
        + (p * (E1 + E2 * p + E3 * p * p))
            / (1.0 + D1 * t68 + D2 * t68 * t68 + (D3 + D4 * t68) * r);
    let rt = r / (rp * rt_lc);

    if rt < 0.0 {
        return Ok(f64::NAN);
    }

    let rtx = libm::sqrt(rt);

    let mut sp = A0
        + (A1 + (A2 + (A3 + (A4 + A5 * rtx) * rtx) * rtx) * rtx) * rtx
        + ft68 * (B0 + (B1 + (B2 + (B3 + (B4 + B5 * rtx) * rtx) * rtx) * rtx) * rtx);
    /*
    The following section of the code is designed for SP < 2 based on the
    Hill et al. (1986) algorithm.  This algorithm is adjusted so that it is
    exactly equal to the PSS-78 algorithm at SP = 2.
    */

    if sp < 2.0 {
        let hill_ratio = hill_ratio_at_sp2(t);
        let x = 400.0 * rt;
        let sqrty = 10.0 * rtx;
        let part1 = 1.0 + x * (1.5 + x);
        let part2 = 1.0 + sqrty * (1.0 + sqrty * (1.0 + sqrty));
        let sp_hill_raw = sp - A0 / part1 - B0 * ft68 / part2;
        sp = hill_ratio * sp_hill_raw;
    }

    /* This line ensures that SP is non-negative. */
    if sp < 0.0 {
        sp = f64::NAN;
    }

    return Ok(sp);
}

#[cfg(test)]
mod tests {
    use super::sp_from_c;

    #[test]
    fn sp_from_c_negative_rt() {
        let sp = sp_from_c(-1.0, 10.0, 100.0).unwrap();
        assert!(sp.is_nan());
    }
}
