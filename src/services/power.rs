use crate::hal::power::PowerBackend;
use crate::hal::battery::{BatteryBackend, BatteryMetrics};

pub struct PowerManager<'a, P: PowerBackend, B: BatteryBackend> {
    pub power_hal: &'a mut P,
    pub battery_hal: &'a mut B,
}

impl<'a, P: PowerBackend, B: BatteryBackend> PowerManager<'a, P, B> {
    pub fn new(power_hal: &'a mut P, battery_hal: &'a mut B) -> Self {
        Self { power_hal, battery_hal }
    }

    pub fn enter_sleep(&mut self) -> Result<(), P::Error> {
        self.power_hal.sleep()
    }

    pub fn shutdown(&mut self) -> Result<(), P::Error> {
        self.power_hal.shutdown()
    }

    pub fn get_battery_percentage(&mut self) -> Result<u8, B::Error> {
        self.battery_hal.level()
    }

    pub fn is_charging(&self) -> Result<bool, B::Error> {
        self.battery_hal.is_charging()
    }

    pub fn get_metrics(&self) -> Result<BatteryMetrics, B::Error> {
        self.battery_hal.metrics()
    }
}
