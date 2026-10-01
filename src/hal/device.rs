use super::{battery::BatteryBackend, clock::RtcBackend, display::EinkDisplay, firmware::FirmwareUpdater, frontlight::Frontlight, input::InputSource, network::NetworkBackend, power::PowerBackend, storage::StorageBackend, touch::TouchSource};
use super::capabilities::DeviceDescriptor;

/// The dependency-injection boundary for a physical reader.
///
/// A device provides implementations of the HAL contracts. Higher layers receive
/// these implementations through `DeviceServices` and remain unaware of the board.
pub trait Device {
    type Display: EinkDisplay;
    type Storage: StorageBackend;
    type Input: InputSource;
    type Power: PowerBackend;
    type Clock: RtcBackend;
    type Battery: BatteryBackend;
    type Frontlight: Frontlight;
    type Network: NetworkBackend;
    type Touch: TouchSource;
    type Firmware: FirmwareUpdater;

    fn descriptor(&self) -> DeviceDescriptor;
    fn into_services(self) -> DeviceServices<Self> where Self: Sized;
}

/// Concrete service bundle constructed once during boot and then dependency-injected.
pub struct DeviceServices<D: Device> {
    pub display: D::Display,
    pub storage: D::Storage,
    pub input: D::Input,
    pub power: D::Power,
    pub clock: D::Clock,
    pub battery: D::Battery,
    pub frontlight: D::Frontlight,
    pub network: D::Network,
    pub touch: D::Touch,
    pub firmware: D::Firmware,
}
