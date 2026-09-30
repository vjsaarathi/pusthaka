use alloc::string::String;

/// Canonical URL value used by OPDS, OTA, sync and Wi-Fi provisioning services.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Url(pub String);

impl Url {
    /// TODO: parse/normalize scheme, host, port, path and query without heap-heavy URI libraries on firmware.
    pub fn parse(value: &str) -> crate::Result<Self> { Ok(Url(String::from(value))) }
}

/// QR payload boundary. The encoder is deliberately isolated from Wi-Fi/network policy.
pub struct QrPayload(pub String);

pub struct QrEncoder;
impl QrEncoder {
    /// TODO: produce a bounded QR matrix suitable for monochrome rendering.
    pub fn encode(&self, _payload: &QrPayload) -> crate::Result<alloc::vec::Vec<u8>> { Ok(alloc::vec::Vec::new()) }
}

/// Natural-sort key helper for filenames and library entries.
pub struct NaturalSort;
impl NaturalSort {
    /// TODO: compare digit runs numerically and non-digits case-insensitively.
    pub fn less(left: &str, right: &str) -> bool { left < right }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_utilities() {
        assert_eq!(Url::parse("http://x").unwrap().0, "http://x");
        assert!(QrEncoder.encode(&QrPayload(String::from("x"))).unwrap().is_empty());
        assert!(NaturalSort::less("a", "b"));
    }
}
