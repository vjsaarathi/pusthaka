use alloc::{boxed::Box, string::String};

use crate::graphics::renderer::Renderer;
use crate::hal::input::InputEvent;

/// Complete activity vocabulary for the planned UI surface.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ActivityId {
    Splash,
    Home,
    Reader,
    XtcReader,
    CrashReport,
    Library,
    FileBrowser,
    Settings,
    TextSettings,
    ButtonRemap,
    ClockSettings,
    ClockSync,
    TimezonePicker,
    LanguageSelect,
    StatusBarSettings,
    HomeButtonSettings,
    KeyboardLayouts,
    OtaUpdate,
    SdFirmwareUpdate,
    FontDownload,
    ClearCache,
    About,
    FrontlightPanel,
    KOReaderAuth,
    KOReaderSettings,
    OpdsServerList,
    OpdsSettings,
    WifiSelection,
    WebServer,
    UsbDrive,
    OpdsBookBrowser,
    OpdsDownload,
    KeyboardEntry,
    Confirmation,
    FullScreenMessage,
    IntervalSelection,
    BmpViewer,
    Sleep,
    EndOfBookOptions,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ActivityResult { pub code: u16, pub payload: Option<String> }

pub type ActivityResultHandler = Box<dyn FnMut(ActivityResult)>;

/// Dependency bundle intentionally passed into activities instead of relying on globals.
pub struct ActivityContext<'r, 'fb> {
    pub renderer: &'r mut Renderer<'fb>,
    pub font_provider: &'r mut dyn crate::graphics::text::FontProvider<Error = crate::CoreError>,
}

/// Activity lifecycle contract.
pub trait Activity {
    fn id(&self) -> ActivityId;
    fn on_enter(&mut self, _ctx: &mut ActivityContext<'_, '_>) -> crate::Result<()> { Ok(()) }
    fn poll(&mut self, _events: &[InputEvent], _ctx: &mut ActivityContext<'_, '_>) -> crate::Result<()> { Ok(()) }
    fn render(&mut self, _ctx: &mut ActivityContext<'_, '_>) -> crate::Result<()> { Ok(()) }
    fn on_exit(&mut self, _ctx: &mut ActivityContext<'_, '_>) -> crate::Result<()> { Ok(()) }
    fn prevent_auto_sleep(&self) -> bool { false }
    fn is_reader(&self) -> bool { false }
}
