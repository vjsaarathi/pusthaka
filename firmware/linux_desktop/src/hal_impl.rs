use reader_os::hal::storage::{StorageBackend, StoragePath, ReadFile, WriteFile};
use reader_os::hal::network::{NetworkBackend, ConnectionState, SecurityType};
use reader_os::hal::battery::{BatteryBackend, BatteryMetrics};
use reader_os::hal::power::PowerBackend;
use reader_os::hal::clock::{RtcBackend, UnixTimestamp};
use reader_os::hal::display::EinkDisplay;
use reader_os::hal::frontlight::Frontlight;
use reader_os::hal::input::{InputSource, InputEvent};
use reader_os::hal::touch::TouchSource;
use reader_os::hal::firmware::FirmwareUpdater;
use reader_os::hal::device::{Device, DeviceServices};
use reader_os::hal::capabilities::DeviceDescriptor;

use std::fs::{File, OpenOptions};
use std::io::{Read, Write, Seek, SeekFrom};
use std::time::{SystemTime, UNIX_EPOCH};

pub struct LinuxStorage;
pub struct LinuxFile(File);

impl ReadFile for LinuxFile {
    type Error = std::io::Error;
    fn read(&mut self, buffer: &mut [u8]) -> Result<usize, Self::Error> { self.0.read(buffer) }
    fn seek(&mut self, offset: u64) -> Result<(), Self::Error> { self.0.seek(SeekFrom::Start(offset))?; Ok(()) }
    fn position(&mut self) -> Result<u64, Self::Error> { self.0.stream_position() }
    fn size(&mut self) -> Result<u64, Self::Error> { Ok(self.0.metadata()?.len()) }
    fn close(&mut self) -> Result<(), Self::Error> { Ok(()) }
}

impl WriteFile for LinuxFile {
    type Error = std::io::Error;
    fn write(&mut self, buffer: &[u8]) -> Result<usize, Self::Error> { self.0.write(buffer) }
    fn flush(&mut self) -> Result<(), Self::Error> { self.0.flush() }
    fn close(&mut self) -> Result<(), Self::Error> { Ok(()) }
}

impl StorageBackend for LinuxStorage {
    type Error = std::io::Error;
    type Reader<'a> = LinuxFile;
    type Writer<'a> = LinuxFile;

