use alloc::{string::String, vec::Vec};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CssUnit { Px, Em, Percent, Pt, None }

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CssLength { pub value: f32, pub unit: CssUnit }

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CssDisplay { Block, Inline, None }

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CssDirection { Ltr, Rtl }

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CssFontWeight { Normal, Bold }

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CssFontStyle { Normal, Italic }

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CssTextDecoration { None, Underline, LineThrough }

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CssTextAlign { Left, Center, Right, Justify }

/// Compact style model mirroring the intentionally limited CrossPoint-style CSS subset.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CssStyle {
    pub display: CssDisplay,
    pub margin_top: CssLength,
    pub margin_bottom: CssLength,
    pub margin_left: CssLength,
    pub margin_right: CssLength,
    pub padding_top: CssLength,
    pub padding_bottom: CssLength,
    pub padding_left: CssLength,
    pub padding_right: CssLength,
    pub text_indent: CssLength,
    pub text_align: CssTextAlign,
    pub direction: CssDirection,
    pub font_weight: CssFontWeight,
    pub font_style: CssFontStyle,
    pub text_decoration: CssTextDecoration,
    pub font_size: CssLength,
    pub line_height: CssLength,
    pub width: CssLength,
    pub max_width: CssLength,
    pub border_bottom: bool,
}

impl Default for CssStyle {
    fn default() -> Self {
        let zero = CssLength { value: 0.0, unit: CssUnit::None };
        Self {
            display: CssDisplay::Block,
            margin_top: zero, margin_bottom: zero, margin_left: zero, margin_right: zero,
            padding_top: zero, padding_bottom: zero, padding_left: zero, padding_right: zero,
            text_indent: zero,
            text_align: CssTextAlign::Left,
            direction: CssDirection::Ltr,
            font_weight: CssFontWeight::Normal,
            font_style: CssFontStyle::Normal,
            text_decoration: CssTextDecoration::None,
            font_size: zero,
            line_height: zero,
            width: zero,
            max_width: zero,
            border_bottom: false,
        }
    }
}

/// Selector subset intentionally exposed by the reader engine.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum CssSelector {
    Element(String),
    Class(String),
    Id(String),
    Compound { element: Option<String>, class: Option<String>, id: Option<String> },
    Descendant(Vec<CssSelector>),
}

#[derive(Clone, Debug, PartialEq)]
pub struct CssRule { pub selectors: Vec<CssSelector>, pub style: CssStyle }

pub const CSS_CACHE_VERSION: u32 = 12;

/// Stateful stylesheet parser.
pub struct CssParser { pub rules: Vec<CssRule> }

impl CssParser {
    /// TODO: parse only the supported selector/property subset and ignore unsupported CSS safely.
    pub fn parse_stream(&mut self, _bytes: &[u8]) -> crate::Result<()> { Ok(()) }

    /// TODO: serialize the parsed rules to the versioned cache format.
    pub fn write_cache(&self) -> crate::Result<()> { Ok(()) }
}
