#![allow(unused)]

extern crate std;

/* error return from gsw_saar et al. */
pub const GSW_INVALID_VALUE: f64 = 9e15;
pub const GSW_ERROR_LIMIT: f64 = 1e10;

pub const INTERP_METHOD_LINEAR: u8 = 1;
pub const INTERP_METHOD_PCHIP: u8 = 2;

/////////////////////////
// Already implemented
/////////////////////////

#[no_mangle]
pub unsafe extern "C" fn gsw_specvol(sa: f64, ct: f64, p: f64) -> f64 {
    crate::volume::specvol(sa, ct, p).unwrap_or(GSW_INVALID_VALUE)
}

#[no_mangle]
pub unsafe extern "C" fn gsw_alpha(sa: f64, ct: f64, p: f64) -> f64 {
    crate::volume::alpha(sa, ct, p).unwrap_or(GSW_INVALID_VALUE)
}

#[no_mangle]
pub unsafe extern "C" fn gsw_beta(sa: f64, ct: f64, p: f64) -> f64 {
    crate::volume::beta(sa, ct, p).unwrap_or(GSW_INVALID_VALUE)
}

#[no_mangle]
pub unsafe extern "C" fn gsw_specvol_sso_0(p: f64) -> f64 {
    crate::volume::specvol_sso_0(p)
}

#[no_mangle]
pub unsafe extern "C" fn gsw_specvol_anom_standard(sa: f64, ct: f64, p: f64) -> f64 {
    crate::volume::specvol_anom_standard(sa, ct, p).unwrap_or(GSW_INVALID_VALUE)
}

#[no_mangle]
pub unsafe extern "C" fn gsw_specvol_alpha_beta(
    sa: f64,
    ct: f64,
    p: f64,
    specvol: *mut f64,
    alpha: *mut f64,
    beta: *mut f64,
) {
    let (s, a, b) = crate::volume::specvol_alpha_beta(sa, ct, p).unwrap_or((
        GSW_INVALID_VALUE,
        GSW_INVALID_VALUE,
        GSW_INVALID_VALUE,
    ));
    *specvol = s;
    *alpha = a;
    *beta = b;
}

#[no_mangle]
pub unsafe extern "C" fn gsw_rho(sa: f64, ct: f64, p: f64) -> f64 {
    crate::volume::rho(sa, ct, p).unwrap_or(GSW_INVALID_VALUE)
}

#[no_mangle]
pub unsafe extern "C" fn gsw_rho_alpha_beta(
    sa: f64,
    ct: f64,
    p: f64,
    rho: *mut f64,
    alpha: *mut f64,
    beta: *mut f64,
) {
    let (r, a, b) = crate::volume::rho_alpha_beta(sa, ct, p).unwrap_or((
        GSW_INVALID_VALUE,
        GSW_INVALID_VALUE,
        GSW_INVALID_VALUE,
    ));

    *rho = r;
    *alpha = a;
    *beta = b;
}

#[no_mangle]
pub unsafe extern "C" fn gsw_sigma0(sa: f64, ct: f64) -> f64 {
    crate::volume::sigma0(sa, ct).unwrap_or(GSW_INVALID_VALUE)
}

#[no_mangle]
pub unsafe extern "C" fn gsw_sigma1(sa: f64, ct: f64) -> f64 {
    crate::volume::sigma1(sa, ct).unwrap_or(GSW_INVALID_VALUE)
}

#[no_mangle]
pub unsafe extern "C" fn gsw_sigma2(sa: f64, ct: f64) -> f64 {
    crate::volume::sigma2(sa, ct).unwrap_or(GSW_INVALID_VALUE)
}

#[no_mangle]
pub unsafe extern "C" fn gsw_sigma3(sa: f64, ct: f64) -> f64 {
    crate::volume::sigma3(sa, ct).unwrap_or(GSW_INVALID_VALUE)
}

#[no_mangle]
pub unsafe extern "C" fn gsw_sigma4(sa: f64, ct: f64) -> f64 {
    crate::volume::sigma4(sa, ct).unwrap_or(GSW_INVALID_VALUE)
}

#[no_mangle]
pub unsafe extern "C" fn gsw_sound_speed(sa: f64, ct: f64, p: f64) -> f64 {
    crate::volume::sound_speed(sa, ct, p).unwrap_or(GSW_INVALID_VALUE)
}

#[no_mangle]
pub unsafe extern "C" fn gsw_dynamic_enthalpy(sa: f64, ct: f64, p: f64) -> f64 {
    crate::volume::dynamic_enthalpy(sa, ct, p).unwrap_or(GSW_INVALID_VALUE)
}

#[no_mangle]
pub unsafe extern "C" fn gsw_sa_from_rho(rho: f64, ct: f64, p: f64) -> f64 {
    crate::volume::sa_from_rho(rho, ct, p).unwrap_or(GSW_INVALID_VALUE)
}

#[no_mangle]
pub unsafe extern "C" fn gsw_specvol_first_derivatives(
    sa: f64,
    ct: f64,
    p: f64,
    v_sa: *mut f64,
    v_ct: *mut f64,
    v_p: *mut f64,
) {
    let (s, c, p) = crate::volume::specvol_first_derivatives(sa, ct, p).unwrap_or((
        GSW_INVALID_VALUE,
        GSW_INVALID_VALUE,
        GSW_INVALID_VALUE,
    ));

    *v_sa = s;
    *v_ct = c;
    *v_p = p;
}

#[no_mangle]
pub unsafe extern "C" fn gsw_sp_from_c(c: f64, t: f64, p: f64) -> f64 {
    crate::practical_salinity::sp_from_c(c, t, p).unwrap_or(GSW_INVALID_VALUE)
}

#[no_mangle]
pub unsafe extern "C" fn gsw_c_from_sp(sp: f64, t: f64, p: f64) -> f64 {
    crate::practical_salinity::c_from_sp(sp, t, p).unwrap_or(GSW_INVALID_VALUE)
}

#[no_mangle]
pub unsafe extern "C" fn gsw_sp_salinometer(rt: f64, t: f64) -> f64 {
    crate::practical_salinity::sp_salinometer(rt, t).unwrap_or(GSW_INVALID_VALUE)
}

