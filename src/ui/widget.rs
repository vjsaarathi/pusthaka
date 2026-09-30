use crate::graphics::renderer::Renderer;
use crate::graphics::geometry::Rect;

pub trait Widget {
    fn bounds(&self) -> Rect;
    fn render(&self, renderer: &mut Renderer<'_>) -> crate::Result<()>;
}

pub struct Label { pub bounds: Rect, pub text: alloc::string::String }
pub struct Button { pub bounds: Rect, pub text: alloc::string::String, pub selected: bool }
pub struct List { pub bounds: Rect, pub selected: usize, pub item_count: usize }
pub struct ProgressBar { pub bounds: Rect, pub current: u64, pub total: u64 }
pub struct Popup { pub bounds: Rect, pub title: alloc::string::String }
pub struct Slider { pub bounds: Rect, pub value: u8 }
