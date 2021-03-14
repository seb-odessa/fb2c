use serde::Serialize;
use diesel::sql_types::Text;
use super::author_mask::{AuthorMask, NvcMethods};
use super::book_record::BookStringified;


#[derive(QueryableByName, Debug, Clone, Serialize)]
pub struct TitleMask {
    #[sql_type = "Text"] pub book_title: String,
    #[sql_type = "Text"] pub first_name: String,
    #[sql_type = "Text"] pub middle_name: String,
    #[sql_type = "Text"] pub last_name: String,
}
impl TitleMask {
    pub fn new(title: String) -> Self {
        Self {
            book_title: title,
            first_name: String::new(),
            middle_name: String::new(),
            last_name: String::new(),
        }
    }

    pub fn is_empty(&self) -> bool {
        self.book_title.is_empty()
    }
}

impl NvcMethods for TitleMask {

    fn get_length_by_name(&self, name: &str) -> usize {
        match name {
            "book_title" => self.book_title.chars().count(),
            _ => 0
        }
    }

    fn get_where_like_clause(&self) -> String {
        if self.book_title.is_empty()
        {
            String::new()
        }
        else
        {
            "WHERE ".to_owned() + &format!("book_title LIKE '{}%'", self.book_title)
        }
    }

    fn get_where_explicit_clause(&self) -> String {
        if self.book_title.is_empty()
        {
            String::new()
        }
        else
        {
            "WHERE ".to_owned() + &format!("book_title = '{}%'", self.book_title)
        }
    }
}



#[derive(Debug, Clone, Serialize)]
pub struct FindTitleContext {
    pub root_url: String,
    pub book_title: String,
    pub titles_nvc: Vec<String>,
    pub title_and_author: Vec<TitleMask>,
    pub titles: Vec<String>,
}
impl FindTitleContext {
    pub fn new(url: &str, mask: &TitleMask) -> Self {
        Self {
            root_url: String::from(url),
            book_title: mask.book_title.clone(),
            titles_nvc: Vec::new(),
            title_and_author: Vec::new(),
            titles: Vec::new(),
        }
    }

    pub fn load_title_nvc(&mut self, nvc: Vec<String>) {
        self.titles_nvc = nvc.iter().map(|title|
            format!("<a href='/{}/{}/'> {} </a>",
                self.root_url,
                AuthorMask::encode(title.clone()),
                title)).collect();
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
