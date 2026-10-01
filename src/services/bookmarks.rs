use alloc::{string::String, vec::Vec};
use crate::hal::storage::{StorageBackend, StoragePath, ReadFile, WriteFile};

#[derive(Clone, Debug, Eq, PartialEq, rkyv::Archive, rkyv::Serialize, rkyv::Deserialize)]
pub struct BookmarkEntry {
    pub book_path: String,
    pub paragraph_index: u32,
    pub text_snippet: String,
    pub timestamp: u64,
}

#[derive(Clone, Debug, Eq, PartialEq, rkyv::Archive, rkyv::Serialize, rkyv::Deserialize)]
pub struct BookmarkStoreData {
    pub bookmarks: Vec<BookmarkEntry>,
}

impl Default for BookmarkStoreData {
    fn default() -> Self {
        Self { bookmarks: Vec::new() }
    }
}

pub struct BookmarkEngine<'a, S: StorageBackend> {
    pub storage: &'a mut S,
    pub path: &'static str,
}

impl<'a, S: StorageBackend> BookmarkEngine<'a, S> {
    pub fn new(storage: &'a mut S, path: &'static str) -> Self {
        Self { storage, path }
    }

    pub fn load(&mut self) -> Result<BookmarkStoreData, S::Error> {
        let mut file = self.storage.open_read(StoragePath(self.path))?;
        let size = file.size()? as usize;
        let mut buf = alloc::vec![0; size];
        file.read(&mut buf)?;
        
        let data: BookmarkStoreData = unsafe {
            rkyv::from_bytes_unchecked(&buf).map_err(|_: rkyv::rancor::Error| panic!())?
        };
        Ok(data)
    }

    pub fn save(&mut self, data: &BookmarkStoreData) -> Result<(), S::Error> {
        let bytes = rkyv::to_bytes::<rkyv::rancor::Error>(data).unwrap();
        let mut file = self.storage.open_write(StoragePath(self.path))?;
        file.write(&bytes)?;
        file.flush()?;
        Ok(())
    }
    
    pub fn add_bookmark(&mut self, entry: BookmarkEntry) -> Result<(), S::Error> {
        let mut data = self.load().unwrap_or_default();
        data.bookmarks.push(entry);
        self.save(&data)
    }
}
