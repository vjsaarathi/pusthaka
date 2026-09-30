/// Monotonic elapsed time measured from boot.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct MonotonicInstant(pub u64);

/// Duration used by hardware-independent timing contracts.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct Duration(pub u64);

impl Duration {
    pub const ZERO: Self = Self(0);
    pub const fn from_millis(value: u64) -> Self { Self(value) }
    pub const fn as_millis(self) -> u64 { self.0 }
}

/// Monotonic time source. The reader core uses this instead of a board timer directly.
pub trait TimeSource {
    fn now(&self) -> MonotonicInstant;
}
