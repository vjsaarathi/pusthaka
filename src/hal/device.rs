use super::{battery::BatteryGauge, clock::Clock, display::EinkDisplay, firmware::FirmwareUpdater, frontlight::Frontlight, input::InputSource, network::NetworkInterface, power::PowerController, storage::StorageBackend, touch::TouchSource};
use super::capabilities::DeviceDescriptor;

/// The dependency-injection boundary for a physical reader.
///
/// A device provides implementations of the HAL contracts. Higher layers receive
/// these implementations through `DeviceServices` and remain unaware of the board.
pub trait Device {
    type Display: EinkDisplay;
    type Storage: StorageBackend;
    type Input: InputSource;
    type Power: PowerController;
    type Clock: Clock;
    type Battery: BatteryGauge;
    type Frontlight: Frontlight;
    type Network: NetworkInterface;
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
