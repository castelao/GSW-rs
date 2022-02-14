//! Practical Salinity
//!

use crate::gsw_internal_const::*;
use crate::gsw_internal_funcs::*;
use crate::gsw_sp_coefficients::*;
use crate::{Error, Result};

fn t68_from_t90(t90: f64) -> f64 {
    t90 * 1.00024
}

/// Practical Salinity from conductivity
///
/// # Arguments
///
/// * `cndc`: conductivity \[ mS/cm \]
/// * `t90`: in-situ temperature (ITS-90) \[ deg C \]
/// * `p`: sea pressure \[ dbar \] (i.e. absolute pressure - 10.1325 dbar)
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
pub fn sp_from_c(cndc: f64, t90: f64, p: f64) -> Result<f64> {
    // The dimensionless conductivity ratio, R, is the conductivity input, C,
    // divided by the present estimate of C(SP=35, t_68=15, p=0) which is
    // 42.9140 mS/cm (=4.29140 S/m^).
    // Matlab only. C didn't follow Matlab here.
    let r = if cfg!(feature = "compat") {
        cndc * 0.023302418791070513
    } else {
        cndc / GSW_C3515
    };

    sp_from_r(r, t90, p)
}

#[cfg(test)]
mod test_sp_from_c {
    use super::{sp_from_c, Error};

    #[test]
    fn zero_cndc() {
        let sp = sp_from_c(0.0, 10.0, 100.0).unwrap();
        assert!((sp - 0.0).abs() <= f64::EPSILON);
    }

    #[test]
    // NaN input results in NaN output.
    // Other libraries using GSW-rs might rely on this behavior to propagate
    // and handle invalid elements.
    fn nan() {
        let sp = sp_from_c(f64::NAN, 1.0, 1.0);
        assert!(sp.unwrap().is_nan());

        let sp = sp_from_c(1.0, f64::NAN, 1.0);
        assert!(sp.unwrap().is_nan());

        let sp = sp_from_c(1.0, 1.0, f64::NAN);
        assert!(sp.unwrap().is_nan());
    }

    #[test]
    // MatLab returns NaN if Rt < 0
    fn negative_cndc() {
        let sp = sp_from_c(-0.1, 10.0, 100.0);

        if cfg!(feature = "compat") {
            assert!(sp.unwrap().is_nan());
            // If rt is > 0, and S_p end up negative, Matlab forces it to zero
            // assert_eq!(sp, Ok(0.0));
        } else {
            match sp {
                // rt < 0
                Err(Error::Undefined) => (),
                // S_p < 0
                Err(Error::NegativeSalinity) => (),
                _ => assert!(false),
            }
        }
    }
}

/// Conductivity from Practical Salinity
///
/// # Arguments
///
/// * `sp`: practical salinity (PSS-78) \[ unitless \]
/// * `t90`: in-situ temperature (ITS-90) \[ deg C \]
/// * `p`: sea pressure \[ dbar \] (i.e. absolute pressure - 10.1325 dbar)
///
///
/// # Example:
/// ```
/// use gsw::practical_salinity::c_from_sp;
/// let cndc = c_from_sp(34.86, 10.0, 100.0).unwrap();
/// assert_eq!(cndc, 37.99819884763376);
/// ```
///
/// # Notes
/// * Practical Salinity is limited between 0 and 42 as defined in the
///   references
pub fn c_from_sp(sp: f64, t90: f64, p: f64) -> Result<f64> {
    // The dimensionless conductivity ratio, R, is the conductivity input, C,
    // divided by the present estimate of C(SP=35, t_68=15, p=0) which is
    // 42.9140 mS/cm (=4.29140 S/m^).
    Ok(GSW_C3515 * r_from_sp(sp, t90, p)?)
}

#[cfg(test)]
mod test_c_from_sp {
    use super::{c_from_sp, Error};

    #[test]
    fn zero_sp() {
        let cndc = c_from_sp(0.0, 0.0, 0.0).unwrap();
        assert!((cndc - 0.000779962392516606).abs() <= f64::EPSILON);
    }

    #[test]
    // Matlab also returns error if S_p < 0.0, thus standard as well as compat
    // returns error.
    fn negative_sp() {
        let cndc = c_from_sp(-0.1, 10.0, 100.0);
        match cndc {
            Err(Error::NegativeSalinity) => (),
            _ => assert!(false),
        }
    }

