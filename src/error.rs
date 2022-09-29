#[cfg(feature = "std")]
extern crate std;

#[cfg_attr(feature = "std", derive(thiserror::Error))]
#[derive(Debug)]
pub enum Error {
    #[cfg_attr(feature = "std", error("Negative salinity"))]
    NegativeSalinity,

    #[cfg_attr(feature = "std", error("Value out of bounds"))]
    /// A value out of expected or working limits. For instance a latitude
    /// larger than 90 degrees or a temperature of 80 C.
    OutOfBounds,

    #[cfg_attr(feature = "std", error("Undefined error"))]
    Undefined,
}

pub type Result<T> = core::result::Result<T, Error>;