#[no_mangle]
pub unsafe extern "C" fn gsw_z_from_p(
    p: f64,
    lat: f64,
    geo_strf_dyn_height: f64,
    sea_surface_geopotential: f64,
) -> f64 {
    crate::conversions::z_from_p(p, lat, geo_strf_dyn_height, sea_surface_geopotential)
}

/////////////////////////
// To be implemented
/////////////////////////

#[no_mangle]
pub unsafe extern "C" fn gsw_add_barrier(
    input_data: *mut f64,
    lon: f64,
    lat: f64,
    long_grid: f64,
    lat_grid: f64,
    dlong_grid: f64,
    dlat_grid: f64,
    output_data: *mut f64,
) {
    //unimplemented!()
}

#[no_mangle]
pub unsafe extern "C" fn gsw_add_mean(data_in: *mut f64, data_out: *mut f64) {
    //unimplemented!()
}

#[no_mangle]
pub unsafe extern "C" fn gsw_adiabatic_lapse_rate_from_ct(sa: f64, ct: f64, p: f64) -> f64 {
    //unimplemented!()
    f64::NAN
}

#[no_mangle]
pub unsafe extern "C" fn gsw_adiabatic_lapse_rate_ice(t: f64, p: f64) -> f64 {
    //unimplemented!()
    f64::NAN
}

#[no_mangle]
pub unsafe extern "C" fn gsw_alpha_on_beta(sa: f64, ct: f64, p: f64) -> f64 {
    //unimplemented!()
    f64::NAN
}

#[no_mangle]
pub unsafe extern "C" fn gsw_alpha_wrt_t_exact(sa: f64, t: f64, p: f64) -> f64 {
    //unimplemented!()
    f64::NAN
}

#[no_mangle]
pub unsafe extern "C" fn gsw_alpha_wrt_t_ice(t: f64, p: f64) -> f64 {
    //unimplemented!()
    f64::NAN
}

#[no_mangle]
pub unsafe extern "C" fn gsw_beta_const_t_exact(sa: f64, t: f64, p: f64) -> f64 {
    //unimplemented!()
    f64::NAN
}

#[no_mangle]
pub unsafe extern "C" fn gsw_cabbeling(sa: f64, ct: f64, p: f64) -> f64 {
    //unimplemented!()
    f64::NAN
}

#[no_mangle]
pub unsafe extern "C" fn gsw_chem_potential_water_ice(t: f64, p: f64) -> f64 {
    //unimplemented!()
    f64::NAN
}

#[no_mangle]
pub unsafe extern "C" fn gsw_chem_potential_water_t_exact(sa: f64, t: f64, p: f64) -> f64 {
    //unimplemented!()
    f64::NAN
}

#[no_mangle]
pub unsafe extern "C" fn gsw_cp_ice(t: f64, p: f64) -> f64 {
    //unimplemented!()
    f64::NAN
}

#[no_mangle]
pub unsafe extern "C" fn gsw_cp_t_exact(sa: f64, t: f64, p: f64) -> f64 {
    //unimplemented!()
    f64::NAN
}

#[no_mangle]
pub unsafe extern "C" fn gsw_ct_first_derivatives(
    sa: f64,
    pt: f64,
    ct_sa: *mut f64,
    ct_pt: *mut f64,
) {
    //unimplemented!()
}

#[no_mangle]
pub unsafe extern "C" fn gsw_ct_first_derivatives_wrt_t_exact(
    sa: f64,
    t: f64,
    p: f64,
    ct_sa_wrt_t: *mut f64,
    ct_t_wrt_t: *mut f64,
    ct_p_wrt_t: *mut f64,
) {
    //unimplemented!()
}

#[no_mangle]
pub unsafe extern "C" fn gsw_ct_freezing(sa: f64, p: f64, saturation_fraction: f64) -> f64 {
    //unimplemented!()
    f64::NAN
}

#[no_mangle]
pub unsafe extern "C" fn gsw_ct_freezing_first_derivatives(
    sa: f64,
    p: f64,
    saturation_fraction: f64,
    ctfreezing_sa: *mut f64,
    ctfreezing_p: *mut f64,
) {
    //unimplemented!()
}

#[no_mangle]
pub unsafe extern "C" fn gsw_ct_freezing_first_derivatives_poly(
    sa: f64,
    p: f64,
    saturation_fraction: f64,
    ctfreezing_sa: *mut f64,
    ctfreezing_p: *mut f64,
) {
    //unimplemented!()
}

#[no_mangle]
pub unsafe extern "C" fn gsw_ct_freezing_poly(sa: f64, p: f64, saturation_fraction: f64) -> f64 {
    //unimplemented!()
    f64::NAN
}

#[no_mangle]
pub unsafe extern "C" fn gsw_ct_from_enthalpy(sa: f64, h: f64, p: f64) -> f64 {
    //unimplemented!()
    f64::NAN
}

#[no_mangle]
pub unsafe extern "C" fn gsw_ct_from_enthalpy_exact(sa: f64, h: f64, p: f64) -> f64 {
    //unimplemented!()
    f64::NAN
}

#[no_mangle]
pub unsafe extern "C" fn gsw_ct_from_entropy(sa: f64, entropy: f64) -> f64 {
    //unimplemented!()
    f64::NAN
}

#[no_mangle]
pub unsafe extern "C" fn gsw_ct_from_pt(sa: f64, pt: f64) -> f64 {
    //unimplemented!()
    f64::NAN
}

#[no_mangle]
pub unsafe extern "C" fn gsw_ct_from_rho(
    rho: f64,
    sa: f64,
    p: f64,
    ct: *mut f64,
    ct_multiple: *mut f64,
) {
    //unimplemented!()
}

#[no_mangle]
pub unsafe extern "C" fn gsw_ct_from_t(sa: f64, t: f64, p: f64) -> f64 {
    //unimplemented!()
    f64::NAN
}

#[no_mangle]
pub unsafe extern "C" fn gsw_ct_maxdensity(sa: f64, p: f64) -> f64 {
    //unimplemented!()
    f64::NAN
}

