
Currently, GSW-Rust do not implement all functions already available in
GSW-C or GSW-Matlab. This document gives an idea of what is missing. If
interested in some specific function, please check our official manual as
the best reference for what we have already done.

We organized the functions in modules following the toolbox card from
TEOS-10 main document. The functions unchecked are missing in GSW-Rust.
The goal is to implement all functions described in TEOS-10, but the
priority is to do it right.


# Conversions

- [ ] gsw_deltasa_from_sp(sp,p,lon,lat)
- [ ] gsw_sstar_from_sp(sp,p,lon,lat)
- [X] gsw_sr_from_sp(sp)
- [X] gsw_sp_from_sr(sr)
- [ ] gsw_sp_from_sa(sa,p,lon,lat)
- [ ] gsw_sstar_from_sa(sa,p,lon,lat)
- [ ] gsw_sa_from_sstar(sstar,p,lon,lat)
- [ ] gsw_sp_from_sstar(sstar,p,lon,lat)
- [ ] gsw_pt_from_ct(sa,ct)
- [ ] gsw_t_from_ct(sa,ct,p)
- [X] gsw_ct_from_pt(sa,pt)
- [ ] gsw_pt_from_t(sa,t,p,p_ref)
- [ ] gsw_pt0_from_t(sa,t,p)
- [X] gsw_z_from_p(p,lat,geo_strf_dyn_height,sea_surface_geopotential)
- [X] gsw_p_from_z(z,lat, geo_strf_dyn_height, sea_surface_geopotential)
- [ ] gsw_entropy_from_ct (sa, ct)
- [ ] gsw_ct_from_entropy (sa, entropy)
- [ ] gsw_entropy_from_pt (sa, pt)
- [ ] gsw_pt_from_entropy (sa, entropy)
- [ ] gsw_entropy_from_t(sa,t,p)
- [ ] gsw_adiabatic_lapse_rate_from_ct(sa,ct,p)



## Missing on GSW-C

- [ ] pot_enthalpy_from_pt
- [ ] t_from_pt0
- [X] t90_from_t48()
- [X] t90_from_t68()
- [ ] z_from_depth
- [ ] depth_from_z
- [X] abs_pressure_from_p()
- [X] p_from_abs_pressure()
- [ ] t_from_entropy()
- [ ] adiabatic_lapse_rate_from_t()
- [ ] molality_from_sa
- [ ] ionic_strength_from_sa
- [X] t68_from_t90()

# Earth

- [X] gsw_grav(lat,p)

## Missing on GSW-C

- [X] coriollis_parameter()
- [X] distance()

# Practical Salinity

- [X] gsw_sp_from_c(c,t,p)
- [X] gsw_c_from_sp(sp,t,p)
- [X] gsw_sp_salinometer(rt,t)
- [ ] gsw_sp_from_sk(sk)

## Missing on GSW-C

- [X] sp_from_r
- [X] r_from_sp

# Spiceness

- [ ] gsw_spiciness0(sa,ct)
- [ ] gsw_spiciness1(sa,ct)
- [ ] gsw_spiciness2(sa,ct)

## Missing on GSW-C

- [ ] gsw_SA_CT_from_sigma0_spiciness0
- [ ] gsw_SA_CT_from_sigma1_spiciness1
- [ ] gsw_SA_CT_from_sigma2_spiciness2

# Volume

- [X] gsw_specvol(sa,ct,p)
- [X] gsw_alpha(sa,ct,p)
- [X] gsw_beta(sa,ct,p)
- [X] gsw_alpha_on_beta(sa,ct,p)
- [X] gsw_specvol_anom_standard(sa,ct,p)
- [X] gsw_rho(sa,ct,p)
- [X] gsw_sigma0(sa,ct)
- [X] gsw_sigma1(sa,ct)
- [X] gsw_sigma2(sa,ct)
- [X] gsw_sigma3(sa,ct)
- [X] gsw_sigma4(sa,ct)
- [X] gsw_cabbeling(sa,ct,p)
- [X] gsw_thermobaric(sa,ct,p)
- [X] gsw_enthalpy(sa,ct,p)
- [X] gsw_enthalpy_diff (sa, ct, p_shallow, p_deep)
- [X] gsw_dynamic_enthalpy(sa,ct,p)
- [X] gsw_sound_speed(sa,ct,p)
- [X] gsw_kappa(sa,ct,p)
- [X] gsw_internal_energy(sa,ct,p)
- [ ] gsw_ct_from_enthalpy (sa, h, p)
- [X] gsw_sa_from_rho(rho,ct,p)
- [X] gsw_ct_maxdensity (sa, p)

