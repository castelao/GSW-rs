extern crate std;

#[no_mangle]
pub unsafe extern "C" fn gsw_specvol(sa: f64, ct: f64, p: f64) -> f64 {
    crate::gsw_specvol(sa, ct, p)
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
