#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ReaderToolbarItem { Chapter, Progress, TextSettings, More }

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ReaderTouchZones {
    pub toolbar_top: u16,
    pub toolbar_bottom: u16,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ReaderAction { PreviousPage, NextPage, ShowToolbar, HideOverlay, OpenContents, OpenTextSettings, OpenMore }
