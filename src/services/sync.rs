use alloc::string::String;
use crate::hal::network::NetworkBackend;

pub struct SyncManager<'a, N: NetworkBackend> {
    pub network_hal: &'a mut N,
}

impl<'a, N: NetworkBackend> SyncManager<'a, N> {
    pub fn new(network_hal: &'a mut N) -> Self {
        Self { network_hal }
    }

    pub fn sync_position(&mut self, api_endpoint: &str, book_hash: &str, position: u32) -> Result<(), N::Error> {
        let payload = alloc::format!("{{\"hash\":\"{}\", \"pos\":{}}}", book_hash, position);
        let mut response = alloc::vec![0; 64];
        self.network_hal.http_post(api_endpoint, payload.as_bytes(), &mut response)?;
        Ok(())
    }
}
