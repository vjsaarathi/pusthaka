use alloc::{string::String, vec::Vec};

use crate::graphics::geometry::Point;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum DocumentFormat { Epub, Xtc, Txt, Bmp }

#[derive(Clone, Debug, Eq, PartialEq, Default)]
pub struct BookMetadata {
    pub title: String,
    pub authors: String,
    pub language: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ResourceId(pub u16);

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Resource {
    pub id: ResourceId,
    pub href: String,
    pub media_type: String,
    pub compressed_size: u32,
    pub uncompressed_size: u32,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SpineItem {
    pub href: String,
    pub cumulative_size: u32,
    pub toc_index: Option<u16>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TocEntry {
    pub title: String,
    pub href: String,
    pub anchor: String,
    pub level: u8,
    pub spine_index: u16,
}

/// Stable reading position intended to survive re-pagination and device changes.
#[derive(Clone, Debug, PartialEq)]
pub struct ReaderPosition {
    pub spine_index: u16,
    pub page_number: u16,
    pub total_pages: u16,
    pub visible_text_offset: Option<u32>,
    pub paragraph_index: Option<u16>,
    pub percentage: f32,
    pub point: Option<Point>,
}

/// Runtime document descriptor.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DocumentDescriptor {
    pub path: String,
    pub format: DocumentFormat,
    pub metadata: BookMetadata,
    pub spine: Vec<SpineItem>,
    pub toc: Vec<TocEntry>,
}
