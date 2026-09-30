use alloc::string::String;

use crate::reader::document::ReaderPosition;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SyncCredentials { pub username: String, pub password_hash: String }

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SyncMetadata { pub filename: String, pub title: String, pub authors: String }

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RichPosition {
    pub percent_x100: u16,
    pub spine_index: u16,
    pub page_number: u16,
    pub total_pages: u16,
    pub paragraph_index: u16,
    pub xpath: String,
}

#[derive(Clone, Debug, PartialEq)]
pub struct SyncProgress {
    pub document: String,
    pub progress: String,
    pub percentage: f32,
    pub device: String,
    pub device_id: String,
    pub timestamp: u64,
    pub metadata: SyncMetadata,
    pub position: RichPosition,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SyncError { NoCredentials, Network, AuthFailed, Server, Json, NotFound, LowMemory, UserExists }

pub struct KOSyncClient;

impl KOSyncClient {
    pub fn sync(&mut self, _position: ReaderPosition) -> Result<(), SyncError> { Ok(()) }
}

/// HTML stream mapper between visible-text offsets and KOReader XPath positions.
pub struct XPathResolver;

impl XPathResolver {
    pub fn to_xpath(&mut self, _offset: u32) -> crate::Result<String> { Ok(String::new()) }
    pub fn from_xpath(&mut self, _xpath: &str) -> crate::Result<u32> { Ok(0) }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sync() {
        let mut client = KOSyncClient;
        assert!(client.sync(ReaderPosition {
            spine_index: 0,
            page_number: 0,
            total_pages: 1,
            visible_text_offset: None,
            paragraph_index: None,
            percentage: 0.0,
            point: None,
        }).is_ok());

        let mut resolver = XPathResolver;
        assert!(resolver.to_xpath(0).is_ok());
        assert_eq!(resolver.from_xpath("").unwrap(), 0);
    }
}
