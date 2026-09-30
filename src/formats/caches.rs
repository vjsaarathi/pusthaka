/// Versioned metadata cache wire format.
pub struct MetaCacheFormat;

impl MetaCacheFormat {
    pub const VERSION: u32 = 1;
    /// TODO: match the documented header/spine/TOC layout exactly and reject truncation/overflow.
    pub fn decode(_bytes: &[u8]) -> crate::Result<()> { Ok(()) }
}

/// Compact index entry used while resolving OPF manifest IDs without keeping the complete manifest in RAM.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ItemIndexEntry {
    pub hash: u32,
    pub length: u16,
    pub offset: u32,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SpineCacheEntry {
    pub href_length: u16,
    pub cumulative_size: u32,
    pub toc_index: i16,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct TocCacheEntry {
    pub title_length: u16,
    pub href_length: u16,
    pub anchor_length: u16,
    pub level: u8,
    pub spine_index: u16,
}

/// Backwards-compatible progress cache format.
pub struct ProgressFormat;

impl ProgressFormat {
    pub fn supported_lengths() -> &'static [usize] { &[4, 6, 10] }
    /// TODO: decode 4-, 6-, and 10-byte historical variants.
    pub fn decode(_bytes: &[u8]) -> crate::Result<()> { Ok(()) }
}

pub struct SectionCacheFormat;
impl SectionCacheFormat {
    /// TODO: define and version the on-disk page/section layout only after the block/page structures stabilize.
    pub fn version() -> u32 { 1 }
}
