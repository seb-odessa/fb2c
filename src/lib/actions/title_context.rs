use serde::Serialize;
use diesel::sql_types::Text;
use super::author_mask::AuthorMask;
use super::book_record::BookStringified;


#[derive(QueryableByName, Debug, Clone, Serialize)]
pub struct TitleMask {
    #[sql_type = "Text"] pub book_title: String,
}
impl TitleMask {
    pub fn new(title: String) -> Self {
        Self { book_title: title }
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct FindTitleContext {
    pub titles_nvc: Vec<String>,
}
impl FindTitleContext {
    pub fn new() -> Self {
        Self { titles_nvc: Vec::new() }
    }
}


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