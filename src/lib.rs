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

mod error;
pub use crate::error::{Error, Result};
