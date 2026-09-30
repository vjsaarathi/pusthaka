use alloc::{string::String, vec::Vec};

use crate::reader::document::{BookMetadata, DocumentFormat};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LibraryEntry {
    pub path: String,
    pub title: String,
    pub author: String,
    pub size_bytes: u64,
    pub format: DocumentFormat,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum LibrarySort { Title, Author, RecentlyRead }

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RecentBook {
    pub path: String,
    pub title: String,
    pub author: String,
    pub cover_path: String,
}

pub struct LibraryIndex { pub entries: Vec<LibraryEntry> }

impl Default for LibraryIndex {
    fn default() -> Self {
        Self::new()
    }
}

impl LibraryIndex {
    pub fn new() -> Self { Self { entries: Vec::new() } }

    pub fn rebuild(&mut self) -> crate::Result<()> { Ok(()) }

    pub fn sort(&mut self, _by: LibrarySort) {}

    pub fn search(&self, _query: &str) -> Vec<&LibraryEntry> { Vec::new() }

    pub fn metadata(entry: &LibraryEntry) -> BookMetadata {
        BookMetadata {
            title: entry.title.clone(),
            authors: entry.author.clone(),
            language: String::from("en"),
        }
    }
}

pub struct RecentBooksStore;

impl RecentBooksStore {
    pub const MAX_ENTRIES: usize = 10;
    pub fn load(&mut self) -> crate::Result<Vec<RecentBook>> { Ok(Vec::new()) }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_library() {
        let mut index = LibraryIndex::new();
        assert!(index.rebuild().is_ok());
        index.sort(LibrarySort::Title);
        assert!(index.search("a").is_empty());

        let entry = LibraryEntry {
            path: String::from(""),
            title: String::from("T"),
            author: String::from("A"),
            size_bytes: 0,
            format: crate::reader::document::DocumentFormat::Epub,
        };
        let meta = LibraryIndex::metadata(&entry);
        assert_eq!(meta.title, "T");

        let mut recent = RecentBooksStore;
        assert!(recent.load().unwrap().is_empty());
    }
}