#[no_mangle]
pub unsafe extern "C" fn gsw_ct_second_derivatives(
    sa: f64,
    pt: f64,
    ct_sa_sa: *mut f64,
    ct_sa_pt: *mut f64,
    ct_pt_pt: *mut f64,
) {
    //unimplemented!()
}

#[no_mangle]
pub unsafe extern "C" fn gsw_deltasa_atlas(p: f64, lon: f64, lat: f64) -> f64 {
    //unimplemented!()
    f64::NAN
}

#[no_mangle]
pub unsafe extern "C" fn gsw_deltasa_from_sp(sp: f64, p: f64, lon: f64, lat: f64) -> f64 {
    //unimplemented!()
    f64::NAN
}

#[no_mangle]
pub unsafe extern "C" fn gsw_dilution_coefficient_t_exact(sa: f64, t: f64, p: f64) -> f64 {
    //unimplemented!()
    f64::NAN
}

#[no_mangle]
pub unsafe extern "C" fn gsw_enthalpy_ct_exact(sa: f64, ct: f64, p: f64) -> f64 {
    //unimplemented!()
    f64::NAN
}

#[no_mangle]
pub unsafe extern "C" fn gsw_enthalpy_diff(sa: f64, ct: f64, p_shallow: f64, p_deep: f64) -> f64 {
    //unimplemented!()
    f64::NAN
}

#[no_mangle]
pub unsafe extern "C" fn gsw_enthalpy(sa: f64, ct: f64, p: f64) -> f64 {
    //unimplemented!()
    f64::NAN
}

#[no_mangle]
pub unsafe extern "C" fn gsw_enthalpy_first_derivatives_ct_exact(
    sa: f64,
    ct: f64,
    p: f64,
    h_sa: *mut f64,
    h_ct: *mut f64,
) {
    //unimplemented!()
}

#[no_mangle]
pub unsafe extern "C" fn gsw_enthalpy_first_derivatives(
    sa: f64,
    ct: f64,
    p: f64,
    h_sa: *mut f64,
    h_ct: *mut f64,
) {
    //unimplemented!()
}

#[no_mangle]
pub unsafe extern "C" fn gsw_enthalpy_ice(t: f64, p: f64) -> f64 {
    //unimplemented!()
    f64::NAN
}

#[no_mangle]
pub unsafe extern "C" fn gsw_enthalpy_second_derivatives_ct_exact(
    sa: f64,
    ct: f64,
    p: f64,
    h_sa_sa: *mut f64,
    h_sa_ct: *mut f64,
    h_ct_ct: *mut f64,
) {
    //unimplemented!()
}

#[no_mangle]
pub unsafe extern "C" fn gsw_enthalpy_second_derivatives(
    sa: f64,
    ct: f64,
    p: f64,
    h_sa_sa: *mut f64,
    h_sa_ct: *mut f64,
    h_ct_ct: *mut f64,
) {
    //unimplemented!()
}

#[no_mangle]
pub unsafe extern "C" fn gsw_enthalpy_sso_0(p: f64) -> f64 {
    //unimplemented!()
    f64::NAN
}

#[no_mangle]
pub unsafe extern "C" fn gsw_enthalpy_t_exact(sa: f64, t: f64, p: f64) -> f64 {
    //unimplemented!()
    f64::NAN
}

#[no_mangle]
pub unsafe extern "C" fn gsw_entropy_first_derivatives(
    sa: f64,
    ct: f64,
    eta_sa: *mut f64,
    eta_ct: *mut f64,
) {
    //unimplemented!()
}

#[no_mangle]
pub unsafe extern "C" fn gsw_entropy_from_ct(sa: f64, ct: f64) -> f64 {
    //unimplemented!()
    f64::NAN
}

#[no_mangle]
pub unsafe extern "C" fn gsw_entropy_from_pt(sa: f64, pt: f64) -> f64 {
    //unimplemented!()
    f64::NAN
}

#[no_mangle]
pub unsafe extern "C" fn gsw_entropy_from_t(sa: f64, t: f64, p: f64) -> f64 {
    //unimplemented!()
    f64::NAN
}

#[no_mangle]
pub unsafe extern "C" fn gsw_entropy_ice(t: f64, p: f64) -> f64 {
    //unimplemented!()
    f64::NAN
}

#[no_mangle]
pub unsafe extern "C" fn gsw_entropy_part(sa: f64, t: f64, p: f64) -> f64 {
    //unimplemented!()
    f64::NAN
}

#[no_mangle]
pub unsafe extern "C" fn gsw_entropy_part_zerop(sa: f64, pt0: f64) -> f64 {
    //unimplemented!()
    f64::NAN
}

#[no_mangle]
pub unsafe extern "C" fn gsw_entropy_second_derivatives(
    sa: f64,
    ct: f64,
    eta_sa_sa: *mut f64,
    eta_sa_ct: *mut f64,
    eta_ct_ct: *mut f64,
) {
    //unimplemented!()
}

#[no_mangle]
pub unsafe extern "C" fn gsw_fdelta(p: f64, lon: f64, lat: f64) -> f64 {
    //unimplemented!()
    f64::NAN
}

#[no_mangle]
pub unsafe extern "C" fn gsw_frazil_properties(
    sa_bulk: f64,
    h_bulk: f64,
    p: f64,
    sa_final: *mut f64,
    ct_final: *mut f64,
    w_ih_final: *mut f64,
) {
    //unimplemented!()
}

#[no_mangle]
pub unsafe extern "C" fn gsw_frazil_properties_potential(
    sa_bulk: f64,
    h_pot_bulk: f64,
    p: f64,
    sa_final: *mut f64,
    ct_final: *mut f64,
    w_ih_final: *mut f64,
) {
    //unimplemented!()
}

#[no_mangle]
pub unsafe extern "C" fn gsw_frazil_properties_potential_poly(
    sa_bulk: f64,
    h_pot_bulk: f64,
    p: f64,
    sa_final: *mut f64,
    ct_final: *mut f64,
    w_ih_final: *mut f64,
) {
    //unimplemented!()
}

