/// ! GSW TEOS10 Constants

/// Conversion factor deciBar to Pascal
pub const DB2PA: f64 = 1.0e4;

/// SSO: Standard Ocean Reference Salinity [ g/kg ].
pub const GSW_SSO: f64 = 35.16504;
pub const GSW_SQRTSSO: f64 = 5.930011804372737;

/// uPS: unit conversion factor for salinities [ g/kg ]
pub const GSW_UPS: f64 = GSW_SSO / 35.0;

#[cfg(feature = "truncated")]
/// Other implementations hardcoded some constants truncating its values
/// sfac  =  1/(40*gsw_ups) ~ 0.0248826675584615
pub const GSW_SFAC: f64 = 0.0248826675584615;

#[cfg(not(feature = "truncated"))]
/// sfac  =  1/(40*gsw_ups) = 0.024882667558461472
/// Two extra digits on precision compared with other implementations. The
/// difference should be negligible but it impacts the validation tests.
pub const GSW_SFAC: f64 = 1.0 / (40.0 * GSW_UPS);