## Missing on GSW-C

- [X] specvol_alpha_beta
- [X] specvol_first_derivatives
- [X] specvol_second_derivatives
- [X] specvol_first_derivatives_wrt_enthalpy
- [X] specvol_second_derivatives_wrt_enthalpy
- [X] specvol_anom
- [X] rho_alpha_beta
- [X] rho_first_derivatives
- [X] rho_second_derivatives
- [ ] rho_first_derivatives_wrt_enthalpy
- [ ] rho_second_derivatives_wrt_enthalpy
- [ ] enthalpy_first_derivatives
- [ ] enthalpy_second_derivatives
- [ ] internal_energy_first_derivatives
- [ ] internal_energy_second_derivatives
- [ ] ct_from_rho

# Geostrophic streamfunctions, acoustic travel time and geostrophic velocity

- [ ] gsw_geo_strf_dyn_height (sa, ct, p, p_ref)

# Volume Exact

# Derivatives

# Ice

# Vertical stability and interpolation

# Thermodynamic interaction between ice and seawater

# Absolute Salinity (SA), Preformed Salinity (Sstar) and Conservative Temperature (CT)

# Internal Functions

- [X] gsw_specvol_sso_0(p)
- [X] gsw_enthalpy_sso_0(p)
- [X] gsw_hill_ratio_at_sp2(t)
- [X] gsw_gibbs(ns,nt,np,sa,t,p)
- [X] gsw_gibbs_ice (nt, np, t, p)


# Missing in GSW-Rust implementation