#[no_mangle]
pub unsafe extern "C" fn gsw_frazil_ratios_adiabatic(
    sa: f64,
    p: f64,
    w_ih: f64,
    dsa_dct_frazil: *mut f64,
    dsa_dp_frazil: *mut f64,
    dct_dp_frazil: *mut f64,
) {
    //unimplemented!()
}

#[no_mangle]
pub unsafe extern "C" fn gsw_frazil_ratios_adiabatic_poly(
    sa: f64,
    p: f64,
    w_ih: f64,
    dsa_dct_frazil: *mut f64,
    dsa_dp_frazil: *mut f64,
    dct_dp_frazil: *mut f64,
) {
    //unimplemented!()
}

#[no_mangle]
pub unsafe extern "C" fn gsw_geo_strf_dyn_height(
    sa: *mut f64,
    ct: *mut f64,
    p: *mut f64,
    p_ref: f64,
    n_levels: ::libc::c_int,
    dyn_height: *mut f64,
) -> *mut f64 {
    //unimplemented
    std::ptr::null_mut::<f64>()
}

#[no_mangle]
pub unsafe extern "C" fn gsw_geo_strf_dyn_height_1(
    sa: *mut f64,
    ct: *mut f64,
    p: *mut f64,
    p_ref: f64,
    n_levels: ::libc::c_int,
    dyn_height: *mut f64,
    max_dp_i: f64,
    interp_method: ::libc::c_int,
) -> ::libc::c_int {
    //unimplemented!()
    0
}

#[no_mangle]
pub unsafe extern "C" fn gsw_geo_strf_dyn_height_pc(
    sa: *mut f64,
    ct: *mut f64,
    delta_p: *mut f64,
    n_levels: ::libc::c_int,
    geo_strf_dyn_height_pc: *mut f64,
    p_mid: *mut f64,
) -> *mut f64 {
    //unimplemented!()
    std::ptr::null_mut::<f64>()
}

#[no_mangle]
pub unsafe extern "C" fn gsw_gibbs_ice(
    nt: ::libc::c_int,
    np: ::libc::c_int,
    t: f64,
    p: f64,
) -> f64 {
    //unimplemented!()
    f64::NAN
}

#[no_mangle]
pub unsafe extern "C" fn gsw_gibbs_ice_part_t(t: f64, p: f64) -> f64 {
    //unimplemented!()
    f64::NAN
}

#[no_mangle]
pub unsafe extern "C" fn gsw_gibbs_ice_pt0(pt0: f64) -> f64 {
    //unimplemented!()
    f64::NAN
}

#[no_mangle]
pub unsafe extern "C" fn gsw_gibbs_ice_pt0_pt0(pt0: f64) -> f64 {
    //unimplemented!()
    f64::NAN
}

#[no_mangle]
pub unsafe extern "C" fn gsw_gibbs(
    ns: ::libc::c_int,
    nt: ::libc::c_int,
    np: ::libc::c_int,
    sa: f64,
    t: f64,
    p: f64,
) -> f64 {
    //unimplemented!()
    f64::NAN
}

#[no_mangle]
pub unsafe extern "C" fn gsw_gibbs_pt0_pt0(sa: f64, pt0: f64) -> f64 {
    //unimplemented!()
    f64::NAN
}

#[no_mangle]
pub unsafe extern "C" fn gsw_grav(lat: f64, p: f64) -> f64 {
    //unimplemented!()
    f64::NAN
}

#[no_mangle]
pub unsafe extern "C" fn gsw_helmholtz_energy_ice(t: f64, p: f64) -> f64 {
    //unimplemented!()
    f64::NAN
}

#[no_mangle]
pub unsafe extern "C" fn gsw_hill_ratio_at_sp2(t: f64) -> f64 {
    //unimplemented!()
    f64::NAN
}

#[no_mangle]
pub unsafe extern "C" fn gsw_ice_fraction_to_freeze_seawater(
    sa: f64,
    ct: f64,
    p: f64,
    t_ih: f64,
    sa_freeze: *mut f64,
    ct_freeze: *mut f64,
    w_ih: *mut f64,
) {
    //unimplemented!()
}

#[no_mangle]
pub unsafe extern "C" fn gsw_internal_energy(sa: f64, ct: f64, p: f64) -> f64 {
    //unimplemented!()
    f64::NAN
}

#[no_mangle]
pub unsafe extern "C" fn gsw_internal_energy_ice(t: f64, p: f64) -> f64 {
    //unimplemented!()
    f64::NAN
}

#[no_mangle]
pub unsafe extern "C" fn gsw_ipv_vs_fnsquared_ratio(
    sa: *mut f64,
    ct: *mut f64,
    p: *mut f64,
    p_ref: f64,
    nz: ::libc::c_int,
    ipv_vs_fnsquared_ratio: *mut f64,
    p_mid: *mut f64,
) {
    //unimplemented!()
}

#[no_mangle]
pub unsafe extern "C" fn gsw_kappa_const_t_ice(t: f64, p: f64) -> f64 {
    //unimplemented!()
    f64::NAN
}

#[no_mangle]
pub unsafe extern "C" fn gsw_kappa(sa: f64, ct: f64, p: f64) -> f64 {
    //unimplemented!()
    f64::NAN
}

#[no_mangle]
pub unsafe extern "C" fn gsw_kappa_ice(t: f64, p: f64) -> f64 {
    //unimplemented!()
    f64::NAN
}

#[no_mangle]
pub unsafe extern "C" fn gsw_kappa_t_exact(sa: f64, t: f64, p: f64) -> f64 {
    //unimplemented!()
    f64::NAN
}

#[no_mangle]
pub unsafe extern "C" fn gsw_latentheat_evap_ct(sa: f64, ct: f64) -> f64 {
    //unimplemented!()
    f64::NAN
}

#[no_mangle]
pub unsafe extern "C" fn gsw_latentheat_evap_t(sa: f64, t: f64) -> f64 {
    //unimplemented!()
    f64::NAN
}

