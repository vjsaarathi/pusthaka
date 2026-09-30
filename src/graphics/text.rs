#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FontFamilyId(pub u16);

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FontId(pub u16);

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum FontWeight { Normal, Bold }

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum FontSlant { Normal, Italic }

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TextDecoration { None, Underline, LineThrough }

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TextDirection { Ltr, Rtl }

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TextAlignment { Left, Center, Right, Justified }

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct TextStyle {
    pub weight: FontWeight,
    pub slant: FontSlant,
    pub decoration: TextDecoration,
    pub direction: TextDirection,
    pub alignment: TextAlignment,
    pub point_size: u16,
}

/// 12.4 fixed-point value used for deterministic text advances.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FixedPoint4(pub i32);

impl FixedPoint4 {
    pub const FRACTION_BITS: i32 = 4;
    pub const ONE: Self = Self(16);

    pub const fn from_raw(raw: i32) -> Self { Self(raw) }
    pub const fn to_pixel(self) -> i32 { (self.0 + 8) >> 4 }
    pub const fn add(self, other: Self) -> Self { Self(self.0 + other.0) }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct GlyphId(pub u32);

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct GlyphMetrics {
    pub width: u16,
    pub height: u16,
    pub bearing_x: i16,
    pub bearing_y: i16,
    pub advance_x: FixedPoint4,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct KerningPair {
    pub left: GlyphId,
    pub right: GlyphId,
    pub adjustment: FixedPoint4,
}

/// Font system boundary. Font storage may be internal flash, SD, or vector fonts.
pub trait FontProvider {
    type Error;
    fn glyph_for_codepoint(&mut self, font: FontId, codepoint: u32) -> Result<(GlyphId, GlyphMetrics), Self::Error>;
    fn kerning(&mut self, font: FontId, left: GlyphId, right: GlyphId) -> Result<FixedPoint4, Self::Error>;
    fn rasterize_glyph(&mut self, font: FontId, glyph: GlyphId, scale: f32) -> Result<alloc::vec::Vec<u8>, Self::Error>;
}
