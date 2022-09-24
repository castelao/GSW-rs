//! # Gibbs Sea Water
//!
//! Gibbs Sea Water Oceanographic Toolbox of TEOS-10 implemented in Rust.
//! version: 3.06.12
//!
//! For the official TEOS-10 documentation and other software implementations,
//! check the webiste: <http://www.teos-10.org>
//!
//! GSW-rs was initially implementated to provide support for embedded
//! applications, but there is no restrictions on using it in regular
//! computers or even HPC. It is an alternative for GSW-C, and we demonstrate
//! how to link Python and R implementations of GSW with our Rust version
//! instead.
//!
//! ## Features
//!
//! From the Cargo Book: "Cargo 'features' provide a mechanism to express
//! conditional compilation and optional dependencies.". The features defined
//! in GSW-rs are:
//!
//! - capi: Include the C-API so that GSW-rs can be accessed as it was a
//!         C-library. For instance, the other GSW implementations based on
//!         GSW-C could be linked with GSW-rs instead by using this feature.
//! - compat: Reproduces the GSW-Matlab implementation for compatibility.
//! - invalidasnan: Returns NaN values on failure. The default behavior is to
//!                 return an error.
//! - nodgdz: Ignores vertical variations of gravity, i.e. no dependency on z.
//!           This might be useful on some numerical models.
//! - std: Activate the Rust standard library. The default implementation does
//!        not rely on std so it can run in embedded systems .
//!
//! For example, to compile it compatible with the Matlab library:
//! cargo build --features compat

////////////////////////////////////////////////////////////////////////////////

// Do not depend on the standard library
#![no_std]

/// cbindgen:ignore
#[allow(unused)]
mod gsw_internal_const;

/// cbindgen:ignore
#[allow(unused)]
mod gsw_sp_coefficients;

/// cbindgen:ignore
#[allow(unused)]
mod gsw_specvol_coefficients;

#[cfg(feature = "capi")]
mod ffi;

pub mod conversions;
pub mod earth;
mod gsw_internal_funcs;
pub mod practical_salinity;
pub mod volume;

mod error;
pub use crate::error::{Error, Result};
