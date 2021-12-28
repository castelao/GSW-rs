//! Practical Salinity
//!

use crate::gsw_internal_const::*;
use crate::gsw_internal_funcs::*;
// use crate::gsw_specvol_coefficients::*;
use crate::{Error, Result};

// Constants from Appendix E.1
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

const P0: f64 = 4.577801212923119e-3;
const P1: f64 = 1.924049429136640e-1;
const P2: f64 = 2.183871685127932e-5;
const P3: f64 = -7.292156330457999e-3;
const P4: f64 = 1.568129536470258e-4;
const P5: f64 = -1.478995271680869e-6;
const P6: f64 = 9.086442524716395e-4;
const P7: f64 = -1.949560839540487e-5;
const P8: f64 = -3.223058111118377e-6;
const P9: f64 = 1.175871639741131e-7;
const P10: f64 = -7.522895856600089e-5;
const P11: f64 = -2.254458513439107e-6;
const P12: f64 = 6.179992190192848e-7;
const P13: f64 = 1.005054226996868e-8;
const P14: f64 = -1.923745566122602e-9;
const P15: f64 = 2.259550611212616e-6;
const P16: f64 = 1.631749165091437e-7;
const P17: f64 = -5.931857989915256e-9;
const P18: f64 = -4.693392029005252e-9;
const P19: f64 = 2.571854839274148e-10;
const P20: f64 = 4.198786822861038e-12;

const Q0: f64 = 5.540896868127855e-5;
const Q1: f64 = 2.015419291097848e-1;
const Q2: f64 = -1.445310045430192e-5;
const Q3: f64 = -1.567047628411722e-2;
const Q4: f64 = 2.464756294660119e-4;
const Q5: f64 = -2.575458304732166e-7;
const Q6: f64 = 5.071449842454419e-3;
const Q7: f64 = -9.081985795339206e-5;
const Q8: f64 = -3.635420818812898e-6;
const Q9: f64 = 2.249490528450555e-8;
const Q10: f64 = -1.143810377431888e-3;
const Q11: f64 = 2.066112484281530e-5;
const Q12: f64 = 7.482907137737503e-7;
const Q13: f64 = 4.019321577844724e-8;
const Q14: f64 = -5.755568141370501e-10;
const Q15: f64 = 1.120748754429459e-4;
const Q16: f64 = -2.420274029674485e-6;
const Q17: f64 = -4.774829347564670e-8;
const Q18: f64 = -4.279037686797859e-9;
const Q19: f64 = -2.045829202713288e-10;
const Q20: f64 = 5.025109163112005e-12;

/*
const R0: f64 = 3.432285006604888e-3;
const R1: f64 = 1.672940491817403e-1;
const R2: f64 = 2.640304401023995e-5;
const R3: f64 = 1.082267090441036e-1;
const R4: f64 = -6.296778883666940e-5;
const R5: f64 = -4.542775152303671e-7;
const R6: f64 = -1.859711038699727e-1;
const R7: f64 = 7.659006320303959e-4;
const R8: f64 = -4.794661268817618e-7;
const R9: f64 = 8.093368602891911e-9;
const R10: f64 = 1.001140606840692e-1;
const R11: f64 = -1.038712945546608e-3;
const R12: f64 = -6.227915160991074e-6;
const R13: f64 = 2.798564479737090e-8;
const R14: f64 = -1.343623657549961e-10;
const R15: f64 = 1.024345179842964e-2;
const R16: f64 = 4.981135430579384e-4;
const R17: f64 = 4.466087528793912e-6;
const R18: f64 = 1.960872795577774e-8;
const R19: f64 = -2.723159418888634e-10;
const R20: f64 = 1.122200786423241e-12;
*/

const S0: f64 = 3.432285006604888e-3;
const S1: f64 = 1.672940491817403e-1;
const S2: f64 = 2.640304401023995e-5;
const S3: f64 = 1.082267090441036e-1;
const S4: f64 = -6.296778883666940e-5;
const S5: f64 = -4.542775152303671e-7;
const S6: f64 = -1.859711038699727e-1;
const S7: f64 = 7.659006320303959e-4;
const S8: f64 = -4.794661268817618e-7;
const S9: f64 = 8.093368602891911e-9;
const S10: f64 = 1.001140606840692e-1;
const S11: f64 = -1.038712945546608e-3;
const S12: f64 = -6.227915160991074e-6;
const S13: f64 = 2.798564479737090e-8;
const S14: f64 = -1.343623657549961e-10;
const S15: f64 = 1.024345179842964e-2;
const S16: f64 = 4.981135430579384e-4;
const S17: f64 = 4.466087528793912e-6;
const S18: f64 = 1.960872795577774e-8;
const S19: f64 = -2.723159418888634e-10;
const S20: f64 = 1.122200786423241e-12;

