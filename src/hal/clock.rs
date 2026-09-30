use crate::kernel::time::{Duration, MonotonicInstant};

/// Wall-clock representation independent of any particular RTC chip.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WallClockTime {
    pub unix_seconds: i64,
}

pub trait Clock {
    type Error;
    fn now_monotonic(&self) -> MonotonicInstant;
    fn sleep(&mut self, duration: Duration) -> Result<(), Self::Error>;
    fn wall_clock(&mut self) -> Result<Option<WallClockTime>, Self::Error>;
    fn set_wall_clock(&mut self, value: WallClockTime) -> Result<(), Self::Error>;
}
