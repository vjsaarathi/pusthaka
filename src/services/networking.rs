use alloc::{string::String, vec::Vec};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum HttpMethod { Get, Head, Post, Put, Delete, Options, Propfind, Mkcol, Move, Copy, Lock, Unlock }

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct HttpRequest<'a> {
    pub method: HttpMethod,
    pub path: &'a str,
    pub headers: Vec<(&'a str, &'a str)>,
    pub body: &'a [u8],
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct HttpResponseHead {
    pub status: u16,
    pub content_type: String,
    pub content_length: Option<u64>,
}

/// Small server-side response convenience type. Large transfers must use a streaming sink.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct HttpResponse { pub head: HttpResponseHead, pub body: Vec<u8> }

/// Network events are copied into the kernel event queue rather than retaining driver buffers.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum NetworkEvent { Connected, Disconnected, UploadProgress { received: u64, total: u64 }, Error }

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FileInfo { pub name: String, pub size: u64, pub is_directory: bool, pub is_epub: bool }

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UploadState {
    pub file_name: String,
    pub path: String,
    pub size: u64,
    pub success: bool,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WebDavOverwrite { Allow, Reject }

/// Request/body streaming sink used by server upload handling and OTA/OPDS clients.
pub trait ByteStreamSink {
    type Error;
    fn write(&mut self, bytes: &[u8]) -> Result<usize, Self::Error>;
    fn finish(&mut self) -> Result<(), Self::Error>;
}

/// Application-level HTTP server service. Transport specifics stay in the HAL adapter.
pub struct WebServerService { pub enabled: bool }

impl WebServerService {
    /// TODO: expose the documented status/file/settings/font/Wi-Fi/OPDS endpoints and WebSocket upload flow.
    pub fn poll(&mut self, _budget: crate::kernel::WorkBudget) -> crate::Result<crate::kernel::PollResult> {
        Ok(crate::kernel::PollResult::Complete)
    }
}

/// WebDAV Class 1 request dispatcher.
pub struct WebDavService;

impl WebDavService {
    pub fn handle(&mut self, _request: &HttpRequest<'_>) -> crate::Result<HttpResponse> {
        Ok(HttpResponse {
            head: HttpResponseHead { status: 200, content_type: String::new(), content_length: None },
            body: Vec::new(),
        })
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WifiNetwork { pub ssid: String, pub rssi: i16, pub secured: bool }

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WifiCredential { pub ssid: String, pub password: String }

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct OpdsServer { pub name: String, pub base_url: String, pub username: Option<String>, pub password: Option<String> }

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct OpdsEntry { pub title: String, pub author: String, pub href: String, pub mime: String, pub is_container: bool }

pub struct OpdsClient;
impl OpdsClient {
    pub fn fetch_page(&mut self, _url: &str) -> crate::Result<Vec<OpdsEntry>> { Ok(Vec::new()) }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum NetworkAuth { None, Basic }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_networking() {
        let mut web = WebServerService { enabled: true };
        assert_eq!(web.poll(crate::kernel::WorkBudget::steps(1)).unwrap(), crate::kernel::PollResult::Complete);

        let mut dav = WebDavService;
        let req = HttpRequest { method: HttpMethod::Get, path: "/", headers: Vec::new(), body: &[] };
        assert_eq!(dav.handle(&req).unwrap().head.status, 200);

        let mut opds = OpdsClient;
        assert!(opds.fetch_page("http://x").unwrap().is_empty());
    }
}
