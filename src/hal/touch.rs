use crate::hal::input::RawTouchSample;
use crate::kernel::time::MonotonicInstant;

pub trait TouchSource {
    type Error;
    fn poll(&mut self, now: MonotonicInstant) -> Result<Option<RawTouchSample>, Self::Error>;
}
