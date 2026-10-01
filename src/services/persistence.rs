use alloc::{string::String, vec::Vec};
use crate::hal::storage::{StorageBackend, StoragePath, ReadFile, WriteFile};

#[derive(Clone, Copy, Debug, Eq, PartialEq, rkyv::Archive, rkyv::Serialize, rkyv::Deserialize)]
pub enum SleepScreenMode { Dark, Custom, Cover, CoverCustom, Default }

#[derive(Clone, Copy, Debug, Eq, PartialEq, rkyv::Archive, rkyv::Serialize, rkyv::Deserialize)]
pub enum CoverFitMode { Fit, Fill }

#[derive(Clone, Copy, Debug, Eq, PartialEq, rkyv::Archive, rkyv::Serialize, rkyv::Deserialize)]
pub enum OrientationPreference { Portrait, LandscapeClockwise, PortraitInverted, LandscapeCounterClockwise }

#[derive(Clone, Copy, Debug, Eq, PartialEq, rkyv::Archive, rkyv::Serialize, rkyv::Deserialize)]
pub enum RefreshFrequency { EveryPage, Every5Pages, Every10Pages, Every15Pages }

#[derive(Clone, Copy, Debug, Eq, PartialEq, rkyv::Archive, rkyv::Serialize, rkyv::Deserialize)]
pub enum LineSpacing { Tight, Normal, Relaxed, Double }

#[derive(Clone, Debug, Eq, PartialEq, rkyv::Archive, rkyv::Serialize, rkyv::Deserialize)]
pub struct Settings {
    pub font_point_size: u16,
    pub sleep_timeout_minutes: u16,
    pub language: String,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            font_point_size: 14,
            sleep_timeout_minutes: 10,
            language: String::from("en"),
        }
    }
}

pub struct SettingsStore<'a, S: StorageBackend> {
    pub storage: &'a mut S,
    pub path: &'static str,
}

impl<'a, S: StorageBackend> SettingsStore<'a, S> {
    pub fn new(storage: &'a mut S, path: &'static str) -> Self {
        Self { storage, path }
    }

    pub fn load(&mut self) -> Result<Settings, S::Error> {
        let mut file = self.storage.open_read(StoragePath(self.path))?;
        let size = file.size()? as usize;
        let mut buf = alloc::vec![0; size];
        file.read(&mut buf)?;
        
        // Zero-copy deserialization back to Settings struct!
        let settings: Settings = unsafe {
            rkyv::from_bytes_unchecked(&buf).map_err(|_: rkyv::rancor::Error| panic!())?
        };
        Ok(settings)
    }

    pub fn save(&mut self, settings: &Settings) -> Result<(), S::Error> {
        let bytes = rkyv::to_bytes::<rkyv::rancor::Error>(settings).unwrap();
        let mut file = self.storage.open_write(StoragePath(self.path))?;
        file.write(&bytes)?;
        file.flush()?;
        Ok(())
    }
}

#[derive(Clone, Debug, Eq, PartialEq, rkyv::Archive, rkyv::Serialize, rkyv::Deserialize)]
pub struct RuntimeState {
    pub open_book_path: String,
    pub reader_activity_load_count: u8,
}

impl Default for RuntimeState {
    fn default() -> Self {
        Self {
            open_book_path: String::new(),
            reader_activity_load_count: 0,
        }
    }
}
