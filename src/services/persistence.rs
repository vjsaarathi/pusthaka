use alloc::{string::String, vec::Vec};

/// Configuration enum values are represented explicitly so serialization remains stable.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SleepScreenMode { Dark, Custom, Cover, CoverCustom, Default }

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CoverFitMode { Fit, Fill }

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum OrientationPreference { Portrait, LandscapeClockwise, PortraitInverted, LandscapeCounterClockwise }

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RefreshFrequency { EveryPage, Every5Pages, Every10Pages, Every15Pages }

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum LineSpacing { Tight, Normal, Relaxed, Double }

/// Persistent user preferences. Keep field names stable once persisted; migrations must be explicit.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Settings {
    pub sleep_screen: SleepScreenMode,
    pub sleep_screen_cover_mode: CoverFitMode,
    pub orientation: OrientationPreference,
    pub refresh_frequency: RefreshFrequency,
    pub status_bar_chapter_page_count: bool,
    pub status_bar_title: bool,
    pub status_bar_battery: bool,
    pub status_bar_clock: bool,
    pub font_family: u16,
    pub font_point_size: u16,
    pub line_spacing: LineSpacing,
    pub paragraph_alignment: crate::graphics::text::TextAlignment,
    pub screen_margin: u16,
    pub word_spacing_percent: u16,
    pub paragraph_indent: i16,
    pub css_enabled: bool,
    pub touch_reader_controls: bool,
    pub sleep_timeout_minutes: u16,
    pub front_button_back: u8,
    pub front_button_confirm: u8,
    pub front_button_left: u8,
    pub front_button_right: u8,
    pub side_button_page_forward: bool,
    pub web_server_enabled: bool,
    pub koreader_sync_enabled: bool,
    pub koreader_sync_url: String,
    pub ui_theme: u8,
    pub language: String,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            sleep_screen: SleepScreenMode::Dark,
            sleep_screen_cover_mode: CoverFitMode::Fit,
            orientation: OrientationPreference::Portrait,
            refresh_frequency: RefreshFrequency::Every15Pages,
            status_bar_chapter_page_count: true,
            status_bar_title: true,
            status_bar_battery: true,
            status_bar_clock: false,
            font_family: 0,
            font_point_size: 14,
            line_spacing: LineSpacing::Normal,
            paragraph_alignment: crate::graphics::text::TextAlignment::Justified,
            screen_margin: 5,
            word_spacing_percent: 100,
            paragraph_indent: 0,
            css_enabled: true,
            touch_reader_controls: true,
            sleep_timeout_minutes: 10,
            front_button_back: 0,
            front_button_confirm: 1,
            front_button_left: 2,
            front_button_right: 3,
            side_button_page_forward: true,
            web_server_enabled: false,
            koreader_sync_enabled: false,
            koreader_sync_url: String::new(),
            ui_theme: 0,
            language: String::from("en"),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct StatusBarSpec {
    pub show_chapter_page_count: bool,
    pub show_title: bool,
    pub show_battery: bool,
    pub show_clock: bool,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ReaderRenderSpec {
    pub font_family: u16,
    pub font_point_size: u16,
    pub line_spacing: LineSpacing,
    pub margin_px: u16,
    pub word_spacing_percent: u16,
    pub paragraph_indent_px: i16,
    pub alignment: crate::graphics::text::TextAlignment,
    pub css_enabled: bool,
}

impl Settings {
    pub fn status_bar_spec(&self) -> StatusBarSpec {
        StatusBarSpec {
            show_chapter_page_count: self.status_bar_chapter_page_count,
            show_title: self.status_bar_title,
            show_battery: self.status_bar_battery,
            show_clock: self.status_bar_clock,
        }
    }

    pub fn reader_render_spec(&self) -> ReaderRenderSpec {
        ReaderRenderSpec {
            font_family: self.font_family,
            font_point_size: self.font_point_size,
            line_spacing: self.line_spacing,
            margin_px: self.screen_margin,
            word_spacing_percent: self.word_spacing_percent,
            paragraph_indent_px: self.paragraph_indent,
            alignment: self.paragraph_alignment,
            css_enabled: self.css_enabled,
        }
    }

    pub fn migrate_legacy(&mut self) {
        // Dummy migration implementation
        if self.font_point_size < 8 {
            self.font_point_size = 14;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_migrate_legacy() {
        let mut settings = Settings::default();
        settings.font_point_size = 5;
        settings.migrate_legacy();
        assert_eq!(settings.font_point_size, 14);
    }
}

/// Session state persisted separately from preferences.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RuntimeState {
    pub open_book_path: String,
    pub recent_sleep_images: Vec<u16>,
    pub recent_overlay_sleep_images: Vec<u16>,
    pub reader_activity_load_count: u8,
    pub last_sleep_from_reader: bool,
    pub show_boot_screen: bool,
}

impl Default for RuntimeState {
    fn default() -> Self {
        Self {
            open_book_path: String::new(),
            recent_sleep_images: Vec::new(),
            recent_overlay_sleep_images: Vec::new(),
            reader_activity_load_count: 0,
            last_sleep_from_reader: false,
            show_boot_screen: true,
        }
    }
}

impl RuntimeState {
    pub fn should_break_reader_crash_loop(&self) -> bool { self.reader_activity_load_count > 2 }
}

/// Generic persistence boundary used by settings/state/stores.
pub trait PersistableStore {
    type Value;
    type Error;
    fn load(&mut self) -> Result<Self::Value, Self::Error>;
    fn save(&mut self, value: &Self::Value) -> Result<(), Self::Error>;
}
