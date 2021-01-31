use serde::Serialize;
use super::book_record::BookRecord;

#[derive(Debug, Clone, Serialize)]
pub struct DownloadContext {
    pub book: BookRecord,
}
impl DownloadContext {
    pub fn new(book: BookRecord) -> Self {
        Self{ book: book }
    }

}