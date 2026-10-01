use alloc::string::String;
use crate::hal::network::NetworkBackend;

pub struct NetworkManager<'a, N: NetworkBackend> {
    pub network_hal: &'a mut N,
}

impl<'a, N: NetworkBackend> NetworkManager<'a, N> {
    pub fn new(network_hal: &'a mut N) -> Self {
        Self { network_hal }
    }

    pub fn ensure_connected(&mut self, ssid: &str, psk: &str) -> Result<(), N::Error> {
        let state = self.network_hal.state()?;
        if state != crate::hal::network::ConnectionState::Connected {
            self.network_hal.connect(ssid, Some(psk))?;
        }
        Ok(())
    }

    pub fn disconnect(&mut self) -> Result<(), N::Error> {
        self.network_hal.disconnect()
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum NetworkEvent { Connected, Disconnected, Error(String) }
