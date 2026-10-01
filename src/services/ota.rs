use alloc::string::String;
use crate::hal::network::NetworkBackend;
use crate::hal::storage::{StorageBackend, StoragePath, WriteFile};

pub struct OtaManager<'a, N: NetworkBackend, S: StorageBackend> {
    pub network_hal: &'a mut N,
    pub storage_hal: &'a mut S,
}

impl<'a, N: NetworkBackend, S: StorageBackend> OtaManager<'a, N, S> {
    pub fn new(network_hal: &'a mut N, storage_hal: &'a mut S) -> Self {
        Self { network_hal, storage_hal }
    }

    pub fn check_for_update(&mut self, server_url: &str) -> Result<bool, N::Error> {
        let mut response = alloc::vec![0; 128];
        let bytes_read = self.network_hal.http_get(server_url, &mut response)?;
        Ok(bytes_read > 0 && response[0] == b'Y')
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum FirmwareImageSource { Network, SdCard }
