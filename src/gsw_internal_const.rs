/// ! GSW TEOS10 Constants

#[allow(clippy::approx_constant)]
/// Pi
pub const PI: f64 = if cfg!(feature = "compat") {
    3.141592653589793
} else {
    core::f64::consts::PI
};

pub const DEG2RAD: f64 = PI / 180.0;

/// If the graviational acceleration were to be regarded as being
/// depth-independent, which is often the case in ocean models, then gamma
/// would be set to be zero.
pub const GAMMA: f64 = if cfg!(feature = "nodgdz") {
    0.0
} else {
    2.26e-7
};

/// Specific Heat [ J/(kg K) ]
/// cp0: The "specific heat" for use with Conservative Temperature
pub const GSW_CP0: f64 = 3_991.867_957_119_63;

pub const GSW_C3515: f64 = 42.914_0;

/// Conversion factor deciBar to Pascal
pub const DB2PA: f64 = 1.0e4;
pub const PA2DB: f64 = 1.0e-4;

/// One standard atmosphere [Pa]
pub const GSW_P0: f64 = 101_325.0;

/// SSO: Standard Ocean Reference Salinity [ g/kg ].
pub const GSW_SSO: f64 = 35.16504;
pub const GSW_SQRTSSO: f64 = 5.930011804372737;

/// uPS: unit conversion factor for salinities (Millero et al., 2008) [g/kg]
/// Reference Salinity SR is uPS times Practical Salinity SP.
pub const GSW_UPS: f64 = GSW_SSO / 35.0;

pub const GSW_SAU: f64 = 40. * GSW_UPS;

/// Convervative Temperature [degree Celsius] (Roquet, 2015 apud TEOS-10 p.131)
pub const GSW_CTU: f64 = 40.;

/// Scalling Pressure [dbar] (Roquet, 2015, apud TEOS-10 p.131)
pub const GSW_PU: f64 = 1e4;

/// sfac  =  1/(40*gsw_ups) ~ 0.024882667558461472
pub const GSW_SFAC: f64 = if cfg!(feature = "compat") {
    // Other implementations hardcoded some constants truncating its values on
    // 1e-16.  The difference should be negligible but it impacts the
    // validation tests.
    0.0248826675584615
} else {
    // Two extra digits on precision compared with other implementations.
    1.0 / (40.0 * GSW_UPS)
};

/// offset = deltaS*gsw_sfac = 5.971840214030754e-1, where deltaS = 24
pub const OFFSET: f64 = if cfg!(feature = "compat") {
    5.971840214030754e-1
} else {
    24.0 * GSW_SFAC
};
