/// Represents a timestamp (seconds since UNIX epoch)
pub type UnixTimestamp = u64;

/// Hardware Real-Time Clock (RTC) abstraction
pub trait RtcBackend {
    type Error: core::fmt::Debug;

    /// Get the current time from the RTC
    fn now(&self) -> Result<UnixTimestamp, Self::Error>;
    
    /// Sync the RTC to a new timestamp (e.g. from NTP)
    fn set_time(&mut self, timestamp: UnixTimestamp) -> Result<(), Self::Error>;
    
    /// Schedule a hardware interrupt to wake the device from sleep at a specific time
    fn set_wake_alarm(&mut self, timestamp: UnixTimestamp) -> Result<(), Self::Error>;
    
    /// Clear any pending hardware alarms
    fn clear_alarm(&mut self) -> Result<(), Self::Error>;
}