- [ ] gsw_adiabatic_lapse_rate_ice (t, p)
- [ ] gsw_alpha_wrt_t_exact(sa,t,p)
- [ ] gsw_alpha_wrt_t_ice (t, p)
- [ ] gsw_beta_const_t_exact(sa,t,p)
- [ ] gsw_chem_potential_water_ice (t, p)
- [ ] gsw_chem_potential_water_t_exact (sa, t, p)
- [ ] gsw_cp_ice (t, p)
- [ ] gsw_cp_t_exact(sa,t,p)
- [ ] gsw_ct_freezing (sa, p, saturation_fraction)
- [ ] gsw_ct_freezing_poly (sa, p, saturation_fraction)
- [ ] gsw_ct_from_enthalpy_exact (sa, h, p)
- [ ] gsw_ct_from_t(sa,t,p)
- [ ] gsw_deltasa_atlas(p,lon,lat)
- [ ] gsw_dilution_coefficient_t_exact (sa, t, p)
- [ ] gsw_enthalpy_ct_exact (sa, ct, p)
- [ ] gsw_enthalpy_ice (t, p)
- [ ] gsw_enthalpy_t_exact(sa,t,p)
- [ ] gsw_entropy_ice (t, p)
- [ ] gsw_entropy_part(sa,t,p)
- [ ] gsw_entropy_part_zerop(sa,pt0)
- [ ] gsw_fdelta(p,lon,lat)
- [ ] gsw_gibbs_ice_part_t (t, p)
- [ ] gsw_gibbs_ice_pt0 (pt0)
- [ ] gsw_gibbs_ice_pt0_pt0 (pt0)
- [ ] gsw_gibbs_pt0_pt0(sa,pt0)
- [ ] gsw_helmholtz_energy_ice (t, p)
- [ ] gsw_internal_energy_ice (t, p)
- [ ] gsw_kappa_const_t_ice (t, p)
- [ ] gsw_kappa_ice (t, p)
- [ ] gsw_kappa_t_exact(sa,t,p)
- [ ] gsw_latentheat_evap_ct(sa,ct)
- [ ] gsw_latentheat_evap_t(sa,t)
- [ ] gsw_latentheat_melting(sa,p)
- [ ] gsw_melting_ice_equilibrium_sa_ct_ratio (sa, p)
- [ ] gsw_melting_ice_equilibrium_sa_ct_ratio_poly (sa, p)
- [ ] gsw_melting_ice_sa_ct_ratio (sa, ct, p, t_ih)
- [ ] gsw_melting_ice_sa_ct_ratio_poly (sa, ct, p, t_ih)
- [ ] gsw_melting_seaice_equilibrium_sa_ct_ratio (sa, p)
- [ ] gsw_melting_seaice_equilibrium_sa_ct_ratio_poly (sa, p)
- [ ] gsw_melting_seaice_sa_ct_ratio (sa, ct, p, sa_seaice, &
- [ ] gsw_melting_seaice_sa_ct_ratio_poly (sa, ct, p, &
- [ ] gsw_o2sol(sa, ct, p, lon, lat)
- [ ] gsw_o2sol_sp_pt(sp, pt)
- [ ] gsw_pot_enthalpy_from_pt_ice (pt0_ice)
- [ ] gsw_pot_enthalpy_from_pt_ice_poly (pt0_ice)
- [ ] gsw_pot_enthalpy_ice_freezing (sa, p)
- [ ] gsw_pot_enthalpy_ice_freezing_poly (sa, p)
- [ ] gsw_pot_rho_t_exact(sa,t,p,p_ref)
- [ ] gsw_pressure_coefficient_ice (t, p)
- [ ] gsw_pressure_freezing_ct (sa, ct, saturation_fraction)
- [ ] gsw_pt0_cold_ice_poly (pot_enthalpy_ice)
- [ ] gsw_pt0_from_t_ice (t, p)
- [ ] gsw_pt_from_pot_enthalpy_ice (pot_enthalpy_ice)
- [ ] gsw_pt_from_pot_enthalpy_ice_poly (pot_enthalpy_ice)
- [ ] gsw_pt_from_pot_enthalpy_ice_poly_dh (pot_enthalpy_ice)
- [ ] gsw_pt_from_t_ice (t, p, p_ref)
- [ ] gsw_rho_ice (t, p)
- [ ] gsw_rho_t_exact(sa,t,p)
- [ ] gsw_sa_freezing_estimate (p, saturation_fraction, ct, t)
- [ ] gsw_sa_freezing_from_ct (ct, p, saturation_fraction)
- [ ] gsw_sa_freezing_from_ct_poly (ct, p, saturation_fraction)
- [ ] gsw_sa_freezing_from_t (t, p, saturation_fraction)
- [ ] gsw_sa_freezing_from_t_poly (t, p, saturation_fraction)
- [ ] gsw_sa_from_sp(sp,p,lon,lat)
- [ ] gsw_sa_from_sp_baltic(sp,lon,lat)
- [ ] gsw_sa_p_inrange (sa, p)
- [ ] gsw_saar(p,long,lat)
- [ ] gsw_sound_speed_ice (t, p)
- [ ] gsw_sound_speed_t_exact(sa,t,p)
- [ ] gsw_sp_from_sa_baltic(sa,lon,lat)
- [ ] gsw_specvol_t_exact(sa,t,p)
- [ ] gsw_t_deriv_chem_potential_water_t_exact (sa, t, p)
- [ ] gsw_t_freezing(sa,p,saturation_fraction)
- [ ] gsw_t_freezing_poly (sa, p, saturation_fraction)
- [ ] gsw_t_from_pt0_ice (pt0_ice, p)
- [ ] gsw_util_interp1q_int (x, iy, x_i) result(y_i)
- [ ] gsw_util_linear_interp (x, y, x_i) result(y_i)
- [ ] gsw_util_sort_real (rarray) result(iarray)
- [ ] gsw_util_xinterp1(x,y,n,x0)
- [ ] gsw_specvol_ice (t, p)
