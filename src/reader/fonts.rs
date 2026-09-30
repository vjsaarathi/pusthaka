use alloc::collections::BTreeMap;
use alloc::string::String;
use alloc::vec::Vec;

use crate::graphics::text::{FontFamilyId, FontId, FontProvider, GlyphId, GlyphMetrics, FixedPoint4};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum FontSource { BuiltIn, Cpfont, TrueType }

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FontDescriptor {
    pub id: FontId,
    pub family: FontFamilyId,
    pub source: FontSource,
    pub point_size: u16,
}

/// Custom SD font metadata. The on-disk format is intentionally kept separate from the renderer.
pub struct CpfontFamily {
    pub family_name: String,
    pub style_count: u8,
    pub glyph_count: u32,
}

/// LRU glyph cache with explicit memory-pressure eviction.
pub struct FontCache {
    pub max_entries: usize,
    pub entries: BTreeMap<u32, GlyphMetrics>,
    pub access_order: Vec<u32>,
}

impl FontCache {
    pub fn new(max_entries: usize) -> Self {
        Self { max_entries, entries: BTreeMap::new(), access_order: Vec::new() }
    }

    /// TODO: enforce LRU eviction and never allocate past the configured budget.
    pub fn evict_if_needed(&mut self) {}
}

pub struct FontSystem<'a> {
    pub fonts: BTreeMap<u16, rusttype::Font<'a>>,
}

impl<'a> FontSystem<'a> {
    pub fn new() -> Self {
        Self { fonts: BTreeMap::new() }
    }
    
    pub fn add_font(&mut self, id: FontId, data: &'a [u8]) -> Result<(), crate::CoreError> {
        let font = rusttype::Font::try_from_bytes(data).ok_or(crate::CoreError::CorruptData)?;
        self.fonts.insert(id.0, font);
        Ok(())
    }
}

impl<'a> FontProvider for FontSystem<'a> {
    type Error = crate::CoreError;

    fn glyph_for_codepoint(&mut self, font: FontId, codepoint: u32) -> Result<(GlyphId, GlyphMetrics), Self::Error> {
        let f = self.fonts.get(&font.0).ok_or(crate::CoreError::NotFound)?;
        let c = core::char::from_u32(codepoint).unwrap_or('?');
        let g = f.glyph(c);
        let id = g.id().0 as u32;
        
        // We use a default scale of 16.0 for base metrics
        let sg = g.scaled(rusttype::Scale::uniform(16.0));
        let h_metrics = sg.h_metrics();
        
        Ok((GlyphId(id), GlyphMetrics {
            width: 0,
            height: 0,
            bearing_x: 0,
            bearing_y: 0,
            advance_x: FixedPoint4::from_raw((h_metrics.advance_width * 16.0) as i32),
        }))
    }

    fn kerning(&mut self, _font: FontId, _left: GlyphId, _right: GlyphId) -> Result<FixedPoint4, Self::Error> {
        Ok(FixedPoint4::from_raw(0))
    }

    fn rasterize_glyph(&mut self, font: FontId, glyph: GlyphId, scale: f32) -> Result<alloc::vec::Vec<u8>, Self::Error> {
        let f = self.fonts.get(&font.0).ok_or(crate::CoreError::NotFound)?;
        let g = f.glyph(rusttype::GlyphId(glyph.0 as u16));
        let sg = g.scaled(rusttype::Scale::uniform(scale));
        let pg = sg.positioned(rusttype::Point { x: 0.0, y: 0.0 });
        
        let bb = pg.pixel_bounding_box().unwrap_or(rusttype::Rect { min: rusttype::Point { x: 0, y: 0 }, max: rusttype::Point { x: 0, y: 0 } });
        let width = bb.width() as usize;
        let height = bb.height() as usize;
        
        // Prepend 4 bytes for width, 4 bytes for height, 4 for bearing_x, 4 for bearing_y
        // Then the bitmap data
        let mut buf = Vec::new();
        buf.extend_from_slice(&(width as u32).to_le_bytes());
        buf.extend_from_slice(&(height as u32).to_le_bytes());
        buf.extend_from_slice(&(bb.min.x as i32).to_le_bytes());
        buf.extend_from_slice(&(bb.min.y as i32).to_le_bytes());
        buf.extend_from_slice(&(pg.unpositioned().h_metrics().advance_width as u32).to_le_bytes());
        
        buf.resize(20 + width * height, 0);
        
        pg.draw(|x, y, v| {
            let idx = 20 + y as usize * width + x as usize;
            if idx < buf.len() {
                buf[idx] = (v * 255.0) as u8;
            }
        });
        
        Ok(buf)
    }
}
