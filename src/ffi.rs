extern crate std;

#[no_mangle]
pub unsafe extern "C" fn gsw_specvol(sa: f64, ct: f64, p: f64) -> f64 {
    crate::gsw_specvol(sa, ct, p)
}

/*
 * double *input_data, double lon, double lat,
                double long_grid, double lat_grid, double dlong_grid,
                double dlat_grid, double *output_data);
*/
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
    unimplemented!();
}

#[no_mangle]
pub unsafe extern "C" fn gsw_add_mean(data_in: *mut f64, data_out: *mut f64) {
    unimplemented!();
}

#[no_mangle]
pub unsafe extern "C" fn gsw_adiabatic_lapse_rate_from_ct(sa: f64, ct: f64, p: f64) -> f64 {
    unimplemented!();
}

#[no_mangle]
pub unsafe extern "C" fn gsw_adiabatic_lapse_rate_ice(t: f64, p: f64) -> f64 {
    unimplemented!();
}

#[no_mangle]
pub unsafe extern "C" fn gsw_alpha(sa: f64, ct: f64, p: f64) -> f64 {
    unimplemented!();
}

#[no_mangle]
pub unsafe extern "C" fn gsw_alpha_on_beta(sa: f64, ct: f64, p: f64) -> f64 {
    unimplemented!();
}

#[no_mangle]
pub unsafe extern "C" fn gsw_alpha_wrt_t_exact(sa: f64, t: f64, p: f64) -> f64 {
    unimplemented!();
}

#[no_mangle]
pub unsafe extern "C" fn gsw_alpha_wrt_t_ice(t: f64, p: f64) -> f64 {
    unimplemented!();
}

#[no_mangle]
pub unsafe extern "C" fn gsw_beta_const_t_exact(sa: f64, t: f64, p: f64) -> f64 {
    unimplemented!();
}

#[no_mangle]
pub unsafe extern "C" fn gsw_beta(sa: f64, ct: f64, p: f64) -> f64 {
    unimplemented!();
}

#[no_mangle]
pub unsafe extern "C" fn gsw_cabbeling(sa: f64, ct: f64, p: f64) -> f64 {
    unimplemented!();
}

#[no_mangle]
pub unsafe extern "C" fn gsw_c_from_sp(sp: f64, t: f64, p: f64) -> f64 {
    unimplemented!();
}

#[no_mangle]
pub unsafe extern "C" fn gsw_chem_potential_water_ice(t: f64, p: f64) -> f64 {
    unimplemented!();
}

#[no_mangle]
pub unsafe extern "C" fn gsw_chem_potential_water_t_exact(sa: f64, t: f64, p: f64) -> f64 {
    unimplemented!();
}

#[no_mangle]
pub unsafe extern "C" fn gsw_cp_ice(t: f64, p: f64) -> f64 {
    unimplemented!();
}

#[cfg(test)]
mod test {
    use super::std::format;

    use inline_c::assert_c;

    #[test]
    fn test_specvol_c() {
        let result: f64 = crate::gsw_specvol(1., 1., 1.);
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
}
