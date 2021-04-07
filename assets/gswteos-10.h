/* GSW TEOS-10 V3.05 definitions and prototypes. */

#ifndef GSWTEOS_10_H
#define GSWTEOS_10_H

#ifdef __cplusplus
extern "C" {
#endif // __cplusplus

double gsw_specvol(double sa, double ct, double p);

void gsw_add_barrier(double *input_data,
                     double lon,
                     double lat,
                     double long_grid,
                     double lat_grid,
                     double dlong_grid,
                     double dlat_grid,
                     double *output_data);

void gsw_add_mean(double *data_in, double *data_out);

double gsw_adiabatic_lapse_rate_from_ct(double sa, double ct, double p);

double gsw_adiabatic_lapse_rate_ice(double t, double p);

double gsw_alpha(double sa, double ct, double p);

double gsw_alpha_on_beta(double sa, double ct, double p);

double gsw_alpha_wrt_t_exact(double sa, double t, double p);

double gsw_alpha_wrt_t_ice(double t, double p);

double gsw_beta_const_t_exact(double sa, double t, double p);

double gsw_beta(double sa, double ct, double p);

double gsw_cabbeling(double sa, double ct, double p);

double gsw_c_from_sp(double sp, double t, double p);

double gsw_chem_potential_water_ice(double t, double p);

double gsw_chem_potential_water_t_exact(double sa, double t, double p);

double gsw_cp_ice(double t, double p);

#ifdef __cplusplus
} // extern "C"
#endif // __cplusplus

#endif /* GSWTEOS_10_H */
