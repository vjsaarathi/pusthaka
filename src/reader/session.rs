use alloc::string::String;
use alloc::vec::Vec;

use super::document::ReaderPosition;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ReaderOverlay { None, Toolbar, Contents, TextSettings, More }

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SavedPosition { pub spine_index: u16, pub page_number: u16 }

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum EndOfBookAction { NextBook, Home, ReadAgain }

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PageDirection { Backward, Forward }

/// Runtime state for both EPUB and XTC readers. Format-specific page engines remain separate.
pub struct ReaderSession {
    pub book_path: String,
    pub overlay: ReaderOverlay,
    pub position: ReaderPosition,
    pub automatic_page_turn: bool,
    pub pages_until_full_refresh: u16,
    pub forced_refresh_pending: bool,
    pub pending_page_jump: Option<u16>,
    pub pending_anchor: Option<String>,
    pub bookmarks: Vec<super::super::services::BookmarkEntry>,
    pub footnote_stack: Vec<SavedPosition>,
    pub max_footnote_depth: usize,
}

impl ReaderSession {
    pub const MAX_FOOTNOTE_DEPTH: usize = 3;

    pub fn push_footnote_position(&mut self, position: SavedPosition) -> bool {
        if self.footnote_stack.len() >= self.max_footnote_depth { return false; }
        self.footnote_stack.push(position);
        true
    }
}
