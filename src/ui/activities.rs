//! Concrete activity type declarations.
//! These are deliberately thin state containers for now. Rendering/navigation logic belongs in
//! each activity implementation, while hardware access belongs in services/HAL.

use super::activity::ActivityId;

macro_rules! declare_activity {
    ($name:ident, $id:expr) => {
        pub struct $name;
        impl super::activity::Activity for $name {
            fn id(&self) -> super::activity::ActivityId { $id }
            fn render(&mut self, ctx: &mut super::activity::ActivityContext<'_, '_>) -> crate::Result<()> {
                let font = crate::graphics::text::FontId(0);
                let style = crate::graphics::text::TextStyle {
                    weight: crate::graphics::text::FontWeight::Normal,
                    slant: crate::graphics::text::FontSlant::Normal,
                    decoration: crate::graphics::text::TextDecoration::None,
                    direction: crate::graphics::text::TextDirection::Ltr,
                    alignment: crate::graphics::text::TextAlignment::Left,
                    point_size: 24,
                };
                ctx.renderer.draw_text(ctx.font_provider, font, crate::graphics::geometry::Point { x: 50, y: 50 }, stringify!($name), style);
                Ok(())
            }
        }
    };
}

declare_activity!(SplashActivity, ActivityId::Splash);
pub struct HomeActivity;
impl super::activity::Activity for HomeActivity {
    fn id(&self) -> super::activity::ActivityId { super::activity::ActivityId::Home }
    fn render(&mut self, ctx: &mut super::activity::ActivityContext<'_, '_>) -> crate::Result<()> {
        let pt1 = crate::graphics::geometry::Point { x: 50, y: 50 };
        let pt2 = crate::graphics::geometry::Point { x: 200, y: 50 };
        let pt3 = crate::graphics::geometry::Point { x: 200, y: 150 };
        let pt4 = crate::graphics::geometry::Point { x: 50, y: 150 };
        ctx.renderer.draw_line(pt1, pt2);
        ctx.renderer.draw_line(pt2, pt3);
        ctx.renderer.draw_line(pt3, pt4);
        ctx.renderer.draw_line(pt4, pt1);
        ctx.renderer.fill_circle(crate::graphics::geometry::Point { x: 125, y: 100 }, 25);
        Ok(())
    }
}
declare_activity!(EpubReaderActivity, ActivityId::Reader);
declare_activity!(XtcReaderActivity, ActivityId::XtcReader);
declare_activity!(CrashReportActivity, ActivityId::CrashReport);
declare_activity!(LibraryListActivity, ActivityId::Library);
declare_activity!(FileBrowserActivity, ActivityId::FileBrowser);
declare_activity!(SettingsActivity, ActivityId::Settings);
declare_activity!(TextSettingsActivity, ActivityId::TextSettings);
declare_activity!(ButtonRemapActivity, ActivityId::ButtonRemap);
declare_activity!(ClockSettingsActivity, ActivityId::ClockSettings);
declare_activity!(ClockSyncActivity, ActivityId::ClockSync);
declare_activity!(TimezonePickerActivity, ActivityId::TimezonePicker);
declare_activity!(LanguageSelectActivity, ActivityId::LanguageSelect);
declare_activity!(StatusBarSettingsActivity, ActivityId::StatusBarSettings);
declare_activity!(HomeButtonSettingsActivity, ActivityId::HomeButtonSettings);
declare_activity!(KeyboardLayoutsActivity, ActivityId::KeyboardLayouts);
declare_activity!(OtaUpdateActivity, ActivityId::OtaUpdate);
declare_activity!(SdFirmwareUpdateActivity, ActivityId::SdFirmwareUpdate);
declare_activity!(FontDownloadActivity, ActivityId::FontDownload);
declare_activity!(ClearCacheActivity, ActivityId::ClearCache);
declare_activity!(AboutActivity, ActivityId::About);
declare_activity!(FrontlightPanelActivity, ActivityId::FrontlightPanel);
declare_activity!(KOReaderAuthActivity, ActivityId::KOReaderAuth);
declare_activity!(KOReaderSettingsActivity, ActivityId::KOReaderSettings);
declare_activity!(OpdsServerListActivity, ActivityId::OpdsServerList);
declare_activity!(OpdsSettingsActivity, ActivityId::OpdsSettings);
declare_activity!(WifiSelectionActivity, ActivityId::WifiSelection);
declare_activity!(CrossPointWebServerActivity, ActivityId::WebServer);
declare_activity!(UsbDriveActivity, ActivityId::UsbDrive);
declare_activity!(OpdsBookBrowserActivity, ActivityId::OpdsBookBrowser);
declare_activity!(OpdsDownloadActivity, ActivityId::OpdsDownload);
declare_activity!(KeyboardEntryActivity, ActivityId::KeyboardEntry);
declare_activity!(ConfirmationActivity, ActivityId::Confirmation);
declare_activity!(FullScreenMessageActivity, ActivityId::FullScreenMessage);
declare_activity!(IntervalSelectionActivity, ActivityId::IntervalSelection);
declare_activity!(BmpViewerActivity, ActivityId::BmpViewer);
declare_activity!(SleepActivity, ActivityId::Sleep);
