//! Practical Salinity Constants
//!

// Constants from Appendix E.1
pub(crate) const A0: f64 = 0.0080;
pub(crate) const A1: f64 = -0.1692;
pub(crate) const A2: f64 = 25.3851;
pub(crate) const A3: f64 = 14.0941;
pub(crate) const A4: f64 = -7.0261;
pub(crate) const A5: f64 = 2.7081;

pub(crate) const B0: f64 = 0.0005;
pub(crate) const B1: f64 = -0.0056;
pub(crate) const B2: f64 = -0.0066;
pub(crate) const B3: f64 = -0.0375;
pub(crate) const B4: f64 = 0.0636;
pub(crate) const B5: f64 = -0.0144;

pub(crate) const C0: f64 = 0.6766097;
pub(crate) const C1: f64 = 2.00564e-2;
pub(crate) const C2: f64 = 1.104259e-4;
pub(crate) const C3: f64 = -6.9698e-7;
pub(crate) const C4: f64 = 1.0031e-9;

pub(crate) const D1: f64 = 3.426e-2;
pub(crate) const D2: f64 = 4.464e-4;
pub(crate) const D3: f64 = 4.215e-1;
pub(crate) const D4: f64 = -3.107e-3;

pub(crate) const E1: f64 = 2.070e-5;
pub(crate) const E2: f64 = -6.370e-10;
pub(crate) const E3: f64 = 3.989e-15;

pub(crate) const K: f64 = 0.0162;

pub(crate) const P0: f64 = 4.577801212923119e-3;
pub(crate) const P1: f64 = 1.924049429136640e-1;
pub(crate) const P2: f64 = 2.183871685127932e-5;
pub(crate) const P3: f64 = -7.292156330457999e-3;
pub(crate) const P4: f64 = 1.568129536470258e-4;
pub(crate) const P5: f64 = -1.478995271680869e-6;
pub(crate) const P6: f64 = 9.086442524716395e-4;
pub(crate) const P7: f64 = -1.949560839540487e-5;
pub(crate) const P8: f64 = -3.223058111118377e-6;
pub(crate) const P9: f64 = 1.175871639741131e-7;
pub(crate) const P10: f64 = -7.522895856600089e-5;
pub(crate) const P11: f64 = -2.254458513439107e-6;
pub(crate) const P12: f64 = 6.179992190192848e-7;
pub(crate) const P13: f64 = 1.005054226996868e-8;
pub(crate) const P14: f64 = -1.923745566122602e-9;
pub(crate) const P15: f64 = 2.259550611212616e-6;
pub(crate) const P16: f64 = 1.631749165091437e-7;
pub(crate) const P17: f64 = -5.931857989915256e-9;
pub(crate) const P18: f64 = -4.693392029005252e-9;
pub(crate) const P19: f64 = 2.571854839274148e-10;
pub(crate) const P20: f64 = 4.198786822861038e-12;

pub(crate) const Q0: f64 = 5.540896868127855e-5;
pub(crate) const Q1: f64 = 2.015419291097848e-1;
pub(crate) const Q2: f64 = -1.445310045430192e-5;
pub(crate) const Q3: f64 = -1.567047628411722e-2;
pub(crate) const Q4: f64 = 2.464756294660119e-4;
pub(crate) const Q5: f64 = -2.575458304732166e-7;
pub(crate) const Q6: f64 = 5.071449842454419e-3;
pub(crate) const Q7: f64 = -9.081985795339206e-5;
pub(crate) const Q8: f64 = -3.635420818812898e-6;
pub(crate) const Q9: f64 = 2.249490528450555e-8;
pub(crate) const Q10: f64 = -1.143810377431888e-3;
pub(crate) const Q11: f64 = 2.066112484281530e-5;
pub(crate) const Q12: f64 = 7.482907137737503e-7;
pub(crate) const Q13: f64 = 4.019321577844724e-8;
pub(crate) const Q14: f64 = -5.755568141370501e-10;
pub(crate) const Q15: f64 = 1.120748754429459e-4;
pub(crate) const Q16: f64 = -2.420274029674485e-6;
pub(crate) const Q17: f64 = -4.774829347564670e-8;
pub(crate) const Q18: f64 = -4.279037686797859e-9;
pub(crate) const Q19: f64 = -2.045829202713288e-10;
pub(crate) const Q20: f64 = 5.025109163112005e-12;