#[no_mangle]
pub unsafe extern "C" fn gsw_latentheat_melting(sa: f64, p: f64) -> f64 {
    //unimplemented!()
    f64::NAN
}

#[no_mangle]
pub unsafe extern "C" fn gsw_linear_interp_sa_ct(
    sa: *mut f64,
    ct: *mut f64,
    p: *mut f64,
    np: ::libc::c_int,
    p_i: *mut f64,
    npi: ::libc::c_int,
    sa_i: *mut f64,
    ct_i: *mut f64,
) {
    //unimplemented!()
}

#[no_mangle]
pub unsafe extern "C" fn gsw_melting_ice_equilibrium_sa_ct_ratio(sa: f64, p: f64) -> f64 {
    //unimplemented!()
    f64::NAN
}

#[no_mangle]
pub unsafe extern "C" fn gsw_melting_ice_equilibrium_sa_ct_ratio_poly(sa: f64, p: f64) -> f64 {
    //unimplemented!()
    f64::NAN
}

#[no_mangle]
pub unsafe extern "C" fn gsw_melting_ice_into_seawater(
    sa: f64,
    ct: f64,
    p: f64,
    w_ih: f64,
    t_ih: f64,
    sa_final: *mut f64,
    ct_final: *mut f64,
    w_ih_final: *mut f64,
) {
    //unimplemented!()
}

#[no_mangle]
pub unsafe extern "C" fn gsw_melting_ice_sa_ct_ratio(sa: f64, ct: f64, p: f64, t_ih: f64) -> f64 {
    //unimplemented!()
    f64::NAN
}

#[no_mangle]
pub unsafe extern "C" fn gsw_melting_ice_sa_ct_ratio_poly(
    sa: f64,
    ct: f64,
    p: f64,
    t_ih: f64,
) -> f64 {
    //unimplemented!()
    f64::NAN
}

#[no_mangle]
pub unsafe extern "C" fn gsw_melting_seaice_equilibrium_sa_ct_ratio(sa: f64, p: f64) -> f64 {
    //unimplemented!()
    f64::NAN
}

#[no_mangle]
pub unsafe extern "C" fn gsw_melting_seaice_equilibrium_sa_ct_ratio_poly(sa: f64, p: f64) -> f64 {
    //unimplemented!()
    f64::NAN
}

#[no_mangle]
pub unsafe extern "C" fn gsw_melting_seaice_into_seawater(
    sa: f64,
    ct: f64,
    p: f64,
    w_seaice: f64,
    sa_seaice: f64,
    t_seaice: f64,
    sa_final: *mut f64,
    ct_final: *mut f64,
) {
    //unimplemented!()
}

#[no_mangle]
pub unsafe extern "C" fn gsw_melting_seaice_sa_ct_ratio(
    sa: f64,
    ct: f64,
    p: f64,
    sa_seaice: f64,
    t_seaice: f64,
) -> f64 {
    //unimplemented!()
    f64::NAN
}

#[no_mangle]
pub unsafe extern "C" fn gsw_melting_seaice_sa_ct_ratio_poly(
    sa: f64,
    ct: f64,
    p: f64,
    sa_seaice: f64,
    t_seaice: f64,
) -> f64 {
    //unimplemented!()
    f64::NAN
}

#[no_mangle]
pub unsafe extern "C" fn gsw_nsquared(
    sa: *mut f64,
    ct: *mut f64,
    p: *mut f64,
    lat: *mut f64,
    nz: ::libc::c_int,
    n2: *mut f64,
    p_mid: *mut f64,
) {
    //unimplemented!()
}

#[no_mangle]
pub unsafe extern "C" fn gsw_o2sol(sa: f64, ct: f64, p: f64, lon: f64, lat: f64) -> f64 {
    //unimplemented!()
    f64::NAN
}

#[no_mangle]
pub unsafe extern "C" fn gsw_o2sol_sp_pt(sp: f64, pt: f64) -> f64 {
    //unimplemented!()
    f64::NAN
}

#[no_mangle]
pub unsafe extern "C" fn gsw_pot_enthalpy_from_pt_ice(pt0_ice: f64) -> f64 {
    //unimplemented!()
    f64::NAN
}

#[no_mangle]
pub unsafe extern "C" fn gsw_pot_enthalpy_from_pt_ice_poly(pt0_ice: f64) -> f64 {
    //unimplemented!()
    f64::NAN
}

#[no_mangle]
pub unsafe extern "C" fn gsw_pot_enthalpy_ice_freezing(sa: f64, p: f64) -> f64 {
    //unimplemented!()
    f64::NAN
}

#[no_mangle]
pub unsafe extern "C" fn gsw_pot_enthalpy_ice_freezing_first_derivatives(
    sa: f64,
    p: f64,
    pot_enthalpy_ice_freezing_sa: *mut f64,
    pot_enthalpy_ice_freezing_p: *mut f64,
) {
    //unimplemented!()
}

#[no_mangle]
pub unsafe extern "C" fn gsw_pot_enthalpy_ice_freezing_first_derivatives_poly(
    sa: f64,
    p: f64,
    pot_enthalpy_ice_freezing_sa: *mut f64,
    pot_enthalpy_ice_freezing_p: *mut f64,
) {
    //unimplemented!()
}

#[no_mangle]
pub unsafe extern "C" fn gsw_pot_enthalpy_ice_freezing_poly(sa: f64, p: f64) -> f64 {
    //unimplemented!()
    f64::NAN
}

#[no_mangle]
pub unsafe extern "C" fn gsw_pot_rho_t_exact(sa: f64, t: f64, p: f64, p_ref: f64) -> f64 {
    //unimplemented!()
    f64::NAN
}

#[no_mangle]
pub unsafe extern "C" fn gsw_pressure_coefficient_ice(t: f64, p: f64) -> f64 {
    //unimplemented!()
    f64::NAN
}

#[no_mangle]
pub unsafe extern "C" fn gsw_pressure_freezing_ct(
    sa: f64,
    ct: f64,
    saturation_fraction: f64,
) -> f64 {
    //unimplemented!()
    f64::NAN
}

