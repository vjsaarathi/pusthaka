

/// Supported precompiled Liang language identifiers.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum HyphenationLanguage { En, De, Es, Fi, Fr, It, Pl, Pt, Ru, Sv, Uk }

/// Core Liang hyphenation contract.
pub trait Hyphenator {
    fn hyphenate(&self, word: &str, language: Option<HyphenationLanguage>) -> HyphenationResult;
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct HyphenationResult { pub break_positions: alloc::vec::Vec<u16>, pub used_fallback: bool }

/// Serialized trie format: 32-bit packed nodes + parallel value storage.
pub struct SerializedHyphenationTrie<'a> {
    pub nodes: &'a [u32],
    pub values: &'a [u8],
}

/// TODO: implement sentinel wrapping, overlapping-pattern maxima, odd-value breaks,
/// explicit/soft hyphen handling, apostrophe protection and fallback splitting.
pub struct LiangHyphenator<'a> { pub trie: Option<SerializedHyphenationTrie<'a>> }

impl<'a> Hyphenator for LiangHyphenator<'a> {
    fn hyphenate(&self, _word: &str, _language: Option<HyphenationLanguage>) -> HyphenationResult {
        HyphenationResult { break_positions: alloc::vec::Vec::new(), used_fallback: false }
    }
}
