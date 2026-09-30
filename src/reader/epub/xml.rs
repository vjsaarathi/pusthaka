/// Generic incremental XML parser contract.
///
/// Concrete parsers must hold only the state required by their document grammar and must
/// accept input in arbitrary chunk boundaries.
pub trait StreamingXmlParser {
    type Error;
    type Output;

    fn feed(&mut self, bytes: &[u8]) -> Result<bool, Self::Error>;
    fn finish(&mut self) -> Result<Self::Output, Self::Error>;
}

/// Extracts the EPUB rootfile path from `META-INF/container.xml`.
pub struct ContainerParser { pub rootfile_path: Option<alloc::string::String> }

impl Default for ContainerParser {
    fn default() -> Self {
        Self::new()
    }
}

impl ContainerParser { pub fn new() -> Self { Self { rootfile_path: None } } }

/// Extracts metadata, manifest, spine and guide data from an OPF package.
pub struct ContentOpfParser;

/// Extracts EPUB3 navigation-document TOC entries.
pub struct TocNavParser;

/// Extracts EPUB2 NCX TOC entries.
pub struct TocNcxParser;