/*
pub(crate) const R0: f64 = 3.432285006604888e-3;
pub(crate) const R1: f64 = 1.672940491817403e-1;
pub(crate) const R2: f64 = 2.640304401023995e-5;
pub(crate) const R3: f64 = 1.082267090441036e-1;
pub(crate) const R4: f64 = -6.296778883666940e-5;
pub(crate) const R5: f64 = -4.542775152303671e-7;
pub(crate) const R6: f64 = -1.859711038699727e-1;
pub(crate) const R7: f64 = 7.659006320303959e-4;
pub(crate) const R8: f64 = -4.794661268817618e-7;
pub(crate) const R9: f64 = 8.093368602891911e-9;
pub(crate) const R10: f64 = 1.001140606840692e-1;
pub(crate) const R11: f64 = -1.038712945546608e-3;
pub(crate) const R12: f64 = -6.227915160991074e-6;
pub(crate) const R13: f64 = 2.798564479737090e-8;
pub(crate) const R14: f64 = -1.343623657549961e-10;
pub(crate) const R15: f64 = 1.024345179842964e-2;
pub(crate) const R16: f64 = 4.981135430579384e-4;
pub(crate) const R17: f64 = 4.466087528793912e-6;
pub(crate) const R18: f64 = 1.960872795577774e-8;
pub(crate) const R19: f64 = -2.723159418888634e-10;
pub(crate) const R20: f64 = 1.122200786423241e-12;
*/

pub(crate) const S0: f64 = 3.432285006604888e-3;
pub(crate) const S1: f64 = 1.672940491817403e-1;
pub(crate) const S2: f64 = 2.640304401023995e-5;
pub(crate) const S3: f64 = 1.082267090441036e-1;
pub(crate) const S4: f64 = -6.296778883666940e-5;
pub(crate) const S5: f64 = -4.542775152303671e-7;
pub(crate) const S6: f64 = -1.859711038699727e-1;
pub(crate) const S7: f64 = 7.659006320303959e-4;
pub(crate) const S8: f64 = -4.794661268817618e-7;
pub(crate) const S9: f64 = 8.093368602891911e-9;
pub(crate) const S10: f64 = 1.001140606840692e-1;
pub(crate) const S11: f64 = -1.038712945546608e-3;
pub(crate) const S12: f64 = -6.227915160991074e-6;
pub(crate) const S13: f64 = 2.798564479737090e-8;
pub(crate) const S14: f64 = -1.343623657549961e-10;
pub(crate) const S15: f64 = 1.024345179842964e-2;
pub(crate) const S16: f64 = 4.981135430579384e-4;
pub(crate) const S17: f64 = 4.466087528793912e-6;
pub(crate) const S18: f64 = 1.960872795577774e-8;
pub(crate) const S19: f64 = -2.723159418888634e-10;
pub(crate) const S20: f64 = 1.122200786423241e-12;

pub(crate) const U0: f64 = 5.180529787390576e-3;
pub(crate) const U1: f64 = 1.052097167201052e-3;
pub(crate) const U2: f64 = 3.666193708310848e-5;
pub(crate) const U3: f64 = 7.112223828976632;
pub(crate) const U4: f64 = -3.631366777096209e-4;
pub(crate) const U5: f64 = -7.336295318742821e-7;
pub(crate) const U6: f64 = -1.576886793288888e+2;
pub(crate) const U7: f64 = -1.840239113483083e-3;
pub(crate) const U8: f64 = 8.624279120240952e-6;
pub(crate) const U9: f64 = 1.233529799729501e-8;
pub(crate) const U10: f64 = 1.826482800939545e+3;
pub(crate) const U11: f64 = 1.633903983457674e-1;
pub(crate) const U12: f64 = -9.201096427222349e-5;
pub(crate) const U13: f64 = -9.187900959754842e-8;
pub(crate) const U14: f64 = -1.442010369809705e-10;
pub(crate) const U15: f64 = -8.542357182595853e+3;
pub(crate) const U16: f64 = -1.408635241899082;
pub(crate) const U17: f64 = 1.660164829963661e-4;
pub(crate) const U18: f64 = 6.797409608973845e-7;
pub(crate) const U19: f64 = 3.345074990451475e-10;
pub(crate) const U20: f64 = 8.285687652694768e-13;
