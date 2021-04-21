/// Conversions
///
/// Other conversions between temperatures, salinities, entropy, pressure and height

/*
other conversions between temperatures, salinities, entropy, pressure and height
gsw_deltaSA_from_SP
gsw_SA_Sstar_from_SP
gsw_SR_from_SP
gsw_SP_from_SR
gsw_SP_from_SA
gsw_Sstar_from_SA
gsw_SA_from_Sstar
gsw_SP_from_Sstar
gsw_pt_from_CT
gsw_t_from_CT
gsw_CT_from_pt
gsw_pot_enthalpy_from_pt
gsw_pt_from_t
gsw_pt0_from_t
gsw_t_from_pt0
*/
/// ITS-90 temperature from IPTS-48 temperature
///
pub fn t90_from_t48(t48: f64) -> f64 {
    (t48 - (4.4e-6) * t48 * (100. - t48)) / 1.00024
}

/// ITS-90 temperature from IPTS-68 temperature
///
pub fn t90_from_t68(t68: f64) -> f64 {
    if cfg!(feature = "compat") {
        t68 * 0.999760057586179
    } else {
        t68 / 1.00024
    }
}

/*
gsw_z_from_p
gsw_p_from_z
gsw_z_from_depth
gsw_depth_from_z
gsw_Abs_Pressure_from_p
gsw_p_from_Abs_Pressure
gsw_entropy_from_CT
gsw_CT_from_entropy
gsw_entropy_from_pt
gsw_pt_from_entropy
gsw_entropy_from_t
gsw_t_from_entropy
gsw_adiabatic_lapse_rate_from_CT
gsw_adiabatic_lapse_rate_from_t
gsw_molality_from_SA
gsw_ionic_strength_from_SA
*/
