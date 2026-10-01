use alloc::string::String;
use alloc::vec::Vec;

/// The universal, zero-copy, pre-compiled internal format for the E-reader.
/// All input formats (EPUB, PDF, TXT) are ingested into this format.
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Debug)]
pub struct ReaderBook {
    // --- Metadata ---
    pub title: String,
    pub author: String,
    pub language: String,
    
    // --- Assets ---
    /// Raw byte slice of the cover image (e.g. JPEG or PNG data)
    pub cover_image: Option<Vec<u8>>,
    /// Inline images mapped by their internal path/ID (stored as a flat list of key-value pairs)
    pub inline_images: Vec<(String, Vec<u8>)>,
    
    // --- Navigation ---
    pub table_of_contents: Vec<TocEntry>,
    
    // --- Content ---
    /// A flat list of semantic blocks spanning the entire book
    pub blocks: Vec<SemanticBlock>,
}

#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Debug)]
pub struct TocEntry {
    pub title: String,
    /// The nesting depth (0 = root chapter, 1 = sub-chapter, etc.)
    pub level: u8,
    /// Which block index this TOC entry points to
    pub target_block_index: u32,
}

#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Debug)]
pub enum SemanticBlock {
    Paragraph { text: String, spans: Vec<TextSpan> },
    Heading { level: u8, text: String },
    Image { id: String, caption: Option<String> },
    PageBreak,
}

#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Debug)]
pub struct TextSpan {
    pub start_index: u32,
    pub end_index: u32,
    pub style: SpanStyle,
}

#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Debug)]
pub enum SpanStyle {
    Normal,
    Italic,
    Bold,
    Underline,
}
