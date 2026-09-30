use crate::services::networking::{HttpRequest, HttpResponseHead};

/// Consumer for a streaming HTTP response body.
pub trait HttpResponseSink {
    fn write(&mut self, bytes: &[u8]) -> crate::Result<usize>;
}

/// Optional network transport boundary. The reader core only uses higher-level services.
pub trait NetworkInterface {
    type Error;
    fn is_available(&self) -> bool;
    fn poll(&mut self) -> Result<(), Self::Error>;

    /// Bodies are streamed into a sink; the transport must never require the whole response
    /// to coexist in RAM. Redirects/auth/TLS policy belongs above or alongside this boundary.
    fn execute_http<S: HttpResponseSink>(&mut self, request: &HttpRequest<'_>, sink: &mut S) -> Result<HttpResponseHead, Self::Error>;
}
