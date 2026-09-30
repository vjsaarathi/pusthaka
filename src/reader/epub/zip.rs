use alloc::string::String;

/// ZIP compression methods accepted by the EPUB engine.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CompressionMethod { Store, Deflate }

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ZipEntry {
    pub path: String,
    pub compressed_size: u32,
    pub uncompressed_size: u32,
    pub compression: CompressionMethod,
    pub local_header_offset: u32,
}

/// Streaming sink used by ZIP extraction so decompressed data never has to be held
/// as one contiguous heap allocation.
pub trait ByteSink {
    type Error;
    fn write(&mut self, bytes: &[u8]) -> Result<usize, Self::Error>;
}

/// TODO: implement EOCD/central-directory scanning and Store/Deflate streaming extraction.
pub struct ZipReader;

impl ZipReader {
    pub fn extract_to_sink<S: ByteSink>(&mut self, _entry: &ZipEntry, _sink: &mut S) -> Result<(), S::Error> {
        Ok(())
    }
}