const U0: f64 = 5.180529787390576e-3;
const U1: f64 = 1.052097167201052e-3;
const U2: f64 = 3.666193708310848e-5;
const U3: f64 = 7.112223828976632;
const U4: f64 = -3.631366777096209e-4;
const U5: f64 = -7.336295318742821e-7;
const U6: f64 = -1.576886793288888e+2;
const U7: f64 = -1.840239113483083e-3;
const U8: f64 = 8.624279120240952e-6;
const U9: f64 = 1.233529799729501e-8;
const U10: f64 = 1.826482800939545e+3;
const U11: f64 = 1.633903983457674e-1;
const U12: f64 = -9.201096427222349e-5;
const U13: f64 = -9.187900959754842e-8;
const U14: f64 = -1.442010369809705e-10;
const U15: f64 = -8.542357182595853e+3;
const U16: f64 = -1.408635241899082;
const U17: f64 = 1.660164829963661e-4;
const U18: f64 = 6.797409608973845e-7;
const U19: f64 = 3.345074990451475e-10;
const U20: f64 = 8.285687652694768e-13;

/// Practical Salinity from conductivity
///
/// # Arguments
///
/// * `t90`: Temperature ITS-90 \[deg C\]
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
    let t68 = t90 * 1.00024;
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
        let hill_ratio = hill_ratio_at_sp2(t90);
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

///
///
/// # Arguments
///
/// * `t90`: Temperature ITS-90 \[deg C\]
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
    let t68 = t90 * 1.00024;
    let ft68 = (t68 - 15.0) / (1.0 + K * (t68 - 15.0));

    let x = libm::sqrt(sp);

    if cfg!(feature = "compat") && (sp < 0.0 || sp > 42.0) {
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
        dsp_drtx = hill_ratio * dsp_drtx;
    }

    // One iteration through the modified Newton-Raphson method (McDougall and
    // Wotherspoon, 2012) achieves an error in Practical Salinity of about
    // 10^-12 for all combinations of the inputs.  One and a half iterations of
    // the modified Newton-Raphson method achevies a maximum error in terms of
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
        dsp_drtx = hill_ratio * dsp_drtx;
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
    rtx = rtx - (sp_est - sp) / dsp_drtx;

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

    // The dimensionless conductivity ratio, R, is the conductivity input, C,
    // divided by the present estimate of C(SP=35, t_68=15, p=0) which is
    // 42.9140 mS/cm (=4.29140 S/m^).
    Ok(GSW_C3515 * r)
}

fn sp_from_r(r: f64, t90: f64, p: f64) -> Result<f64> {
    let t68 = t90 * 1.00024;
    let ft68 = (t68 - 15.0) / (1.0 + K * (t68 - 15.0));

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
        let hill_ratio = hill_ratio_at_sp2(t90);
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

    Ok(sp)
}

///
///
/// # Arguments
///
/// * `t90`: Temperature ITS-90 \[deg C\]
///
/// # Example:
/// ```
/// use gsw::practical_salinity::r_from_sp;
/// let ratio = r_from_sp(34.86, 10.0, 100.0).unwrap();
/// assert_eq!(ratio, 0.8854499428539347);
/// ```
pub fn r_from_sp(sp: f64, t90: f64, p: f64) -> Result<f64> {
    let t68 = t90 * 1.00024;
    let ft68 = (t68 - 15.0) / (1.0 + K * (t68 - 15.0));

    let x = libm::sqrt(sp);

    // TEOS-10 & Hill et. al. 1986 limited range to 0-42, but all other
    // libraries ignores that.
    if cfg!(feature = "compat") && (sp < 0.0 || sp > 42.0) {
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
        dsp_drtx = hill_ratio * dsp_drtx;
    }

    // One iteration through the modified Newton-Raphson method (McDougall and
    // Wotherspoon, 2012) achieves an error in Practical Salinity of about
    // 10^-12 for all combinations of the inputs.  One and a half iterations of
    // the modified Newton-Raphson method achevies a maximum error in terms of
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
        dsp_drtx = hill_ratio * dsp_drtx;
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
    rtx = rtx - (sp_est - sp) / dsp_drtx;

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

///
///
/// # Arguments
///
/// * `Rt`: C(SP,t_68,0)/C(SP=35,t_68,0) \[ unitless \]
///
/// # Example:
/// ```
/// use gsw::practical_salinity::sp_salinometer;
/// let sp = sp_salinometer(0.9, 10.0).unwrap();
/// assert_eq!(sp, 31.130296542699828);
/// ```
//
pub fn sp_salinometer(rt: f64, t90: f64) -> Result<f64> {
    let t68 = t90 * 1.00024;
    let ft68 = (t68 - 15.0) / (1.0 + K * (t68 - 15.0));

    if rt < 0.0 {
        return Ok(f64::NAN);
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

    /* This line ensures that SP is non-negative. */
    if sp < 0.0 {
        sp = f64::NAN;
    }

    return Ok(sp);
}
