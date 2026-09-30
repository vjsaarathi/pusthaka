use crate::hal::capabilities::{CapabilitySet, DeviceDescriptor, DisplayDescriptor, InputDescriptor, MemoryProfile, StorageDescriptor};

/// Host-only device descriptor used to validate hardware-independent logic.
pub struct VirtualDevice {
    pub descriptor: DeviceDescriptor,
}

impl VirtualDevice {
    pub fn new(width: u16, height: u16) -> Self {
        let framebuffer_bytes = (width as usize * height as usize).div_ceil(8);
        let mut capabilities = CapabilitySet::empty();
        capabilities.insert(crate::hal::capabilities::Capability::Grayscale);
        capabilities.insert(crate::hal::capabilities::Capability::Touch);
        Self {
            descriptor: DeviceDescriptor {
                id: "virtual",
                display: DisplayDescriptor {
                    width,
                    height,
                    native_grayscale_levels: 4,
                    framebuffer_bytes,
                    has_partial_refresh: true,
                    has_async_refresh: true,
                },
                input: InputDescriptor { physical_buttons: 7, has_touch: true, has_gesture_input: true },
                storage: StorageDescriptor {
                    capacity_bytes: 32 * 1024 * 1024,
                    removable: true,
                    supports_directories: true,
                    supports_atomic_rename: true,
                    filesystem_name: "virtual",
                },
                memory: MemoryProfile {
                    total_ram_bytes: 8 * 1024 * 1024,
                    firmware_flash_bytes: 16 * 1024 * 1024,
                    max_framebuffer_bytes: 64 * 1024,
                    max_font_cache_bytes: 512 * 1024,
                    max_working_set_bytes: 1024 * 1024,
                },
                capabilities,
            },
        }
    }
}