#[no_mangle]
pub unsafe extern "C" fn gsw_pt0_cold_ice_poly(pot_enthalpy_ice: f64) -> f64 {
    //unimplemented!()
    f64::NAN
}

#[no_mangle]
pub unsafe extern "C" fn gsw_pt0_from_t(sa: f64, t: f64, p: f64) -> f64 {
    //unimplemented!()
    f64::NAN
}

#[no_mangle]
pub unsafe extern "C" fn gsw_pt0_from_t_ice(t: f64, p: f64) -> f64 {
    //unimplemented!()
    f64::NAN
}

#[no_mangle]
pub unsafe extern "C" fn gsw_pt_first_derivatives(
    sa: f64,
    ct: f64,
    pt_sa: *mut f64,
    pt_ct: *mut f64,
) {
    //unimplemented!()
}

#[no_mangle]
pub unsafe extern "C" fn gsw_pt_from_ct(sa: f64, ct: f64) -> f64 {
    //unimplemented!()
    f64::NAN
}

#[no_mangle]
pub unsafe extern "C" fn gsw_pt_from_entropy(sa: f64, entropy: f64) -> f64 {
    //unimplemented!()
    f64::NAN
}

#[no_mangle]
pub unsafe extern "C" fn gsw_pt_from_pot_enthalpy_ice(pot_enthalpy_ice: f64) -> f64 {
    //unimplemented!()
    f64::NAN
}

#[no_mangle]
pub unsafe extern "C" fn gsw_pt_from_pot_enthalpy_ice_poly_dh(pot_enthalpy_ice: f64) -> f64 {
    //unimplemented!()
    f64::NAN
}

#[no_mangle]
pub unsafe extern "C" fn gsw_pt_from_pot_enthalpy_ice_poly(pot_enthalpy_ice: f64) -> f64 {
    //unimplemented!()
    f64::NAN
}

#[no_mangle]
pub unsafe extern "C" fn gsw_pt_from_t(sa: f64, t: f64, p: f64, p_ref: f64) -> f64 {
    //unimplemented!()
    f64::NAN
}

#[no_mangle]
pub unsafe extern "C" fn gsw_pt_from_t_ice(t: f64, p: f64, p_ref: f64) -> f64 {
    //unimplemented!()
    f64::NAN
}

#[no_mangle]
pub unsafe extern "C" fn gsw_pt_second_derivatives(
    sa: f64,
    ct: f64,
    pt_sa_sa: *mut f64,
    pt_sa_ct: *mut f64,
    pt_ct_ct: *mut f64,
) {
    //unimplemented!()
}

#[no_mangle]
pub unsafe extern "C" fn gsw_rho_first_derivatives(
    sa: f64,
    ct: f64,
    p: f64,
    drho_dsa: *mut f64,
    drho_dct: *mut f64,
    drho_dp: *mut f64,
) {
    //unimplemented!()
}

#[no_mangle]
pub unsafe extern "C" fn gsw_rho_first_derivatives_wrt_enthalpy(
    sa: f64,
    ct: f64,
    p: f64,
    rho_sa: *mut f64,
    rho_h: *mut f64,
) {
    //unimplemented!()
}

#[no_mangle]
pub unsafe extern "C" fn gsw_rho_ice(t: f64, p: f64) -> f64 {
    //unimplemented!()
    f64::NAN
}

#[no_mangle]
pub unsafe extern "C" fn gsw_rho_second_derivatives(
    sa: f64,
    ct: f64,
    p: f64,
    rho_sa_sa: *mut f64,
    rho_sa_ct: *mut f64,
    rho_ct_ct: *mut f64,
    rho_sa_p: *mut f64,
    rho_ct_p: *mut f64,
) {
    //unimplemented!()
}

#[no_mangle]
pub unsafe extern "C" fn gsw_rho_second_derivatives_wrt_enthalpy(
    sa: f64,
    ct: f64,
    p: f64,
    rho_sa_sa: *mut f64,
    rho_sa_h: *mut f64,
    rho_h_h: *mut f64,
) {
    //unimplemented!()
}

#[no_mangle]
pub unsafe extern "C" fn gsw_rho_t_exact(sa: f64, t: f64, p: f64) -> f64 {
    //unimplemented!()
    f64::NAN
}

#[no_mangle]
pub unsafe extern "C" fn gsw_rr68_interp_sa_ct(
    sa: *mut f64,
    ct: *mut f64,
    p: *mut f64,
    mp: ::libc::c_int,
    p_i: *mut f64,
    mp_i: ::libc::c_int,
    sa_i: *mut f64,
    ct_i: *mut f64,
) {
    //unimplemented!()
}

#[no_mangle]
pub unsafe extern "C" fn gsw_saar(p: f64, lon: f64, lat: f64) -> f64 {
    //unimplemented!()
    f64::NAN
}

#[no_mangle]
pub unsafe extern "C" fn gsw_sa_freezing_estimate(
    p: f64,
    saturation_fraction: f64,
    ct: *mut f64,
    t: *mut f64,
) -> f64 {
    //unimplemented!()
    f64::NAN
}

#[no_mangle]
pub unsafe extern "C" fn gsw_sa_freezing_from_ct(ct: f64, p: f64, saturation_fraction: f64) -> f64 {
    //unimplemented!()
    f64::NAN
}

#[no_mangle]
pub unsafe extern "C" fn gsw_sa_freezing_from_ct_poly(
    ct: f64,
    p: f64,
    saturation_fraction: f64,
) -> f64 {
    //unimplemented!()
    f64::NAN
}

#[no_mangle]
pub unsafe extern "C" fn gsw_sa_freezing_from_t(t: f64, p: f64, saturation_fraction: f64) -> f64 {
    //unimplemented!()
    f64::NAN
}

#[no_mangle]
pub unsafe extern "C" fn gsw_sa_freezing_from_t_poly(
    t: f64,
    p: f64,
    saturation_fraction: f64,
) -> f64 {
    //unimplemented!()
    f64::NAN
}

#[no_mangle]
pub unsafe extern "C" fn gsw_sa_from_sp_baltic(sp: f64, lon: f64, lat: f64) -> f64 {
    //unimplemented!()
    f64::NAN
}

