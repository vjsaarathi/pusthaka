use crate::hal::device::DeviceServices;
use crate::hal::device::Device;

/// Top-level runtime object. It is intentionally constructed and passed around rather
/// than exposed as a global mutable singleton.
pub struct System<D: Device> {
    pub device: DeviceServices<D>,
}

impl<D: Device> System<D> {
    pub fn new(device: D) -> Self {
        Self { device: device.into_services() }
    }
}