    #[test]
    #[cfg(not(feature = "compat"))]
    // Matlab does not check for upper limit, thus ignore this test if compiled
    // with "compat".
    fn overlimit_sp() {
        let cndc = c_from_sp(42.1, 10.0, 100.0);
        match cndc {
            Err(Error::Undefined) => (),
            _ => assert!(false),
        }
    }
}

/// Practical Salinity from conductivity ratio
///
/// # Arguments
///
/// * `r`: conductivity ratio \[ unitless \]
/// * `t90`: in-situ temperature (ITS-90) \[ deg C \]
/// * `p`: sea pressure \[ dbar \] (i.e. absolute pressure - 10.1325 dbar)
///
/// # Example:
/// ```
/// use gsw::practical_salinity::sp_from_r;
/// let sp = sp_from_r(1.0, 15.0, 100.0).unwrap();
/// assert_eq!(sp, 34.95619860613106);
/// ```
pub fn sp_from_r(r: f64, t90: f64, p: f64) -> Result<f64> {
    let t68 = t68_from_t90(t90);
    let ft68 = (t68 - 15.0) / (1.0 + K * (t68 - 15.0));

    // rt_lc corresponds to rt as defined in the UNESCO 44 (1983) routines.
    let rt_lc = C0 + (C1 + (C2 + (C3 + C4 * t68) * t68) * t68) * t68;
    let rp = 1.0
        + (p * (E1 + E2 * p + E3 * p * p))
            / (1.0 + D1 * t68 + D2 * t68 * t68 + (D3 + D4 * t68) * r);
    let rt = r / (rp * rt_lc);

    // if rt < 0, Matlab returns NaN and C returns GSW_INVALID_VALUE
    if rt < 0.0 {
        if cfg!(feature = "invalidasnan") {
            return Ok(f64::NAN);
        } else {
            return Err(Error::Undefined);
        }
    }

    let rtx = libm::sqrt(rt);

    let mut sp = A0
        + (A1 + (A2 + (A3 + (A4 + A5 * rtx) * rtx) * rtx) * rtx) * rtx
        + ft68 * (B0 + (B1 + (B2 + (B3 + (B4 + B5 * rtx) * rtx) * rtx) * rtx) * rtx);

    // The following section of the code is designed for SP < 2 based on the
    // Hill et al. (1986) algorithm.  This algorithm is adjusted so that it is
    // exactly equal to the PSS-78 algorithm at SP = 2.
    if sp < 2.0 {
        let hill_ratio = hill_ratio_at_sp2(t90);
        let x = 400.0 * rt;
        let sqrty = 10.0 * rtx;
        let part1 = 1.0 + x * (1.5 + x);
        let part2 = 1.0 + sqrty * (1.0 + sqrty * (1.0 + sqrty));
        let sp_hill_raw = sp - A0 / part1 - B0 * ft68 / part2;
        sp = hill_ratio * sp_hill_raw;
    }

    if sp < 0.0 {
        // MatLab forces zero if negative.
        if cfg!(feature = "compat") {
            Ok(0.0)
        // C lib returns GSW_INVALID_VALUE instead
        } else if cfg!(feature = "invalidasnan") {
            Ok(f64::NAN)
        } else {
            Err(Error::NegativeSalinity)
        }
    } else {
        Ok(sp)
    }
}

#[cfg(test)]
mod test_sp_from_r {
    use super::sp_from_r;

    #[test]
    // NaN input results in NaN output.
    // Other libraries using GSW-rs might rely on this behavior to propagate
    // and handle invalid elements.
    fn nan() {
        let sp = sp_from_r(f64::NAN, 1.0, 1.0);
        assert!(sp.unwrap().is_nan());

        let sp = sp_from_r(1.0, f64::NAN, 1.0);
        assert!(sp.unwrap().is_nan());

        let sp = sp_from_r(1.0, 1.0, f64::NAN);
        assert!(sp.unwrap().is_nan());
    }
}

