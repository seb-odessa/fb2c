use serde::Serialize;
use super::author_mask::AuthorMask;
use super::book_record::BookStringified;

#[derive(Debug, Clone, Serialize)]
pub struct TitleContext {
    pub first_name: String,
    pub middle_name: String,
    pub last_name: String,
    pub title: String,
    pub books: Vec<BookStringified>,
}
impl TitleContext {
    pub fn new(author: &AuthorMask, title: String) -> Self {
        Self{
            first_name: author.get_encoded_by_name("first_name"),
            middle_name: author.get_encoded_by_name("middle_name"),
            last_name: author.get_encoded_by_name("last_name"),
            title: title,
            books: Vec::new(),
        }
    }

}