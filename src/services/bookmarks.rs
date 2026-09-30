use alloc::string::String;

use crate::reader::document::ReaderPosition;

#[derive(Clone, Debug, PartialEq)]
pub struct BookmarkEntry {
    pub xpath: String,
    pub summary: String,
    pub percentage: f32,
    pub computed_spine_index: u16,
    pub computed_chapter_page_count: u16,
    pub computed_chapter_progress: f32,
    pub visible_text_offset: Option<u32>,
    pub position: Option<ReaderPosition>,
}

pub struct BookmarkStore;

impl BookmarkStore {
    /// TODO: derive a stable sanitized per-book filename and read/write bookmark JSON.
    pub fn save(_book_path: &str, _bookmarks: &[BookmarkEntry]) -> crate::Result<()> { Ok(()) }
    pub fn load(_book_path: &str) -> crate::Result<alloc::vec::Vec<BookmarkEntry>> { Ok(alloc::vec::Vec::new()) }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bookmarks_save_load() {
        assert!(BookmarkStore::save("test.epub", &[]).is_ok());
        let bookmarks = BookmarkStore::load("test.epub").unwrap();
        assert!(bookmarks.is_empty());
    }
}