#[allow(clippy::manual_range_contains)]
/// Conductivity ratio from Practical Salinity
///
/// # Arguments
///
/// * `sp`: practical salinity (PSS-78) \[ unitless \]
/// * `t90`: in-situ temperature (ITS-90) \[ deg C \]
/// * `p`: sea pressure \[ dbar \] (i.e. absolute pressure - 10.1325 dbar)
///
/// # Example:
/// ```
/// use gsw::practical_salinity::r_from_sp;
/// let ratio = r_from_sp(34.86, 10.0, 100.0).unwrap();
/// assert_eq!(ratio, 0.8854499428539347);
/// ```
pub fn r_from_sp(sp: f64, t90: f64, p: f64) -> Result<f64> {
    let t68 = t68_from_t90(t90);
    let ft68 = (t68 - 15.0) / (1.0 + K * (t68 - 15.0));

    let x = libm::sqrt(sp);

    // TEOS-10 & Hill et. al. 1986 limited range to 0-42, but all other
    // libraries ignores that.
    // MatLab returns error if SP < 0
    if sp < 0.0 {
        return Err(Error::NegativeSalinity);
    } else if !cfg!(feature = "compat") && (sp > 42.0) {
        return Err(Error::Undefined);
    }

    // Finding the starting value of Rtx, the square root of Rt, using four
    // different polynomials of SP and t68.

    let mut rtx = if sp >= 9.0 {
        P0 + x
            * (P1
                + P4 * t68
                + x * (P3 + P7 * t68 + x * (P6 + P11 * t68 + x * (P10 + P16 * t68 + x * P15))))
            + t68
                * (P2
                    + t68
                        * (P5
                            + x * x * (P12 + x * P17)
                            + P8 * x
                            + t68 * (P9 + x * (P13 + x * P18) + t68 * (P14 + P19 * x + P20 * t68))))
    } else if sp >= 0.25 && sp < 9.0 {
        Q0 + x
            * (Q1
                + Q4 * t68
                + x * (Q3 + Q7 * t68 + x * (Q6 + Q11 * t68 + x * (Q10 + Q16 * t68 + x * Q15))))
            + t68
                * (Q2
                    + t68
                        * (Q5
                            + x * x * (Q12 + x * Q17)
                            + Q8 * x
                            + t68 * (Q9 + x * (Q13 + x * Q18) + t68 * (Q14 + Q19 * x + Q20 * t68))))
    } else if sp >= 0.003 && sp < 0.25 {
        S0 + x
            * (S1
                + S4 * t68
                + x * (S3 + S7 * t68 + x * (S6 + S11 * t68 + x * (S10 + S16 * t68 + x * S15))))
            + t68
                * (S2
                    + t68
                        * (S5
                            + x * x * (S12 + x * S17)
                            + S8 * x
                            + t68 * (S9 + x * (S13 + x * S18) + t68 * (S14 + S19 * x + S20 * t68))))
    // S_p < 0.003 the only possible condition left, thus this is equivalent to
    // if sp < 0.003 {
    } else {
        U0 + x
            * (U1
                + U4 * t68
                + x * (U3 + U7 * t68 + x * (U6 + U11 * t68 + x * (U10 + U16 * t68 + x * U15))))
            + t68
                * (U2
                    + t68
                        * (U5
                            + x * x * (U12 + x * U17)
                            + U8 * x
                            + t68 * (U9 + x * (U13 + x * U18) + t68 * (U14 + U19 * x + U20 * t68))))
    };

    // Finding the starting value of dSP_dRtx, the derivative of SP with
    // respect to Rtx.
    let mut dsp_drtx = A1
        + (2.0 * A2 + (3.0 * A3 + (4.0 * A4 + 5.0 * A5 * rtx) * rtx) * rtx) * rtx
        + ft68 * (B1 + (2.0 * B2 + (3.0 * B3 + (4.0 * B4 + 5.0 * B5 * rtx) * rtx) * rtx) * rtx);

    if sp < 2.0 {
        let x = 400.0 * (rtx * rtx);
        let sqrty = 10.0 * rtx;
        let part1 = 1.0 + x * (1.5 + x);
        let part2 = 1.0 + sqrty * (1.0 + sqrty * (1.0 + sqrty));
        let hill_ratio = hill_ratio_at_sp2(t90);
        dsp_drtx = dsp_drtx
            + A0 * 800.0 * rtx * (1.5 + 2.0 * x) / (part1 * part1)
            + B0 * ft68 * (10.0 + sqrty * (20.0 + 30.0 * sqrty)) / (part2 * part2);
        dsp_drtx *= hill_ratio;
    }

    // One iteration through the modified Newton-Raphson method (McDougall and
    // Wotherspoon, 2012) achieves an error in Practical Salinity of about
    // 10^-12 for all combinations of the inputs.  One and a half iterations of
    // the modified Newton-Raphson method achieves a maximum error in terms of
    // Practical Salinity of better than 2x10^-14 everywhere.
    //
    // We recommend one and a half iterations of the modified Newton-Raphson
    // method.
    //
    // Begin the modified Newton-Raphson method.
    let mut sp_est = A0
        + (A1 + (A2 + (A3 + (A4 + A5 * rtx) * rtx) * rtx) * rtx) * rtx
        + ft68 * (B0 + (B1 + (B2 + (B3 + (B4 + B5 * rtx) * rtx) * rtx) * rtx) * rtx);
    if sp_est < 2.0 {
        let x = 400.0 * (rtx * rtx);
        let sqrty = 10.0 * rtx;
        let part1 = 1.0 + x * (1.5e0 + x);
        let part2 = 1.0 + sqrty * (1.0 + sqrty * (1.0 + sqrty));
        let sp_hill_raw = sp_est - A0 / part1 - B0 * ft68 / part2;
        let hill_ratio = hill_ratio_at_sp2(t90);
        sp_est = hill_ratio * sp_hill_raw;
    }

    let rtx_old = rtx;
    rtx = rtx_old - (sp_est - sp) / dsp_drtx;

    // This mean value of Rtx, Rtxm, is the value of Rtx at which the
    // derivative dSP_dRtx is evaluated.
    let rtxm = 0.5 * (rtx + rtx_old);

    let mut dsp_drtx = A1
        + (2.0 * A2 + (3.0 * A3 + (4.0 * A4 + 5.0 * A5 * rtxm) * rtxm) * rtxm) * rtxm
        + ft68 * (B1 + (2.0 * B2 + (3.0 * B3 + (4.0 * B4 + 5.0 * B5 * rtxm) * rtxm) * rtxm) * rtxm);
    if sp_est < 2.0 {
        let x = 400.0 * (rtxm * rtxm);
        let sqrty = 10.0 * rtxm;
        let part1 = 1.0 + x * (1.5e0 + x);
        let part2 = 1.0 + sqrty * (1.0 + sqrty * (1.0 + sqrty));
        dsp_drtx = dsp_drtx
            + A0 * 800.0 * rtxm * (1.5e0 + 2.0 * x) / (part1 * part1)
            + B0 * ft68 * (10.0 + sqrty * (20.0 + 30.0 * sqrty)) / (part2 * part2);
        let hill_ratio = hill_ratio_at_sp2(t90);
        dsp_drtx *= hill_ratio;
    }

    // The line below is where Rtx is updated at the end of the one full
    // iteration of the modified Newton-Raphson technique.
    rtx = rtx_old - (sp_est - sp) / dsp_drtx;
    // Now we do another half iteration of the modified Newton-Raphson
    // technique, making a total of one and a half modified N-R iterations.
    sp_est = A0
        + (A1 + (A2 + (A3 + (A4 + A5 * rtx) * rtx) * rtx) * rtx) * rtx
        + ft68 * (B0 + (B1 + (B2 + (B3 + (B4 + B5 * rtx) * rtx) * rtx) * rtx) * rtx);
    if sp_est < 2.0 {
        let x = 400.0 * (rtx * rtx);
        let sqrty = 10.0 * rtx;
        let part1 = 1.0 + x * (1.5e0 + x);
        let part2 = 1.0 + sqrty * (1.0 + sqrty * (1.0 + sqrty));
        let sp_hill_raw = sp_est - A0 / part1 - B0 * ft68 / part2;
        let hill_ratio = hill_ratio_at_sp2(t90);
        sp_est = hill_ratio * sp_hill_raw;
    }
    rtx -= (sp_est - sp) / dsp_drtx;

    // Now go from Rtx to Rt and then to the conductivity ratio R at pressure p.
    let rt = rtx * rtx;

    let aa = D3 + D4 * t68;
    let bb = 1.0 + t68 * (D1 + D2 * t68);
    let cc = p * (E1 + p * (E2 + E3 * p));
    // rt_lc (i.e. rt_lower_case) corresponDs to rt as DefineD in the
    // UNESCO 44 (1983) routines.
    let rt_lc = C0 + (C1 + (C2 + (C3 + C4 * t68) * t68) * t68) * t68;

    let dd = bb - aa * rt_lc * rt;
    let ee = rt_lc * rt * aa * (bb + cc);
    let ra = libm::sqrt(dd * dd + 4.0 * ee) - dd;
    let r = 0.5 * ra / aa;

    Ok(r)
}

