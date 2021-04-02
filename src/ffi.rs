#[no_mangle]
pub unsafe extern "C" fn gsw_specvol(sa: f64, ct: f64, p: f64) -> f64 {
    crate::gsw_specvol(sa, ct, p)
}