    fn begin(&mut self) -> Result<(), Self::Error> { Ok(()) }
    fn open_read<'a>(&'a mut self, path: StoragePath<'_>) -> Result<Self::Reader<'a>, Self::Error> {
        Ok(LinuxFile(File::open(path.0)?))
    }
    fn open_write<'a>(&'a mut self, path: StoragePath<'_>) -> Result<Self::Writer<'a>, Self::Error> {
        Ok(LinuxFile(OpenOptions::new().write(true).create(true).truncate(true).open(path.0)?))
    }
    fn exists(&mut self, path: StoragePath<'_>) -> Result<bool, Self::Error> { Ok(std::path::Path::new(path.0).exists()) }
    fn remove(&mut self, path: StoragePath<'_>) -> Result<(), Self::Error> { std::fs::remove_file(path.0) }
    fn mkdir(&mut self, path: StoragePath<'_>) -> Result<(), Self::Error> { std::fs::create_dir_all(path.0) }
    fn rename(&mut self, from: StoragePath<'_>, to: StoragePath<'_>) -> Result<(), Self::Error> { std::fs::rename(from.0, to.0) }
}

pub struct LinuxNetwork;
impl NetworkBackend for LinuxNetwork {
    fn initialize(&mut self) -> Result<(), Self::Error> { Ok(()) }
    type Error = std::io::Error;
    
    fn scan(&mut self) -> Result<Vec<(String, SecurityType, i8)>, Self::Error> { Ok(vec![]) }
    fn connect(&mut self, _ssid: &str, _password: Option<&str>) -> Result<(), Self::Error> { Ok(()) }
    fn disconnect(&mut self) -> Result<(), Self::Error> { Ok(()) }
    fn state(&self) -> Result<ConnectionState, Self::Error> { Ok(ConnectionState::Connected) }
    fn http_get(&mut self, url: &str, response_buffer: &mut [u8]) -> Result<usize, Self::Error> {
        let mut resp = reqwest::blocking::get(url).map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;
        resp.read(response_buffer)
    }
    fn http_post(&mut self, _url: &str, _payload: &[u8], _response_buffer: &mut [u8]) -> Result<usize, Self::Error> { Ok(0) }
}

pub struct LinuxBattery;
impl BatteryBackend for LinuxBattery {
    type Error = std::io::Error;
    fn is_charging(&self) -> Result<bool, Self::Error> { Ok(true) }
    fn level(&self) -> Result<u8, Self::Error> { Ok(100) }
    fn metrics(&self) -> Result<BatteryMetrics, Self::Error> {
        Ok(BatteryMetrics { level_percent: 100, voltage_mv: 4200, current_ma: 500, temperature_c: 30 })
    }
}

pub struct LinuxPower;
impl PowerBackend for LinuxPower {
    type Error = std::io::Error;
    fn sleep(&mut self) -> Result<(), Self::Error> { Ok(()) }
    fn shutdown(&mut self) -> Result<(), Self::Error> { std::process::exit(0) }
    fn reboot(&mut self) -> Result<(), Self::Error> { std::process::exit(0) }
}

pub struct LinuxRtc;
impl RtcBackend for LinuxRtc {
    type Error = std::io::Error;
    fn now(&self) -> Result<UnixTimestamp, Self::Error> {
        Ok(SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs())
    }
    fn set_time(&mut self, _timestamp: UnixTimestamp) -> Result<(), Self::Error> { Ok(()) }
    fn clear_alarm(&mut self) -> Result<(), Self::Error> { Ok(()) }
    fn set_wake_alarm(&mut self, _timestamp: UnixTimestamp) -> Result<(), Self::Error> { Ok(()) }
    
}

pub struct DummyDisplay;
impl EinkDisplay for DummyDisplay {
    type Error = std::io::Error;
    
    
    
    
    
    
    fn sleep(&mut self) -> Result<(), Self::Error> { Ok(()) }
    fn capabilities(&self) -> reader_os::hal::display::DisplayCapabilities { reader_os::hal::display::DisplayCapabilities { grayscale_levels: 16, partial_refresh: true, asynchronous_refresh: false, waveform_control: false } }
    fn refresh(&mut self, _: &reader_os::graphics::framebuffer::FrameBufferView<'_>, _: reader_os::hal::display::RefreshRequest) -> Result<(), Self::Error> { Ok(()) }
    fn is_busy(&self) -> bool { false }
}

pub struct DummyFrontlight;
impl Frontlight for DummyFrontlight {
    type Error = std::io::Error;
    
    
    fn set_brightness_percent(&mut self, _: u8) -> Result<(), Self::Error> { Ok(()) }
    fn brightness_percent(&self) -> u8 { 0 }
    fn supports_warmth(&self) -> bool { false }
    fn set_warmth_percent(&mut self, _: u8) -> Result<(), Self::Error> { Ok(()) }
    fn warmth_percent(&self) -> u8 { 0 }
}

pub struct DummyInput;
impl InputSource for DummyInput {
    type Error = std::io::Error;
    
    
    fn poll(&mut self, _: reader_os::kernel::time::MonotonicInstant) -> Result<(), Self::Error> { Ok(()) }
    fn button_state(&self, _: u8) -> bool { false }
}

pub struct DummyTouch;
impl TouchSource for DummyTouch {
    type Error = std::io::Error;
    
    fn poll(&mut self, _: reader_os::kernel::time::MonotonicInstant) -> Result<Option<reader_os::hal::input::RawTouchSample>, Self::Error> { Ok(None) }
}

pub struct DummyFirmware;
impl FirmwareUpdater for DummyFirmware {
    type Error = std::io::Error;
    
    
    
    
    fn stage(&mut self, _: &reader_os::services::ota::FirmwareImageSource) -> Result<(), Self::Error> { Ok(()) }
    fn commit_boot_target(&mut self) -> Result<(), Self::Error> { Ok(()) }
}

pub struct LinuxDevice;
impl Device for LinuxDevice {
    type Display = DummyDisplay;
    type Storage = LinuxStorage;
    type Input = DummyInput;
    type Power = LinuxPower;
    type Clock = LinuxRtc;
    type Battery = LinuxBattery;
    type Frontlight = DummyFrontlight;
    type Network = LinuxNetwork;
    type Touch = DummyTouch;
    type Firmware = DummyFirmware;

    fn descriptor(&self) -> DeviceDescriptor {
        DeviceDescriptor {
            id: "Linux Desktop",
            display: reader_os::hal::capabilities::DisplayDescriptor { width: 800, height: 600, native_grayscale_levels: 16, framebuffer_bytes: 800*600, has_partial_refresh: true, has_async_refresh: false },
            input: reader_os::hal::capabilities::InputDescriptor { physical_buttons: 0, has_touch: true, has_gesture_input: false },
            storage: reader_os::hal::capabilities::StorageDescriptor { capacity_bytes: 1000000, removable: false, supports_directories: true, supports_atomic_rename: true, filesystem_name: "ext4" },
            memory: reader_os::hal::capabilities::MemoryProfile::MINIMUM,
            capabilities: reader_os::hal::capabilities::CapabilitySet::empty(),
        }
    }

    fn into_services(self) -> DeviceServices<Self> {
        DeviceServices {
            display: DummyDisplay,
            storage: LinuxStorage,
            input: DummyInput,
            power: LinuxPower,
            clock: LinuxRtc,
            battery: LinuxBattery,
            frontlight: DummyFrontlight,
            network: LinuxNetwork,
            touch: DummyTouch,
            firmware: DummyFirmware,
        }
    }
}
