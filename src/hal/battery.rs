/// Advanced metrics for physical battery health and status
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct BatteryMetrics {
    /// Percentage from 0 to 100
    pub level_percent: u8,
    /// Voltage in millivolts
    pub voltage_mv: u16,
    /// Current draw in milliamps (negative = draining, positive = charging)
    pub current_ma: i16,
    /// Estimated temperature in Celsius
    pub temperature_c: i8,
}

/// Abstract representation of the physical battery controller or fuel gauge (e.g. MAX17048)
pub trait BatteryBackend {
    type Error: core::fmt::Debug;

    /// Check if the device is currently plugged into a power source
    fn is_charging(&self) -> Result<bool, Self::Error>;
    
    /// Retrieve the current battery percentage (0-100)
    fn level(&self) -> Result<u8, Self::Error>;
    
    /// Retrieve extensive metrics from the fuel gauge
    fn metrics(&self) -> Result<BatteryMetrics, Self::Error>;
}
