/// StarDict format contract (`.ifo`, `.idx`, `.dict`, optional `.dict.dz`).
pub struct StarDictFormat;

impl StarDictFormat {
    pub const VERSION: &'static str = "3.0.0";
    /// TODO: parse key/value IFO metadata without assuming dictionary text encoding beyond the declared format.
    pub fn parse_ifo(_bytes: &[u8]) -> crate::Result<()> { Ok(()) }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct StarDictIndexRecord { pub word_end: usize, pub offset: u32, pub size: u32 }

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct StarDictInfo {
    pub word_count: u32,
    pub idx_file_size: u32,
    pub book_name: alloc::string::String,
    pub same_type_sequence: alloc::string::String,
}