#[no_mangle]
pub unsafe extern "C" fn gsw_sa_from_sp(sp: f64, p: f64, lon: f64, lat: f64) -> f64 {
    //unimplemented!()
    f64::NAN
}

#[no_mangle]
pub unsafe extern "C" fn gsw_sa_from_sstar(sstar: f64, p: f64, lon: f64, lat: f64) -> f64 {
    //unimplemented!()
    f64::NAN
}

#[no_mangle]
pub unsafe extern "C" fn gsw_sa_p_inrange(sa: f64, p: f64) -> ::libc::c_int {
    //unimplemented!()
    0
}

#[no_mangle]
pub unsafe extern "C" fn gsw_seaice_fraction_to_freeze_seawater(
    sa: f64,
    ct: f64,
    p: f64,
    sa_seaice: f64,
    t_seaice: f64,
    sa_freeze: *mut f64,
    ct_freeze: *mut f64,
    w_seaice: *mut f64,
) {
    //unimplemented!()
}

#[no_mangle]
pub unsafe extern "C" fn gsw_sound_speed_ice(t: f64, p: f64) -> f64 {
    //unimplemented!()
    f64::NAN
}

#[no_mangle]
pub unsafe extern "C" fn gsw_sound_speed_t_exact(sa: f64, t: f64, p: f64) -> f64 {
    //unimplemented!()
    f64::NAN
}

#[no_mangle]
pub unsafe extern "C" fn gsw_specvol_first_derivatives_wrt_enthalpy(
    sa: f64,
    ct: f64,
    p: f64,
    v_sa: *mut f64,
    v_h: *mut f64,
) {
    //unimplemented!()
}

#[no_mangle]
pub unsafe extern "C" fn gsw_specvol_ice(t: f64, p: f64) -> f64 {
    //unimplemented!()
    f64::NAN
}

#[no_mangle]
pub unsafe extern "C" fn gsw_specvol_second_derivatives(
    sa: f64,
    ct: f64,
    p: f64,
    v_sa_sa: *mut f64,
    v_sa_ct: *mut f64,
    v_ct_ct: *mut f64,
    v_sa_p: *mut f64,
    v_ct_p: *mut f64,
) {
    //unimplemented!()
}

#[no_mangle]
pub unsafe extern "C" fn gsw_specvol_second_derivatives_wrt_enthalpy(
    sa: f64,
    ct: f64,
    p: f64,
    v_sa_sa: *mut f64,
    v_sa_h: *mut f64,
    v_h_h: *mut f64,
) {
    //unimplemented!()
}

#[no_mangle]
pub unsafe extern "C" fn gsw_specvol_t_exact(sa: f64, t: f64, p: f64) -> f64 {
    //unimplemented!()
    f64::NAN
}

#[no_mangle]
pub unsafe extern "C" fn gsw_sp_from_sa_baltic(sa: f64, lon: f64, lat: f64) -> f64 {
    //unimplemented!()
    f64::NAN
}

#[no_mangle]
pub unsafe extern "C" fn gsw_sp_from_sa(sa: f64, p: f64, lon: f64, lat: f64) -> f64 {
    //unimplemented!()
    f64::NAN
}

#[no_mangle]
pub unsafe extern "C" fn gsw_sp_from_sk(sk: f64) -> f64 {
    //unimplemented!()
    f64::NAN
}

#[no_mangle]
pub unsafe extern "C" fn gsw_sp_from_sr(sr: f64) -> f64 {
    //unimplemented!()
    f64::NAN
}

#[no_mangle]
pub unsafe extern "C" fn gsw_sp_from_sstar(sstar: f64, p: f64, lon: f64, lat: f64) -> f64 {
    //unimplemented!()
    f64::NAN
}

#[no_mangle]
pub unsafe extern "C" fn gsw_spiciness0(sa: f64, ct: f64) -> f64 {
    //unimplemented!()
    f64::NAN
}

#[no_mangle]
pub unsafe extern "C" fn gsw_spiciness1(sa: f64, ct: f64) -> f64 {
    //unimplemented!()
    f64::NAN
}

#[no_mangle]
pub unsafe extern "C" fn gsw_spiciness2(sa: f64, ct: f64) -> f64 {
    //unimplemented!()
    f64::NAN
}

#[no_mangle]
pub unsafe extern "C" fn gsw_sr_from_sp(sp: f64) -> f64 {
    //unimplemented!()
    f64::NAN
}

#[no_mangle]
pub unsafe extern "C" fn gsw_sstar_from_sa(sa: f64, p: f64, lon: f64, lat: f64) -> f64 {
    //unimplemented!()
    f64::NAN
}

#[no_mangle]
pub unsafe extern "C" fn gsw_sstar_from_sp(sp: f64, p: f64, lon: f64, lat: f64) -> f64 {
    //unimplemented!()
    f64::NAN
}

#[no_mangle]
pub unsafe extern "C" fn gsw_t_deriv_chem_potential_water_t_exact(sa: f64, t: f64, p: f64) -> f64 {
    //unimplemented!()
    f64::NAN
}

#[no_mangle]
pub unsafe extern "C" fn gsw_t_freezing(sa: f64, p: f64, saturation_fraction: f64) -> f64 {
    //unimplemented!()
    f64::NAN
}

#[no_mangle]
pub unsafe extern "C" fn gsw_t_freezing_first_derivatives_poly(
    sa: f64,
    p: f64,
    saturation_fraction: f64,
    tfreezing_sa: *mut f64,
    tfreezing_p: *mut f64,
) {
    //unimplemented!()
}

#[no_mangle]
pub unsafe extern "C" fn gsw_t_freezing_first_derivatives(
    sa: f64,
    p: f64,
    saturation_fraction: f64,
    tfreezing_sa: *mut f64,
    tfreezing_p: *mut f64,
) {
    //unimplemented!()
}

#[no_mangle]
pub unsafe extern "C" fn gsw_t_freezing_poly(sa: f64, p: f64, saturation_fraction: f64) -> f64 {
    //unimplemented!()
    f64::NAN
}

