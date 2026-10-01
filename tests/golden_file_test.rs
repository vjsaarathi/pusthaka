use reader_os::reader::ingestion::{IngestionEngine, IngestionFormat};
use reader_os::reader::format::SemanticBlock;

#[test]
#[cfg(feature = "ingestion")]
fn test_epub_golden_file_fidelity() {
    // 1. Create a "Golden" EPUB payload in memory.
    // This perfectly mimics a valid EPUB HTML file with exact semantic tags.
    let golden_html = r#"
        <?xml version="1.0" encoding="utf-8"?>
        <html xmlns="http://www.w3.org/1999/xhtml">
            <body>
                <h1>The Genesis</h1>
                <p>In the beginning, there was code.</p>
                <p>And the code was zero-copy.</p>
            </body>
        </html>
    "#;

    let mut zip_buffer = std::vec::Vec::new();
    {
        use std::io::Write;
        let mut zip = zip::ZipWriter::new(std::io::Cursor::new(&mut zip_buffer));
        let options = zip::write::FileOptions::default();
        zip.start_file("chapter1.html", options).unwrap();
        zip.write_all(golden_html.as_bytes()).unwrap();
        zip.finish().unwrap();
    }

    // 2. Run the Ingestion Engine (EPUB -> ReaderBook Binary)
    let book = IngestionEngine::import_in_memory(IngestionFormat::Epub, &zip_buffer)
        .expect("Failed to ingest EPUB");

    // 3. STRICT ASSERTIONS: Prove the 1-to-1 semantic mapping worked flawlessly.
    
    // Assert TOC picked up the <h1> tag precisely
    assert_eq!(book.table_of_contents.len(), 1);
    assert_eq!(book.table_of_contents[0].title, "The Genesis");
    assert_eq!(book.table_of_contents[0].target_block_index, 0);

    // Assert the exact number of blocks were mapped
    assert_eq!(book.blocks.len(), 3);

    // Assert Block 0 is exactly a Heading
    match &book.blocks[0] {
        SemanticBlock::Heading { level, text } => {
            assert_eq!(*level, 1);
            assert_eq!(text, "The Genesis");
        },
        _ => panic!("Expected Heading, got something else"),
    }

    // Assert Block 1 is exactly a Paragraph with correct text
    match &book.blocks[1] {
        SemanticBlock::Paragraph { text, .. } => {
            assert_eq!(text, "In the beginning, there was code.");
        },
        _ => panic!("Expected Paragraph"),
    }

    // Assert Block 2 is exactly a Paragraph with correct text
    match &book.blocks[2] {
        SemanticBlock::Paragraph { text, .. } => {
            assert_eq!(text, "And the code was zero-copy.");
        },
        _ => panic!("Expected Paragraph"),
    }
}
