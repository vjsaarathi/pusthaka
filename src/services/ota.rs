use alloc::string::String;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum OtaError { NoUpdate, HttpError, JsonParseError, OlderVersion, Internal, OutOfMemory, WrongDevice }

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ReleaseInfo { pub version: String, pub asset_url: String, pub board_tag: String }

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum FirmwareImageSource { RemoteUrl(String), UserStoragePath(String) }

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FirmwareImageMetadata { pub chip_id: u16, pub board_tag: String, pub size_bytes: u32 }

/// OTA state machine. The implementation must validate chip ID and board identity before staging.
pub struct OtaService;

impl OtaService {
    pub fn check_for_update(&mut self) -> Result<Option<ReleaseInfo>, OtaError> { Ok(None) }

    pub fn install(&mut self, _image: FirmwareImageSource) -> Result<(), OtaError> { Ok(()) }
}

/// Chunk-boundary-safe scanner for a firmware board identification marker.
pub struct BoardTagScanner { pub pattern: &'static [u8], pub matched: usize }

impl BoardTagScanner {
    pub fn new(pattern: &'static [u8]) -> Self { Self { pattern, matched: 0 } }
    pub fn feed(&mut self, chunk: &[u8]) -> bool {
        for &byte in chunk {
            if self.pattern[self.matched] == byte {
                self.matched += 1;
                if self.matched == self.pattern.len() { return true; }
            } else {
                self.matched = if self.pattern[0] == byte { 1 } else { 0 };
            }
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ota() {
        let mut ota = OtaService;
        assert!(ota.check_for_update().unwrap().is_none());
        assert!(ota.install(FirmwareImageSource::RemoteUrl(String::new())).is_ok());

        let mut scanner = BoardTagScanner::new(b"TEST");
        assert!(!scanner.feed(b"TE"));
        assert!(scanner.feed(b"ST"));
    }
}
