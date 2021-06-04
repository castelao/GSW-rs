#[cfg(feature = "std")]
extern crate std;

#[cfg_attr(feature = "std", derive(thiserror::Error))]
#[derive(Debug)]
pub enum Error {
    #[cfg_attr(feature = "std", error("Negative salinity"))]
    NegativeSalinity,
}

pub type Result<T> = core::result::Result<T, Error>;
