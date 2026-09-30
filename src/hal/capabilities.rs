/// Optional or required hardware capability represented as a single bit.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(u32)]
pub enum Capability {
    Grayscale = 1 << 0,
    Touch = 1 << 1,
    Frontlight = 1 << 2,
    WarmFrontlight = 1 << 3,
    Rtc = 1 << 4,
    BatteryGauge = 1 << 5,
    Accelerometer = 1 << 6,
    Wifi = 1 << 7,
    UsbMassStorage = 1 << 8,
    Ota = 1 << 9,
}

/// Compact capability bit-set used by runtime feature selection.
///
/// This is intentionally a small value type so a device descriptor can be copied
/// without heap allocation or dynamic dispatch.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Default)]
pub struct CapabilitySet(pub u32);

pub type DeviceCapabilities = CapabilitySet;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum FeatureProfile { Core, Extended, Connected }

impl CapabilitySet {
    pub const fn empty() -> Self { Self(0) }

    pub const fn from_capability(capability: Capability) -> Self {
        Self(capability as u32)
    }

    pub const fn contains(self, capability: Capability) -> bool {
        self.0 & (capability as u32) != 0
    }

    pub const fn contains_all(self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }

    pub fn insert(&mut self, capability: Capability) {
        self.0 |= capability as u32;
    }

    pub fn remove(&mut self, capability: Capability) {
        self.0 &= !(capability as u32);
    }
}

/// Hard memory properties reported by a device.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct MemoryProfile {
    pub total_ram_bytes: usize,
    pub firmware_flash_bytes: usize,
    pub max_framebuffer_bytes: usize,
    pub max_font_cache_bytes: usize,
    pub max_working_set_bytes: usize,
}

impl MemoryProfile {
    pub const MINIMUM: Self = Self {
        total_ram_bytes: 512 * 1024,
        firmware_flash_bytes: 4 * 1024 * 1024,
        max_framebuffer_bytes: 64 * 1024,
        max_font_cache_bytes: 128 * 1024,
        max_working_set_bytes: 256 * 1024,
    };
}

/// Hardware-neutral display description.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DisplayDescriptor {
    pub width: u16,
    pub height: u16,
    pub native_grayscale_levels: u8,
    pub framebuffer_bytes: usize,
    pub has_partial_refresh: bool,
    pub has_async_refresh: bool,
}

/// Hardware-neutral input description.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct InputDescriptor {
    pub physical_buttons: u8,
    pub has_touch: bool,
    pub has_gesture_input: bool,
}

/// Hardware-neutral user-storage description.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct StorageDescriptor {
    pub capacity_bytes: u64,
    pub removable: bool,
    pub supports_directories: bool,
    pub supports_atomic_rename: bool,
    pub filesystem_name: &'static str,
}

/// Complete immutable descriptor supplied by a concrete device implementation.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DeviceDescriptor {
    pub id: &'static str,
    pub display: DisplayDescriptor,
    pub input: InputDescriptor,
    pub storage: StorageDescriptor,
    pub memory: MemoryProfile,
    pub capabilities: CapabilitySet,
}

impl DeviceDescriptor {
    /// Checks the hard minimum contract before boot continues.
    pub fn satisfies_minimum(&self) -> bool {
        self.memory.total_ram_bytes >= MemoryProfile::MINIMUM.total_ram_bytes
            && self.memory.firmware_flash_bytes >= MemoryProfile::MINIMUM.firmware_flash_bytes
            && self.memory.max_framebuffer_bytes >= self.display.framebuffer_bytes
            && self.storage.capacity_bytes > 0
            && self.display.width > 0
            && self.display.height > 0
            && self.display.framebuffer_bytes <= self.memory.max_framebuffer_bytes
    }
}
