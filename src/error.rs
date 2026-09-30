use core::fmt;

/// Top-level error categories shared by the hardware-independent layers.
///
/// Device drivers should retain their precise native error types and translate only
/// at the service boundary when a generic error is required.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CoreError {
    /// A required capability is not present on the current device.
    Unsupported,
    /// A requested object or resource does not exist.
    NotFound,
    /// A caller supplied invalid data.
    InvalidInput,
    /// Persistent data is malformed or incompatible.
    CorruptData,
    /// A bounded operation ran out of working memory.
    OutOfMemory,
    /// An operation cannot continue yet and must be resumed later.
    Busy,
    /// Generic I/O failure after a lower-level error was intentionally erased.
    Io,
    /// Generic network failure after a lower-level error was intentionally erased.
    Network,
    /// A background/foreground state machine reached an unrecoverable state.
    State,
}

impl fmt::Display for CoreError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let text = match self {
            Self::Unsupported => "unsupported",
            Self::NotFound => "not found",
            Self::InvalidInput => "invalid input",
            Self::CorruptData => "corrupt data",
            Self::OutOfMemory => "out of memory",
            Self::Busy => "busy",
            Self::Io => "I/O error",
            Self::Network => "network error",
            Self::State => "invalid state",
        };
        f.write_str(text)
    }
}

/// Convenient result type for hardware-independent ReaderOS APIs.
pub type Result<T> = core::result::Result<T, CoreError>;
