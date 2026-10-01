use reader_os::reader::ingestion::{IngestionEngine, IngestionFormat};
use reader_os::hal::storage::{StorageBackend, ReadFile, WriteFile, StoragePath};
use std::time::Instant;
use std::fs::File;
use std::io::{Write, Read};

// Mock Storage Backend for testing E2E
struct MockStorage;

struct MockFile(File);

impl ReadFile for MockFile {
    type Error = std::io::Error;
    fn read(&mut self, buffer: &mut [u8]) -> Result<usize, Self::Error> { self.0.read(buffer) }
    fn seek(&mut self, _offset: u64) -> Result<(), Self::Error> { Ok(()) }
    fn position(&mut self) -> Result<u64, Self::Error> { Ok(0) }
    fn size(&mut self) -> Result<u64, Self::Error> { Ok(0) }
    fn close(&mut self) -> Result<(), Self::Error> { Ok(()) }
}

impl WriteFile for MockFile {
    type Error = std::io::Error;
    fn write(&mut self, buffer: &[u8]) -> Result<usize, Self::Error> { self.0.write(buffer) }
    fn flush(&mut self) -> Result<(), Self::Error> { self.0.flush() }
    fn close(&mut self) -> Result<(), Self::Error> { Ok(()) }
}

impl StorageBackend for MockStorage {
    type Error = std::io::Error;
    type Reader<'a> = MockFile;
    type Writer<'a> = MockFile;

    fn begin(&mut self) -> Result<(), Self::Error> { Ok(()) }
    fn open_read<'a>(&'a mut self, path: StoragePath<'_>) -> Result<Self::Reader<'a>, Self::Error> {
        Ok(MockFile(File::open(path.0)?))
    }
    fn open_write<'a>(&'a mut self, path: StoragePath<'_>) -> Result<Self::Writer<'a>, Self::Error> {
        Ok(MockFile(File::create(path.0)?))
    }
    fn exists(&mut self, _path: StoragePath<'_>) -> Result<bool, Self::Error> { Ok(true) }
    fn remove(&mut self, _path: StoragePath<'_>) -> Result<(), Self::Error> { Ok(()) }
    fn mkdir(&mut self, _path: StoragePath<'_>) -> Result<(), Self::Error> { Ok(()) }
    fn rename(&mut self, _from: StoragePath<'_>, _to: StoragePath<'_>) -> Result<(), Self::Error> { Ok(()) }
}

#[test]
fn test_txt_ingestion_and_storage() {
    let mut storage = MockStorage;
    let raw_text = b"# Chapter 1\nThis is a test document.\n# Chapter 2\nMore text.";
    
    let start = Instant::now();
    let result = IngestionEngine::import_and_save(&mut storage, IngestionFormat::Txt, raw_text, "test_book.rkyv");
    let duration = start.elapsed();
    
    assert!(result.is_ok());
    println!("TXT Ingestion + Storage took: {:?}", duration);
    
    let metadata = std::fs::metadata("test_book.rkyv").unwrap();
    assert!(metadata.len() > 0);
}

#[test]
#[cfg(feature = "ingestion")]
fn benchmark_epub_ingestion() {
    // Generate a dummy valid zip file to simulate an EPUB
    use std::io::Write;
    let mut zip_buffer = std::vec::Vec::new();
    {
        let mut zip = zip::ZipWriter::new(std::io::Cursor::new(&mut zip_buffer));
        let options = zip::write::FileOptions::default();
        zip.start_file("test.html", options).unwrap();
        zip.write_all(b"<p>Test EPUB content</p>").unwrap();
        zip.finish().unwrap();
    }
    
    let start = Instant::now();
    let book = IngestionEngine::import_in_memory(IngestionFormat::Epub, &zip_buffer).unwrap();
    let duration = start.elapsed();
    
    assert_eq!(book.title, "EPUB Doc");
    println!("EPUB Ingestion Benchmark took: {:?}", duration);
}

#[test]
#[cfg(feature = "ingestion")]
fn benchmark_pdf_ingestion() {
    let dummy_pdf = b"%PDF-1.4\n1 0 obj\n<< /Type /Catalog /Pages 2 0 R >>\nendobj\n";
    
    let start = Instant::now();
    let result = IngestionEngine::import_in_memory(IngestionFormat::Pdf, dummy_pdf);
    let duration = start.elapsed();
    
    // It should fail gracefully since it's a dummy corrupted PDF
    assert!(result.is_err());
    println!("PDF (Failed) Ingestion Benchmark took: {:?}", duration);
}
