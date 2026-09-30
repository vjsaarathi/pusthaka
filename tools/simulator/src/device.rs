use reader_os::hal::device::{Device, DeviceServices};
use reader_os::hal::capabilities::*;
use reader_os::hal::battery::{BatteryGauge, BatteryStatus};
use reader_os::hal::clock::Clock;
use reader_os::hal::display::{EinkDisplay, DisplayCapabilities, RefreshRequest};
use reader_os::hal::firmware::FirmwareUpdater;
use reader_os::hal::frontlight::Frontlight;
use reader_os::hal::input::{InputSource, InputEvent};
use reader_os::hal::network::{NetworkInterface, HttpResponseSink};
use reader_os::hal::power::PowerController;
use reader_os::hal::storage::{StorageBackend, ReadFile, WriteFile, StoragePath};
use reader_os::hal::touch::{TouchSource};
use reader_os::kernel::time::{Duration, MonotonicInstant};
use reader_os::hal::clock::WallClockTime;
use reader_os::services::ota::FirmwareImageSource;
use reader_os::services::networking::{HttpRequest, HttpResponseHead};
use reader_os::graphics::framebuffer::{FrameBufferView};
use std::vec::Vec;

#[derive(Debug)]
pub struct MockError;

pub struct SimDisplay {
    pub width: u16,
    pub height: u16,
    pub fb_bytes: std::vec::Vec<u8>,
}
impl EinkDisplay for SimDisplay {
    type Error = MockError;
    fn capabilities(&self) -> DisplayCapabilities {
        DisplayCapabilities { grayscale_levels: 4, partial_refresh: true, asynchronous_refresh: true, waveform_control: true }
    }
    fn refresh(&mut self, frame: &FrameBufferView<'_>, _request: RefreshRequest) -> Result<(), Self::Error> {
        let size = frame.bytes.len().min(self.fb_bytes.len());
        self.fb_bytes[..size].copy_from_slice(&frame.bytes[..size]);
        Ok(())
    }
    fn is_busy(&self) -> bool { false }
    fn sleep(&mut self) -> Result<(), Self::Error> { Ok(()) }
}

