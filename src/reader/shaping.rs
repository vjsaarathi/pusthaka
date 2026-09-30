use alloc::vec::Vec;

/// Bidirectional text levels for a paragraph/line.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum BidiClass { L, R, An, En, Neutral }

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct BidiRun { pub start: usize, pub end: usize, pub level: u8 }

/// Unicode Bidirectional Algorithm boundary used by the renderer.
pub trait BidiResolver {
    fn resolve(&self, codepoints: &[u32]) -> Vec<BidiRun>;
}

pub trait ArabicShaper {
    fn shape(&self, codepoints: &[u32]) -> Vec<u32>;
}

/// TODO: implement UBA levels, line reordering, Arabic shaping and mirrored punctuation.
pub struct UnicodeShaper;

impl ArabicShaper for UnicodeShaper {
    fn shape(&self, codepoints: &[u32]) -> Vec<u32> { codepoints.to_vec() }
}