#[cfg(test)]
mod test_r_from_sp {
    use super::r_from_sp;

    #[test]
    // NaN input results in NaN output.
    // Other libraries using GSW-rs might rely on this behavior to propagate
    // and handle invalid elements.
    fn nan() {
        let r = r_from_sp(f64::NAN, 1.0, 1.0);
        assert!(r.unwrap().is_nan());

        let r = r_from_sp(1.0, f64::NAN, 1.0);
        assert!(r.unwrap().is_nan());

        let r = r_from_sp(1.0, 1.0, f64::NAN);
        assert!(r.unwrap().is_nan());
    }
}

/// Practical Salinity from a laboratory salinometer
///
/// # Arguments
///
/// * `Rt`: C(SP, t_68, 0)/C(SP=35,  t_68, 0) \[ unitless \]
/// * `t90`: temperature (ITS-90) of the bath of the salinometer \[ deg C \]
///
/// # Example:
/// ```
/// use gsw::practical_salinity::sp_salinometer;
/// let sp = sp_salinometer(0.9, 10.0).unwrap();
/// assert_eq!(sp, 31.130296542699828);
/// ```
//
pub fn sp_salinometer(rt: f64, t90: f64) -> Result<f64> {
    let t68 = t68_from_t90(t90);
    let ft68 = (t68 - 15.0) / (1.0 + K * (t68 - 15.0));

    // if rt < 0, Matlab returns NaN
    if rt < 0.0 {
        if cfg!(feature = "invalidasnan") {
            return Ok(f64::NAN);
        } else {
            return Err(Error::Undefined);
        }
    }

    let rtx = libm::sqrt(rt);

    let mut sp = A0
        + (A1 + (A2 + (A3 + (A4 + A5 * rtx) * rtx) * rtx) * rtx) * rtx
        + ft68 * (B0 + (B1 + (B2 + (B3 + (B4 + B5 * rtx) * rtx) * rtx) * rtx) * rtx);

    // The following section of the code is designed for SP < 2 based on the
    // Hill et al. (1986) algorithm.  This algorithm is adjusted so that it is
    // exactly equal to the PSS-78 algorithm at SP = 2.
    if sp < 2.0 {
        let hill_ratio = hill_ratio_at_sp2(t90);
        let x = 400.0 * rt;
        let sqrty = 10.0 * rtx;
        let part1 = 1.0 + x * (1.5 + x);
        let part2 = 1.0 + sqrty * (1.0 + sqrty * (1.0 + sqrty));
        let sp_hill_raw = sp - A0 / part1 - B0 * ft68 / part2;
        sp = hill_ratio * sp_hill_raw;
    }

    if sp < 0.0 {
        // MatLab forces zero if negative.
        if cfg!(feature = "compat") {
            Ok(0.0)
        } else if cfg!(feature = "invalidasnan") {
            Ok(f64::NAN)
        } else {
            Err(Error::NegativeSalinity)
        }
    } else {
        Ok(sp)
    }
}

#[cfg(test)]
mod test_sp_salinometer {
    use super::{sp_salinometer, Error};

    #[test]
    // NaN input results in NaN output.
    // Other libraries using GSW-rs might rely on this behavior to propagate
    // and handle invalid elements.
    fn nan() {
        let sp = sp_salinometer(f64::NAN, 1.0);
        assert!(sp.unwrap().is_nan());

        let sp = sp_salinometer(1.0, f64::NAN);
        assert!(sp.unwrap().is_nan());
    }

    #[test]
    fn ratio_one() {
        let sp = sp_salinometer(1.0, 15.0).unwrap();
        assert!((sp - 35.00000000000001).abs() <= f64::EPSILON);
    }

    #[test]
    fn negative_rt() {
        let sp = sp_salinometer(-0.1, 15.0);

        if cfg!(feature = "compat") {
            assert!(sp.unwrap().is_nan());
            // If rt is > 0, and S_p end up negative, Matlab forces it to zero
            // assert_eq!(sp, Ok(0.0));
        } else {
            match sp {
                // rt < 0
                Err(Error::Undefined) => (),
                // S_p < 0
                Err(Error::NegativeSalinity) => (),
                _ => assert!(false),
            }
        }
    }
}
