use alloc::{string::String, vec::Vec};

use super::blocks::Block;
use super::css::CssStyle;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum HtmlTag {
    P, Div, H1, H2, H3, H4, H5, H6, Blockquote, Pre, Hr, Br, Script, Style, Head, Nav, Header, Footer,
    Ul, Ol, Li, B, Strong, I, Em, U, S, Strike, Del, Sup, Sub, Span,
    A, Small, Code, Img, Svg, Image, Table, Thead, Tbody, Tfoot, Tr, Td, Th,
    Ruby, Rt, Rp, Aside, Unknown,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct StyleStackEntry {
    pub bold: bool,
    pub italic: bool,
    pub underline: bool,
    pub strike: bool,
    pub direction_rtl: bool,
    pub superscript: bool,
    pub subscript: bool,
    pub depth: u16,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ListKind { Ordered, Unordered }

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ListContext { pub kind: ListKind, pub counter: u16, pub depth: u8, pub start_value: u16 }

/// Cooperative HTML→block-model parser. No DOM is required or permitted.
pub struct ChapterHtmlParser {
    pub style_stack: Vec<StyleStackEntry>,
    pub list_stack: Vec<ListContext>,
    pub effective_style: CssStyle,
    pub current_word: String,
    pub blocks: Vec<Block>,
    pub soft_flush_words: u16,
}

impl ChapterHtmlParser {
    /// Parse a bounded slice and return whether the chapter has reached end-of-input.
    pub fn parse_step(&mut self, _input: &[u8], _max_words: u16) -> crate::Result<bool> {
        Ok(true)
    }

    /// TODO: apply CSS rule → inline style stack → HTML attribute precedence.
    pub fn update_effective_style(&mut self) {}
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PageLink { pub href: String, pub anchor: String }
