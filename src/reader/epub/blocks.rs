use alloc::{string::String, vec::Vec};

use crate::graphics::geometry::Rect;
use crate::graphics::text::TextStyle;
use super::html::PageLink;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct StoredWord { pub text: String, pub style: TextStyle }

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TextBlock { pub words: Vec<StoredWord>, pub link: Option<PageLink> }

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ImageBlock { pub resource_path: String, pub target: Rect, pub preserve_aspect: bool }

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct HorizontalRule { pub thickness: u16 }

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Block { Text(TextBlock), Image(ImageBlock), Rule(HorizontalRule), Spacer { pixels: u16 } }

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FootnoteEntry { pub id: String, pub text: String }

/// Chunked bump-style string storage used to avoid many small allocations during parsing.
pub struct WordStore {
    pub chunk_size: usize,
    pub used_bytes: usize,
}

impl Default for WordStore {
    fn default() -> Self {
        Self::new()
    }
}

impl WordStore {
    pub const DEFAULT_CHUNK_SIZE: usize = 2048;
    pub const fn new() -> Self { Self { chunk_size: Self::DEFAULT_CHUNK_SIZE, used_bytes: 0 } }
}
