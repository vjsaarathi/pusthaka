use alloc::vec::Vec;

use crate::graphics::geometry::Rect;
use crate::graphics::text::{FontId, FixedPoint4, TextAlignment, TextDirection};
use crate::reader::epub::blocks::Block;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct LayoutContext {
    pub viewport: Rect,
    pub font: FontId,
    pub point_size: u16,
    pub line_spacing_percent: u16,
    pub margin_px: u16,
    pub word_spacing_percent: u16,
    pub alignment: TextAlignment,
    pub direction: TextDirection,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PositionedWord {
    pub x: FixedPoint4,
    pub y: FixedPoint4,
    pub width: FixedPoint4,
    pub glyph_count: u16,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LayoutLine { pub words: Vec<PositionedWord>, pub baseline: i32, pub height: u16 }

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PageLayout { pub lines: Vec<LayoutLine>, pub height: u16 }

/// Stateless layout engine contract. Persistent parser state belongs to the pagination engine.
pub struct LayoutEngine;

impl LayoutEngine {
    pub fn layout(&self, _blocks: &[Block], _ctx: LayoutContext) -> crate::Result<PageLayout> {
        Ok(PageLayout { lines: Vec::new(), height: 0 })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_layout() {
        let engine = LayoutEngine;
        let ctx = LayoutContext {
            viewport: Rect { x: 0, y: 0, width: 100, height: 100 },
            font: FontId(0),
            point_size: 14,
            line_spacing_percent: 100,
            margin_px: 5,
            word_spacing_percent: 100,
            alignment: TextAlignment::Left,
            direction: TextDirection::Ltr,
        };
        let layout = engine.layout(&[], ctx).unwrap();
        assert!(layout.lines.is_empty());
    }
}
