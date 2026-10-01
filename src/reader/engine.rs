use crate::reader::document::ReaderPosition;
use crate::reader::layout::PageLayout;
use crate::graphics::text::FontProvider;

pub trait DocumentEngine {
    /// Initializes the engine, parsing only essential metadata (no full loads).
    fn load(&mut self) -> crate::Result<()>;
    fn total_pages(&self) -> u16;
    /// Computes and returns the layout data for a given page on demand.
    fn get_page_layout(&mut self, font_provider: &mut dyn FontProvider<Error = crate::CoreError>, position: &ReaderPosition, viewport_width: u16, viewport_height: u16) -> crate::Result<PageLayout>;
}