#[no_mangle]
pub unsafe extern "C" fn gsw_t_from_ct(sa: f64, ct: f64, p: f64) -> f64 {
    //unimplemented!()
    f64::NAN
}

#[no_mangle]
pub unsafe extern "C" fn gsw_t_from_pt0_ice(pt0_ice: f64, p: f64) -> f64 {
    //unimplemented!()
    f64::NAN
}

#[no_mangle]
pub unsafe extern "C" fn gsw_thermobaric(sa: f64, ct: f64, p: f64) -> f64 {
    //unimplemented!()
    f64::NAN
}

#[no_mangle]
pub unsafe extern "C" fn gsw_turner_rsubrho(
    sa: *mut f64,
    ct: *mut f64,
    p: *mut f64,
    nz: ::libc::c_int,
    tu: *mut f64,
    rsubrho: *mut f64,
    p_mid: *mut f64,
) {
    //unimplemented!()
}

#[no_mangle]
pub unsafe extern "C" fn gsw_util_indx(x: *mut f64, n: ::libc::c_int, z: f64) -> ::libc::c_int {
    //unimplemented!()
    0
}

#[no_mangle]
pub unsafe extern "C" fn gsw_util_interp1q_int(
    nx: ::libc::c_int,
    x: *mut f64,
    iy: *mut ::libc::c_int,
    nxi: ::libc::c_int,
    x_i: *mut f64,
    y_i: *mut f64,
) -> *mut f64 {
    //unimplemented!()
    std::ptr::null_mut::<f64>()
}

#[no_mangle]
pub unsafe extern "C" fn gsw_util_linear_interp(
    nx: ::libc::c_int,
    x: *mut f64,
    ny: ::libc::c_int,
    y: *mut f64,
    nxi: ::libc::c_int,
    x_i: *mut f64,
    y_i: *mut f64,
) -> *mut f64 {
    //unimplemented!()
    std::ptr::null_mut::<f64>()
}

#[no_mangle]
pub unsafe extern "C" fn gsw_util_sort_real(
    rarray: *mut f64,
    nx: ::libc::c_int,
    iarray: *mut ::libc::c_int,
) {
    //unimplemented!()
}

#[no_mangle]
pub unsafe extern "C" fn gsw_util_xinterp1(
    x: *mut f64,
    y: *mut f64,
    n: ::libc::c_int,
    x0: f64,
) -> f64 {
    //unimplemented!()
    f64::NAN
}

#[no_mangle]
pub unsafe extern "C" fn gsw_util_pchip_interp(
    x: *mut f64,
    y: *mut f64,
    n: ::libc::c_int,
    xi: *mut f64,
    yi: *mut f64,
    ni: ::libc::c_int,
) -> ::libc::c_int {
    //unimplemented!()
    0
}

#[no_mangle]
pub unsafe extern "C" fn gsw_p_from_z(
    z: f64,
    lat: f64,
    geo_strf_dyn_height: f64,
    sea_surface_geopotential: f64,
) -> f64 {
    //unimplemented!()
    f64::NAN
}

#[cfg(test)]
mod test {
    use super::std::format;

    use inline_c::assert_c;

    #[test]
    fn test_specvol_c() {
        let result: f64 = crate::volume::specvol(1., 1., 1.).unwrap();
        (assert_c! {
            #include <stdio.h>
            #include "gswteos-10.h"

            int main() {
                printf("%.15f", gsw_specvol(1., 1., 1.));

                return 0;
            }
        })
        .success()
        .stdout(format!("{:.15}", result));
    }

    #[test]
    fn test_alpha_c() {
        let result: f64 = crate::volume::alpha(1., 1., 1.).unwrap();
        (assert_c! {
            #include <stdio.h>
            #include "gswteos-10.h"

            int main() {
                printf("%.15f", gsw_alpha(1., 1., 1.));

                return 0;
            }
        })
        .success()
        .stdout(format!("{:.15}", result));
    }

    #[test]
    fn test_beta_c() {
        let result: f64 = crate::volume::beta(1., 1., 1.).unwrap();
        (assert_c! {
            #include <stdio.h>
            #include "gswteos-10.h"

            int main() {
                printf("%.15f", gsw_beta(1., 1., 1.));

                return 0;
            }
        })
        .success()
        .stdout(format!("{:.15}", result));
    }

    #[test]
    fn test_specvol_sso_0_c() {
        let result: f64 = crate::volume::specvol_sso_0(1.);
        (assert_c! {
            #include <stdio.h>
            #include "gswteos-10.h"

            int main() {
                printf("%.15f", gsw_specvol_sso_0(1.));

                return 0;
            }
        })
        .success()
        .stdout(format!("{:.15}", result));
    }

    #[test]
    fn test_specvol_anom_standard_c() {
        let result: f64 = crate::volume::specvol_anom_standard(1., 1., 1.).unwrap();
        (assert_c! {
            #include <stdio.h>
            #include "gswteos-10.h"

            int main() {
                printf("%.15f", gsw_specvol_anom_standard(1., 1., 1.));

                return 0;
            }
        })
        .success()
        .stdout(format!("{:.15}", result));
    }

    #[test]
    fn test_specvol_alpha_beta_c() {
        let result = crate::volume::specvol_alpha_beta(1., 1., 1.).unwrap();
        (assert_c! {
            #include <stdio.h>
            #include "gswteos-10.h"

            int main() {
                double specvol, alpha, beta;
                gsw_specvol_alpha_beta(1., 1., 1., &specvol, &alpha, &beta);
                printf("%.15f %.15f %.15f", specvol, alpha, beta);

                return 0;
            }
        })
        .success()
        .stdout(format!(
            "{:.15} {:.15} {:.15}",
            result.0, result.1, result.2
        ));
    }

    #[test]
    fn test_rho_c() {
        let result: f64 = crate::volume::rho(1., 1., 1.).unwrap();
        (assert_c! {
            #include <stdio.h>
            #include "gswteos-10.h"

            int main() {
                printf("%.15f", gsw_rho(1., 1., 1.));

                return 0;
            }
        })
        .success()
        .stdout(format!("{:.15}", result));
    }
}
