use serde::Serialize;
use super::author_mask::AuthorMask;

#[derive(Debug, Clone, Serialize)]
pub struct RootContext {
    pub first_name_nvc: Vec<String>,
    pub middle_name_nvc: Vec<String>,
    pub last_name_nvc: Vec<String>,
}
impl RootContext {
    pub fn new() -> Self {
        Self {
            first_name_nvc: Vec::new(),
            middle_name_nvc: Vec::new(),
            last_name_nvc: Vec::new(),
        }
    }

    pub fn load_first_name_nvc(&mut self, nvc: Vec<String>) {
        self.first_name_nvc = nvc.iter().map(|value|
            format!("<a href='/find_authors/{}/-/-/'>{}</a>",
                AuthorMask::encode(value.clone()),
                value)).collect();
    }

    pub fn load_middle_name_nvc(&mut self, nvc: Vec<String>) {
        self.middle_name_nvc = nvc.iter().map(|value|
            format!("<a href='/find_authors/-/{}/-/'>{}</a>",
                AuthorMask::encode(value.clone()),
                value)).collect();
    }

    pub fn load_last_name_nvc(&mut self, nvc: Vec<String>) {
        self.last_name_nvc = nvc.iter().map(|value|
            format!("<a href='/find_authors/-/-/{}/'>{}</a>",
                AuthorMask::encode(value.clone()),
                value)).collect();
    }
}
