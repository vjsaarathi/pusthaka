use crate::graphics::geometry::Rect;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ThemeId { Classic, Lyra, Lyra3Covers, RoundedRaff }

/// Geometry/spacing metrics. New layouts must add named metrics rather than scattering pixels through activities.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ThemeMetrics {
    pub header_height: u16,
    pub header_title_margin: u16,
    pub progress_bar_height: u16,
    pub scrollbar_width: u16,
    pub cover_grid_columns: u8,
    pub cover_grid_rows: u8,
    pub status_bar_height: u16,
    pub button_hints_height: u16,
    pub list_row_height: u16,
    pub list_padding: u16,
}

/// Rendering strategy for the UI.
pub trait Theme {
    fn id(&self) -> ThemeId;
    fn metrics(&self) -> ThemeMetrics;
    fn screen_safe_area(&self, screen: Rect) -> Rect;
    /// TODO: implement all common UI primitives through the renderer, keeping visual policy out of activities.
    fn draw_header(&self, renderer: &mut crate::graphics::renderer::Renderer<'_>, title: &str, show_back: bool) {
        let _ = (renderer, title, show_back);
    }
}
