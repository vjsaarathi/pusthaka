use alloc::{string::String, vec::Vec};
use crate::hal::storage::{StorageBackend, StoragePath, ReadFile, WriteFile};

#[derive(Clone, Debug, Eq, PartialEq, rkyv::Archive, rkyv::Serialize, rkyv::Deserialize)]
pub struct LibraryEntry {
    pub path: String,
    pub title: String,
    pub author: String,
    pub size_bytes: u64,
}

#[derive(Clone, Debug, Eq, PartialEq, rkyv::Archive, rkyv::Serialize, rkyv::Deserialize)]
pub struct LibraryIndex {
    pub entries: Vec<LibraryEntry>,
}

impl Default for LibraryIndex {
    fn default() -> Self {
        Self { entries: Vec::new() }
    }
}

pub struct LibraryEngine<'a, S: StorageBackend> {
    pub storage: &'a mut S,
    pub cache_path: &'static str,
}

impl<'a, S: StorageBackend> LibraryEngine<'a, S> {
    pub fn new(storage: &'a mut S, cache_path: &'static str) -> Self {
        Self { storage, cache_path }
    }

    pub fn load_index(&mut self) -> Result<LibraryIndex, S::Error> {
        let mut file = self.storage.open_read(StoragePath(self.cache_path))?;
        let size = file.size()? as usize;
        let mut buf = alloc::vec![0; size];
        file.read(&mut buf)?;
        
        let index: LibraryIndex = unsafe {
            rkyv::from_bytes_unchecked(&buf).map_err(|_: rkyv::rancor::Error| panic!())?
        };
        Ok(index)
    }

    pub fn save_index(&mut self, index: &LibraryIndex) -> Result<(), S::Error> {
        let bytes = rkyv::to_bytes::<rkyv::rancor::Error>(index).unwrap();
        let mut file = self.storage.open_write(StoragePath(self.cache_path))?;
        file.write(&bytes)?;
        file.flush()?;
        Ok(())
    }
}
