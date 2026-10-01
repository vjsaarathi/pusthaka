/// Abstraction for device power states and power management ICs (PMIC)
pub trait PowerBackend {
    type Error: core::fmt::Debug;

    /// Put the CPU into a deep sleep state (Suspend to RAM)
    /// The system will halt here until a hardware interrupt wakes it up.
    fn sleep(&mut self) -> Result<(), Self::Error>;
    
    /// Power down the device entirely.
    /// This function should ideally never return.
    fn shutdown(&mut self) -> Result<(), Self::Error>;
    
    /// Hard reboot the device.
    /// This function should ideally never return.
    fn reboot(&mut self) -> Result<(), Self::Error>;
}
