use alloc::string::ToString;
use alloc::vec::Vec;

use crate::graphics::text::FontProvider;
use crate::reader::document::ReaderPosition;
use crate::reader::engine::DocumentEngine;
use crate::reader::layout::{PageLayout, LayoutLine};

pub struct TxtEngine<'a> {
    pub raw_data: &'a [u8],
    /// Stores (start_offset, end_offset) for lazy loading pages without allocating strings
    pub page_offsets: Vec<(usize, usize)>,
}

impl<'a> TxtEngine<'a> {
    pub fn new(raw_data: &'a [u8]) -> Self {
        Self {
            raw_data,
            page_offsets: Vec::new(),
        }
    }
}

impl<'a> DocumentEngine for TxtEngine<'a> {
    fn load(&mut self) -> crate::Result<()> {
        // Lazy pagination: scan through the raw bytes to find line breaks or chunk boundaries.
        // We do NOT allocate strings here. We only store the byte offsets.
        // This keeps memory overhead to just a few bytes per page.
        let mut start = 0;
        let chunk_size = 512; // Arbitrary chunk size for text
        
        while start < self.raw_data.len() {
            let mut end = start + chunk_size;
            if end >= self.raw_data.len() {
                end = self.raw_data.len();
            } else {
                // Try to find a newline to break neatly
                while end > start && self.raw_data[end] != b'\n' {
                    end -= 1;
                }
                if end == start {
                    end = start + chunk_size; // Force break if no newline
                } else {
                    end += 1; // Include the newline
                }
            }
            self.page_offsets.push((start, end));
            start = end;
        }

        if self.page_offsets.is_empty() {
            self.page_offsets.push((0, 0));
        }
        Ok(())
    }

    fn total_pages(&self) -> u16 {
        self.page_offsets.len() as u16
    }

    fn get_page_layout(&mut self, _font_provider: &mut dyn FontProvider<Error = crate::CoreError>, position: &ReaderPosition, _viewport_width: u16, _viewport_height: u16) -> crate::Result<PageLayout> {
        let page_idx = (position.page_number as usize).saturating_sub(1).min(self.page_offsets.len().saturating_sub(1));
        let (start, end) = self.page_offsets[page_idx];
        
        // Extract just the needed chunk from raw data and convert to string
        let chunk = &self.raw_data[start..end];
        let text = core::str::from_utf8(chunk).unwrap_or("Encoding Error").to_string();

        // Create a layout definition mapping the text to screen coordinates.
        // In a full implementation, we'd calculate real font metrics here.
        let line = LayoutLine {
            text,
            words: alloc::vec![],
            baseline: 100,
            height: 20,
        };

        Ok(PageLayout {
            lines: alloc::vec![line],
            height: 20,
        })
    }
}
