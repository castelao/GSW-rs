//! Gibbs Sea Water Oceanographic Toolbox of TEOS-10
//!

/// Specific Heat [ J/(kg K) ]
/// cp0: The "specific heat" for use with Conservative Temperature
const GSW_CP0: f64 = 3991.86795711963;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