pub struct SimStorageReader;
impl ReadFile for SimStorageReader {
    type Error = MockError;
    fn read(&mut self, _buffer: &mut [u8]) -> Result<usize, Self::Error> { Ok(0) }
    fn seek(&mut self, _offset: u64) -> Result<(), Self::Error> { Ok(()) }
    fn position(&mut self) -> Result<u64, Self::Error> { Ok(0) }
    fn size(&mut self) -> Result<u64, Self::Error> { Ok(0) }
    fn close(&mut self) -> Result<(), Self::Error> { Ok(()) }
}
pub struct SimStorageWriter;
impl WriteFile for SimStorageWriter {
    type Error = MockError;
    fn write(&mut self, buffer: &[u8]) -> Result<usize, Self::Error> { Ok(buffer.len()) }
    fn flush(&mut self) -> Result<(), Self::Error> { Ok(()) }
    fn close(&mut self) -> Result<(), Self::Error> { Ok(()) }
}
pub struct SimStorage;
impl StorageBackend for SimStorage {
    type Error = MockError;
    type Reader<'a> = SimStorageReader;
    type Writer<'a> = SimStorageWriter;
    fn begin(&mut self) -> Result<(), Self::Error> { Ok(()) }
    fn open_read<'a>(&'a mut self, _path: StoragePath<'_>) -> Result<Self::Reader<'a>, Self::Error> { Ok(SimStorageReader) }
    fn open_write<'a>(&'a mut self, _path: StoragePath<'_>) -> Result<Self::Writer<'a>, Self::Error> { Ok(SimStorageWriter) }
    fn exists(&mut self, _path: StoragePath<'_>) -> Result<bool, Self::Error> { Ok(false) }
    fn remove(&mut self, _path: StoragePath<'_>) -> Result<(), Self::Error> { Ok(()) }
    fn mkdir(&mut self, _path: StoragePath<'_>) -> Result<(), Self::Error> { Ok(()) }
    fn rename(&mut self, _from: StoragePath<'_>, _to: StoragePath<'_>) -> Result<(), Self::Error> { Ok(()) }
}

pub struct SimInput {
    pub pending_events: std::vec::Vec<InputEvent>,
}
impl InputSource for SimInput {
    type Error = MockError;
    fn poll(&mut self, _now: MonotonicInstant) -> Result<(), Self::Error> { Ok(()) }
    fn button_state(&self, _index: u8) -> bool { false }
}

pub struct SimPower;
impl PowerController for SimPower {
    type Error = MockError;
    fn hold_power_rails(&mut self) -> Result<(), Self::Error> { Ok(()) }
    fn enter_sleep(&mut self) -> Result<(), Self::Error> { Ok(()) }
    fn reset(&mut self) -> Result<(), Self::Error> { Ok(()) }
    fn prevent_sleep(&mut self, _prevent: bool) -> Result<(), Self::Error> { Ok(()) }
}

pub struct SimClock;
impl Clock for SimClock {
    type Error = MockError;
    fn now_monotonic(&self) -> MonotonicInstant { MonotonicInstant(0) }
    fn sleep(&mut self, _duration: Duration) -> Result<(), Self::Error> { Ok(()) }
    fn wall_clock(&mut self) -> Result<Option<WallClockTime>, Self::Error> { Ok(Some(WallClockTime { unix_seconds: 0 })) }
    fn set_wall_clock(&mut self, _value: WallClockTime) -> Result<(), Self::Error> { Ok(()) }
}

pub struct SimBattery;
impl BatteryGauge for SimBattery {
    type Error = MockError;
    fn status(&mut self) -> Result<BatteryStatus, Self::Error> { Ok(BatteryStatus { percent: 100, charging: false, millivolts: 4200 }) }
}

pub struct SimFrontlight;
impl Frontlight for SimFrontlight {
    type Error = MockError;
    fn set_brightness_percent(&mut self, _value: u8) -> Result<(), Self::Error> { Ok(()) }
    fn brightness_percent(&self) -> u8 { 0 }
    fn supports_warmth(&self) -> bool { false }
    fn set_warmth_percent(&mut self, _value: u8) -> Result<(), Self::Error> { Ok(()) }
    fn warmth_percent(&self) -> u8 { 0 }
}

pub struct SimNetwork;
impl NetworkInterface for SimNetwork {
    type Error = MockError;
    fn is_available(&self) -> bool { false }
    fn poll(&mut self) -> Result<(), Self::Error> { Ok(()) }
    fn execute_http<S: HttpResponseSink>(&mut self, _request: &HttpRequest<'_>, _sink: &mut S) -> Result<HttpResponseHead, Self::Error> {
        Err(MockError)
    }
}

pub struct SimTouch;
impl TouchSource for SimTouch {
    type Error = MockError;
    fn poll(&mut self, _now: MonotonicInstant) -> Result<Option<reader_os::hal::input::RawTouchSample>, Self::Error> { Ok(None) }
}

pub struct SimFirmware;
impl FirmwareUpdater for SimFirmware {
    type Error = MockError;
    fn stage(&mut self, _image: &FirmwareImageSource) -> Result<(), Self::Error> { Ok(()) }
    fn commit_boot_target(&mut self) -> Result<(), Self::Error> { Ok(()) }
}

pub struct SimulatorDevice {
    descriptor: DeviceDescriptor,
}

impl SimulatorDevice {
    pub fn new(width: u16, height: u16) -> Self {
        let framebuffer_bytes = width as usize * height as usize; // Grayscale8 takes 1 byte per pixel
        let mut capabilities = CapabilitySet::empty();
        capabilities.insert(Capability::Grayscale);
        capabilities.insert(Capability::Touch);
        Self {
            descriptor: DeviceDescriptor {
                id: "simulator",
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
                    removable: false,
                    supports_directories: true,
                    supports_atomic_rename: true,
                    filesystem_name: "simulator",
                },
                memory: MemoryProfile {
                    total_ram_bytes: 64 * 1024 * 1024,
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

impl Device for SimulatorDevice {
    type Display = SimDisplay;
    type Storage = SimStorage;
    type Input = SimInput;
    type Power = SimPower;
    type Clock = SimClock;
    type Battery = SimBattery;
    type Frontlight = SimFrontlight;
    type Network = SimNetwork;
    type Touch = SimTouch;
    type Firmware = SimFirmware;

    fn descriptor(&self) -> DeviceDescriptor {
        self.descriptor.clone()
    }

    fn into_services(self) -> DeviceServices<Self> {
        DeviceServices {
            display: SimDisplay { width: self.descriptor.display.width, height: self.descriptor.display.height, fb_bytes: vec![0; self.descriptor.display.framebuffer_bytes] },
            storage: SimStorage,
            input: SimInput { pending_events: Vec::new() },
            power: SimPower,
            clock: SimClock,
            battery: SimBattery,
            frontlight: SimFrontlight,
            network: SimNetwork,
            touch: SimTouch,
            firmware: SimFirmware,
        }
    }
}
