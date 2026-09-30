use alloc::string::String;

use super::blocks::Block;
use super::cache::BookMetadataCacheHeader;
use super::css::CssParser;
use super::html::ChapterHtmlParser;
use super::zip::ZipReader;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum EpubLoadState { Closed, Opened, MetadataCached, ParsingContainer, ParsingOpf, ParsingToc, ParsingCss, Ready, Failed }

/// Central EPUB coordinator. All expensive operations remain streamable/resumable.
pub struct EpubEngine {
    pub path: String,
    pub state: EpubLoadState,
    pub zip: ZipReader,
    pub metadata: Option<BookMetadataCacheHeader>,
    pub html_parser: Option<ChapterHtmlParser>,
    pub css_parser: Option<CssParser>,
    pub active_blocks: alloc::vec::Vec<Block>,
}

impl EpubEngine {
    pub fn load(&mut self) -> crate::Result<()> {
        Ok(())
    }

    pub fn load_step(&mut self, _budget: crate::kernel::WorkBudget) -> crate::Result<crate::kernel::PollResult> {
        Ok(crate::kernel::PollResult::Complete)
    }

    pub fn extract_cover(&mut self) -> crate::Result<()> {
        Ok(())
    }
}
