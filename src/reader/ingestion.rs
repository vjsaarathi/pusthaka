#[cfg(feature = "std")]
extern crate std;
use alloc::string::ToString;
use alloc::vec::Vec;
use crate::reader::format::{ReaderBook, TocEntry, SemanticBlock};
use crate::hal::storage::{StorageBackend, StoragePath, WriteFile};

pub enum IngestionFormat {
    Txt,
    Epub,
    Pdf,
}

#[derive(Debug)]
pub enum IngestionError {
    InvalidFormat,
    ParseError,
    SerializationError,
    StorageError,
}

/// The Engine responsible for compiling raw documents into our universal binary format.
pub struct IngestionEngine;

impl IngestionEngine {
    /// Imports a raw text or epub document and returns the parsed ReaderBook struct in memory.
    pub fn import_in_memory(format: IngestionFormat, raw_data: &[u8]) -> Result<ReaderBook, IngestionError> {
        match format {
            IngestionFormat::Txt => Self::parse_txt(raw_data),
            #[cfg(feature = "ingestion")]
            IngestionFormat::Epub => Self::parse_epub(raw_data),
            #[cfg(feature = "ingestion")]
            IngestionFormat::Pdf => Self::parse_pdf(raw_data),
            #[cfg(not(feature = "ingestion"))]
            _ => Err(IngestionError::InvalidFormat),
        }
    }

    /// Imports a raw document, compiles it, and saves the binary cache directly to the storage engine.
    pub fn import_and_save<S: StorageBackend>(
        storage: &mut S,
        format: IngestionFormat,
        raw_data: &[u8],
        save_path: &str,
    ) -> Result<(), IngestionError> {
        let book = Self::import_in_memory(format, raw_data)?;
        
        let bytes = rkyv::to_bytes::<rkyv::rancor::Error>(&book)
            .map_err(|_| IngestionError::SerializationError)?;

        let mut file = storage.open_write(StoragePath(save_path))
            .map_err(|_| IngestionError::StorageError)?;
            
        file.write(&bytes).map_err(|_| IngestionError::StorageError)?;
        file.flush().map_err(|_| IngestionError::StorageError)?;
        file.close().map_err(|_| IngestionError::StorageError)?;

        Ok(())
    }

    fn parse_txt(raw_data: &[u8]) -> Result<ReaderBook, IngestionError> {
        let text = core::str::from_utf8(raw_data).map_err(|_| IngestionError::ParseError)?;
        let mut blocks = Vec::new();
        let mut toc = Vec::new();
        
        for line in text.lines() {
            let line = line.trim();
            if line.is_empty() { continue; }
            if line.starts_with('#') {
                let heading = line.trim_start_matches('#').trim().to_string();
                blocks.push(SemanticBlock::Heading { level: 1, text: heading.clone() });
                toc.push(TocEntry { title: heading, level: 0, target_block_index: (blocks.len() - 1) as u32 });
            } else {
                blocks.push(SemanticBlock::Paragraph { text: line.to_string(), spans: Vec::new() });
            }
        }
        
        Ok(ReaderBook { title: "Text Doc".to_string(), author: "".to_string(), language: "en".to_string(), cover_image: None, inline_images: Vec::new(), table_of_contents: toc, blocks })
    }

    #[cfg(feature = "ingestion")]
    fn parse_epub(raw_data: &[u8]) -> Result<ReaderBook, IngestionError> {
        use std::io::{Cursor, Read};
        use quick_xml::events::Event;
        use quick_xml::reader::Reader;

        let reader = Cursor::new(raw_data);
        let mut archive = zip::ZipArchive::new(reader).map_err(|_| IngestionError::ParseError)?;
        
        let mut blocks = Vec::new();
        let mut toc = Vec::new();
        
        for i in 0..archive.len() {
            let mut file = archive.by_index(i).map_err(|_| IngestionError::ParseError)?;
            if file.name().ends_with(".html") || file.name().ends_with(".xhtml") {
                let mut html_content = String::new();
                file.read_to_string(&mut html_content).unwrap_or_default();
                
                let mut xml_reader = Reader::from_str(&html_content);
                xml_reader.trim_text(true);
                
                let mut buf = Vec::new();
                let mut current_text = String::new();
                let mut in_p = false;
                let mut in_h = false;
                
                loop {
                    match xml_reader.read_event_into(&mut buf) {
                        Ok(Event::Start(ref e)) => {
                            let tag = e.name();
                            let tag_ref = tag.as_ref();
                            if tag_ref == b"p" || tag_ref == b"div" {
                                in_p = true;
                                current_text.clear();
                            } else if tag_ref == b"h1" || tag_ref == b"h2" || tag_ref == b"h3" {
                                in_h = true;
                                current_text.clear();
                            }
                        },
                        Ok(Event::Text(e)) => {
                            if in_p || in_h {
                                if let Ok(txt) = e.unescape() {
                                    current_text.push_str(&txt);
                                    current_text.push(' ');
                                }
                            }
                        },
                        Ok(Event::End(ref e)) => {
                            let tag = e.name();
                            let tag_ref = tag.as_ref();
                            if tag_ref == b"p" || tag_ref == b"div" {
                                let trimmed = current_text.trim();
                                if !trimmed.is_empty() {
                                    blocks.push(SemanticBlock::Paragraph { text: trimmed.to_string(), spans: Vec::new() });
                                }
                                in_p = false;
                                current_text.clear();
                            } else if tag_ref == b"h1" || tag_ref == b"h2" || tag_ref == b"h3" {
                                let trimmed = current_text.trim();
                                if !trimmed.is_empty() {
                                    let txt = trimmed.to_string();
                                    blocks.push(SemanticBlock::Heading { level: 1, text: txt.clone() });
                                    toc.push(TocEntry {
                                        title: txt,
                                        level: 0,
                                        target_block_index: (blocks.len() - 1) as u32,
                                    });
                                }
                                in_h = false;
                                current_text.clear();
                            }
                        },
                        Ok(Event::Eof) => break,
                        _ => (),
                    }
                    buf.clear();
                }
            }
        }
        
        Ok(ReaderBook {
            title: "EPUB Document".to_string(),
            author: "Unknown".to_string(),
            language: "en".to_string(),
            cover_image: None,
            inline_images: Vec::new(),
            table_of_contents: toc,
            blocks,
        })
    }

    #[cfg(feature = "ingestion")]
    fn parse_pdf(raw_data: &[u8]) -> Result<ReaderBook, IngestionError> {
        // Robust PDF extraction (using `pdf-extract`)
        let text = pdf_extract::extract_text_from_mem(raw_data).map_err(|_| IngestionError::ParseError)?;
        let mut blocks = Vec::new();
        
        let mut current_paragraph = String::new();
        for line in text.lines() {
            let trimmed = line.trim();
            if trimmed.is_empty() {
                if !current_paragraph.is_empty() {
                    blocks.push(SemanticBlock::Paragraph { text: current_paragraph.clone(), spans: Vec::new() });
                    current_paragraph.clear();
                }
            } else {
                if !current_paragraph.is_empty() {
                    current_paragraph.push(' ');
                }
                current_paragraph.push_str(trimmed);
            }
        }
        if !current_paragraph.is_empty() {
            blocks.push(SemanticBlock::Paragraph { text: current_paragraph, spans: Vec::new() });
        }
        
        Ok(ReaderBook { title: "PDF Document".to_string(), author: "Unknown".to_string(), language: "en".to_string(), cover_image: None, inline_images: Vec::new(), table_of_contents: Vec::new(), blocks })
    }
}
