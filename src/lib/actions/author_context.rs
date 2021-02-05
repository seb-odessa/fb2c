use serde::Serialize;
use super::author_mask::AuthorMask;

#[derive(Debug, Clone, Serialize)]
pub struct FindAuthorContext {
    pub first_name: String,
    pub first_name_previous: String,
    pub middle_name: String,
    pub middle_name_previous: String,
    pub last_name: String,
    pub last_name_previous: String,
    pub root_url: String,
    pub authors: Vec<String>,
    pub first_name_nvc: Vec<String>,
    pub middle_name_nvc: Vec<String>,
    pub last_name_nvc: Vec<String>,
}
impl FindAuthorContext {

    pub fn new(url: &str, mask: &AuthorMask) -> Self {
        Self {
            first_name: mask.get_encoded_by_name("first_name"),
            first_name_previous: mask.get_encoded_by_name_previous("first_name"),
            middle_name: mask.get_encoded_by_name("middle_name"),
            middle_name_previous: mask.get_encoded_by_name_previous("middle_name"),
            last_name: mask.get_encoded_by_name("last_name"),
            last_name_previous: mask.get_encoded_by_name_previous("last_name"),
            root_url: String::from(url),
            authors: Vec::new(),
            first_name_nvc: Vec::new(),
            middle_name_nvc: Vec::new(),
            last_name_nvc: Vec::new(),
        }
    }

    pub fn load_first_name_nvc(&mut self, nvc: Vec<String>) {
        self.first_name_nvc = nvc.iter().map(|first|
            format!("<a href='/{}/{}/{}/{}/'>{}</a>",
                self.root_url,
                AuthorMask::encode(first.clone()),
                AuthorMask::encode(self.middle_name.clone()),
                AuthorMask::encode(self.last_name.clone()),
                first)).collect();
    }

    pub fn load_middle_name_nvc(&mut self, nvc: Vec<String>) {
        self.middle_name_nvc = nvc.iter().map(|middle|
            format!("<a href='/{}/{}/{}/{}/'>{}</a>",
                self.root_url,
                AuthorMask::encode(self.first_name.clone()),
                AuthorMask::encode(middle.clone()),
                AuthorMask::encode(self.last_name.clone()),
                middle)).collect();
    }

    pub fn load_last_name_nvc(&mut self, nvc: Vec<String>) {
        self.last_name_nvc = nvc.iter().map(|last|
            format!("<a href='/{}/{}/{}/{}/'>{}</a>",
                self.root_url,
                AuthorMask::encode(self.first_name.clone()),
                AuthorMask::encode(self.middle_name.clone()),
                AuthorMask::encode(last.clone()),
                last)).collect();
    }

}

#[derive(Debug, Clone, Serialize)]
pub struct AuthorContext {
    pub first_name: String,
    pub middle_name: String,
    pub last_name: String,
    pub root_url: String,
    pub titles: Vec<String>,
}
impl AuthorContext {
    pub fn new(url: &str, mask: &AuthorMask) -> Self {
        Self {
            first_name: mask.first_name.clone(),
            middle_name: mask.middle_name.clone(),
            last_name: mask.last_name.clone(),
            root_url: String::from(url),
            titles: Vec::new()
        }
    }
}

