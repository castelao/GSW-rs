//! # Gibbs Sea Water
//!
//! Unofficial Gibbs Sea Water Oceanographic Toolbox of TEOS-10,
//! based on version 3.06.12, implemented in Rust.
//!
//! Note that we do follow TEOS-10 manual and references, but his library is
//! not endorsed by TEOS-10 committee. For the official TEOS-10 documentation
//! and other software implementations, check: <http://www.teos-10.org>
//!
//! GSW-rs was initially implemented to provide support for embedded
//! applications, but there is no restrictions on using it in regular
//! computers or even HPC. It is an alternative for GSW-C, and we demonstrate
//! how to link Python and R implementations of GSW with our Rust version
//! instead.
//!
//! ## Features
//!
//! A few customizations can be choosen at compiling time by activating the
//! following features:
//!
//! - **capi**: Include the C-API so that GSW-rs can be accessed as if it was
//!   a C-library. For instance, the other GSW implementations based on GSW-C
//!   could be linked with GSW-rs instead by using this feature.
//! - **compat**: Reproduces the GSW-Matlab implementation for compatibility.
//!   This option can result in negligible rounding differences. The most
//!   notable differences are on handling special cases such as negative
//!   salinity.
//! - **invalidasnan**: Returns NaN values on failure. The default behavior is
//!   to return an error. This is the closest option to reproduce CSW-C
//!   implementation, when that one deviates from GSW-Matlab. The default
//!   behavior returns NaN if the input is a NaN, or returns an *Error* on
//!   failure.
//! - **nodgdz**: Ignores vertical variations of gravity, i.e. no dependency
//!   on *z*. This might be useful on some numerical models.
//! - **std**: Activate the Rust standard library. The default implementation
//!   does not rely on standard library so it can run in embedded systems.
//!
//! For instance, to compile it compatible with the Matlab library:
//! ```text
//! cargo build --features compat
//! ```
//!
//! While compat results in precisely reproducing GSW-Matlab results, there is
//! no equivalent for GSW-C, which is currently the base for Julia, Python,
//! and R. The closest option to that is using **invalidasnan**, i.e.
//! `--features invalidasnan`.
//!
//! Since these checks are defined at compiling time, those do not necessarily
//! imply extra computing cost at running time. For instance, the cost of the
//! **compat** checks do not affect at all when compiled with the default
//! behavior.
//!
//! ## Design considerations
//!
//! - Functions that can result in failure return type Result<T>, as usuall
//!   in Rust. We prefer to return errors instead of NaN for two main reasons:
//!   We can shortcut unecessary long calculations, and we can provide more
//!   information context for the next layer to support automatic decisions.
//!   For instance, an application using GSW-rs might respond differently
//!   if gets back an error due to an invalid salinity such as a negative
//!   value versus a feasible salinity but out of the valid range.
//! - The validation dataset from GSW-Matlab is converted to small binaries
//!   so that we can run tests directly in embedded systems.
//! - We reinforce the theoretical validity range. If the original publication
//!   of an approximation defines that it was valid in the salinity range from
//!   0 to 42, we reinforce that in the implemented function. We believe that
//!   it is our job to avoid conceptual errors.
//!
//! ## References
//!
//! There is a long list of references which should be all listed at:
//!
//! IOC, SCOR and IAPSO, 2010: The international thermodynamic equation of
//! seawater – 2010: Calculation and use of thermodynamic properties.
//! Intergovernmental Oceanographic Commission, Manuals and Guides No. 56,
//! UNESCO (English), 196 pp.
//!
//! Note that in GSW-rs, the functions missing references means that we have
//! not have the chance to review it yet, but we are working on that.
//!
//! To cite this library, please check guidance in the README.

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
