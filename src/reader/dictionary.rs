use alloc::string::String;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DictionaryInfo {
    pub name: String,
    pub word_count: u32,
    pub idx_file_size: u32,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DictionaryEntry { pub word: String, pub definition: String }

/// StarDict index entry. Numeric fields are big-endian on disk.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct StarDictIndexEntry { pub offset: u32, pub size: u32 }

pub trait Dictionary {
    type Error;
    fn lookup(&mut self, word: &str) -> Result<Option<DictionaryEntry>, Self::Error>;
}

/// Registry discovers dictionaries under the user storage dictionary root.
pub struct DictionaryRegistry { pub active: Option<DictionaryInfo> }

impl DictionaryRegistry {
    /// TODO: scan `/dictionaries/` for `.ifo` sets and validate supported formats.
    pub fn discover(&mut self) -> crate::Result<()> { Ok(()) }
}

/// DictZip random-access decompression state.
pub struct DictZipReader;

impl DictZipReader {
    /// TODO: parse gzip extra fields and map uncompressed offsets to compressed chunks.
    pub fn read_range(&mut self, _offset: u32, _size: u32, _out: &mut [u8]) -> crate::Result<usize> {
        Ok(0)
    }
}

pub struct StarDict;

impl Dictionary for StarDict {
    type Error = crate::CoreError;
    fn lookup(&mut self, _word: &str) -> Result<Option<DictionaryEntry>, Self::Error> {
        Ok(None)
    }
}
