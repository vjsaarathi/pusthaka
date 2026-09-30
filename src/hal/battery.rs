#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct BatteryStatus {
    pub percent: u8,
    pub charging: bool,
    pub millivolts: u16,
}

pub trait BatteryGauge {
    type Error;
    fn status(&mut self) -> Result<BatteryStatus, Self::Error>;
}
