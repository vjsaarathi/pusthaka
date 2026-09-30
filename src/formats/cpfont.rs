/// `.cpfont` version 4 format contract.
pub struct CpfontFormat;

impl CpfontFormat {
    pub const MAGIC: [u8; 4] = *b"CPFN";
    pub const VERSION: u8 = 4;

    /// Header occupies magic + version + style count.
    pub const HEADER_BYTES: usize = 6;

    /// TODO: validate the header and expose style/glyph/kerning/ligature/bitmap offsets without loading the file.
    pub fn validate_header(_bytes: &[u8]) -> crate::Result<()> { Ok(()) }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CpfontHeader { pub version: u8, pub style_count: u8 }

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CpfontStyleHeader {
    pub glyph_count: u16,
    pub kern_count: u16,
    pub ligature_count: u16,
    pub glyph_index_offset: u32,
    pub kern_offset: u32,
    pub ligature_offset: u32,
    pub bitmap_offset: u32,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CpfontGlyphInterval {
    pub start_codepoint: u32,
    pub count: u16,
    pub glyph_data_offset: u32,
}
