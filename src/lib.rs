//! # Gibbs Sea Water
//!
//! Gibbs Sea Water Oceanographic Toolbox of TEOS-10 implemented in Rust.
//! version: 3.06.12
//!
//! http://www.teos-10.org
//!

////////////////////////////////////////////////////////////////////////////////

// Do not depend on the standard library
#![no_std]

/// cbindgen:ignore
#[allow(unused)]
mod gsw_internal_const;

/// cbindgen:ignore
#[allow(unused)]
mod gsw_specvol_coefficients;

#[cfg(feature = "capi")]
mod ffi;

pub mod conversions;
mod gsw_internal_funcs;
pub mod volume;

use crate::gsw_internal_const::*;
pub use crate::volume::{
    alpha, beta, rho, specvol, specvol_alpha_beta, specvol_anom_standard, specvol_sso_0,
};

#[cfg(test)]
mod tests {
    use super::GSW_SFAC;

    #[test]
    // Calculated SFAC is slightly different than the prescribed SFAC in other
    // packages. Prescribed ends in 4615 while the here calculated ends in
    // 461472. Which is the correct one?
    fn test_const_sfac() {
        if cfg!(feature = "compat") {
            assert_eq!(GSW_SFAC, 0.0248826675584615);
        } else {
            assert_eq!(GSW_SFAC, 0.024882667558461472);
        }
    }

    #[test]
    fn test_specvol_alpha_beta() {
        #[cfg(feature = "compat")]
        // Values from C implementation
        assert_eq!(
            super::specvol_alpha_beta(34.537484086977358, 27.793319825682374, 50.),
            (
                0.00097826888242888476,
                0.00031741177706767163,
                0.00071877529859646001
            )
        );
    }
}
