use reader_os::reader::format::ReaderBook;

fn test_rkyv(book: &ReaderBook) {
    let bytes = rkyv::to_bytes::<rkyv::rancor::Error>(book).unwrap();
    let archived = unsafe { rkyv::access_unchecked::<<ReaderBook as rkyv::Archive>::Archived>(&bytes) };
    let book: ReaderBook = rkyv::deserialize::<ReaderBook, rkyv::rancor::Error>(archived).unwrap();
}
fn main() {}
