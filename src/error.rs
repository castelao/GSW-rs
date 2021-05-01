cfg_if::cfg_if! {
if #[cfg(feature = "std")] {
  extern crate std;

  #[derive(thiserror::Error, Debug)]
  pub enum Error {
    #[error("Negative salinity")]
    NegativeSalinity
  }

  impl From<&str> for Error {
    fn from(other: &str) -> Error {
      match other {
        "Negative SA" => Error::NegativeSalinity,
        _ => unimplemented!()
      }
    }
  }

} else {
  pub type Error = &'static str;
}
}

pub type Result<T> = core::result::Result<T, Error>;
