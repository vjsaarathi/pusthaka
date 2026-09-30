/// Immutable path view used by the storage service.
pub struct StoragePath<'a>(pub &'a str);

/// Read-only file handle contract.
pub trait ReadFile {
    type Error;
    fn read(&mut self, buffer: &mut [u8]) -> Result<usize, Self::Error>;
    fn seek(&mut self, offset: u64) -> Result<(), Self::Error>;
    fn position(&mut self) -> Result<u64, Self::Error>;
    fn size(&mut self) -> Result<u64, Self::Error>;
    fn close(&mut self) -> Result<(), Self::Error>;
}

/// Writable file handle contract.
pub trait WriteFile {
    type Error;
    fn write(&mut self, buffer: &[u8]) -> Result<usize, Self::Error>;
    fn flush(&mut self) -> Result<(), Self::Error>;
    fn close(&mut self) -> Result<(), Self::Error>;
}

/// Portable storage service contract.
///
/// Implementations must serialize access to a non-thread-safe SD/FAT stack. Recursive
/// locking is an implementation option, but the observable contract is that a nested
/// storage operation cannot corrupt the backing bus/filesystem.
pub trait StorageBackend {
    type Error;
    type Reader<'a>: ReadFile<Error = Self::Error> where Self: 'a;
    type Writer<'a>: WriteFile<Error = Self::Error> where Self: 'a;

    fn begin(&mut self) -> Result<(), Self::Error>;
    fn open_read<'a>(&'a mut self, path: StoragePath<'_>) -> Result<Self::Reader<'a>, Self::Error>;
    fn open_write<'a>(&'a mut self, path: StoragePath<'_>) -> Result<Self::Writer<'a>, Self::Error>;
    fn exists(&mut self, path: StoragePath<'_>) -> Result<bool, Self::Error>;
    fn remove(&mut self, path: StoragePath<'_>) -> Result<(), Self::Error>;
    fn mkdir(&mut self, path: StoragePath<'_>) -> Result<(), Self::Error>;
    fn rename(&mut self, from: StoragePath<'_>, to: StoragePath<'_>) -> Result<(), Self::Error>;
}

/// Read/write mode for scratch files managed by ReaderOS.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum StorageIntent {
    UserData,
    SystemData,
    Cache,
    Scratch,
}
