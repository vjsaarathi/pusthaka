use alloc::{string::String, vec::Vec};

/// Types of network security
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SecurityType { Open, Wep, Wpa, Wpa2, Wpa3 }

/// Network connection status
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ConnectionState { Disconnected, Connecting, Connected, AuthFailed, NetworkNotFound }

/// Trait representing an abstract Network interface (e.g., WiFi chip, Ethernet)
pub trait NetworkBackend {
    type Error: core::fmt::Debug;

    /// Initialize the network hardware
    fn initialize(&mut self) -> Result<(), Self::Error>;
    
    /// Scan for available networks (Access Points)
    fn scan(&mut self) -> Result<Vec<(String, SecurityType, i8)>, Self::Error>;
    
    /// Connect to a specific access point
    fn connect(&mut self, ssid: &str, password: Option<&str>) -> Result<(), Self::Error>;
    
    /// Disconnect from the current network
    fn disconnect(&mut self) -> Result<(), Self::Error>;
    
    /// Get the current connection state
    fn state(&self) -> Result<ConnectionState, Self::Error>;
    
    /// Execute an HTTP GET request (useful for OTA and OPDS)
    fn http_get(&mut self, url: &str, response_buffer: &mut [u8]) -> Result<usize, Self::Error>;
    
    /// Execute an HTTP POST request
    fn http_post(&mut self, url: &str, payload: &[u8], response_buffer: &mut [u8]) -> Result<usize, Self::Error>;
}
