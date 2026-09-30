use alloc::string::String;

/// Disk cache version for the metadata serializer.
pub const META_CACHE_VERSION: u32 = 1;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BookMetadataCacheHeader {
    pub version: u32,
    pub title: String,
    pub author: String,
    pub language: String,
    pub spine_count: u16,
}

/// Existing progress cache compatibility shape.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ProgressCache {
    pub spine_index: u16,
    pub page_number: u16,
    pub total_page_count: Option<u16>,
    pub visible_text_offset: Option<u32>,
}

impl ProgressCache {
    /// TODO: accept the legacy 4-, 6-, and 10-byte encodings exactly.
    pub fn decode(_bytes: &[u8]) -> crate::Result<Self> {
        Ok(Self { spine_index: 0, page_number: 0, total_page_count: None, visible_text_offset: None })
    }

    /// TODO: write the canonical current representation.
    pub fn encode(&self, _out: &mut [u8]) -> crate::Result<usize> { Ok(0) }
}

pub struct BookCache;

impl BookCache {
    /// TODO: derive the deterministic per-book cache path from the canonical book path.
    pub fn path_for(_book_path: &str) -> String { String::new() }
}
