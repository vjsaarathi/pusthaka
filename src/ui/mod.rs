//! Hardware-independent UI framework and activity model.

pub mod activity;
pub mod activities;
pub mod navigation;
pub mod keyboard;
pub mod list;
pub mod reader;
pub mod sleep;
pub mod theme;
pub mod widget;

pub use activity::{Activity, ActivityContext, ActivityId, ActivityResult, ActivityResultHandler};
pub use navigation::{ActivityManager, PendingTransition};
pub use theme::{Theme, ThemeId, ThemeMetrics};
