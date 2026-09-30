//! Deterministic graphics primitives and display-independent rendering.

pub mod bitmap;
pub mod color;
pub mod dithering;
pub mod framebuffer;
pub mod geometry;
pub mod renderer;
pub mod text;

pub use framebuffer::{FrameBuffer, FrameBufferView, PixelFormat};
pub use geometry::{Point, Rect, Rotation, Size};
pub use renderer::{RenderMode, Renderer};
pub use text::{FontFamilyId, FontId, TextAlignment, TextDirection, TextStyle};
