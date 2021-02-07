use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct RootContext {
    pub first_name_nvc: Vec<String>,
    pub middle_name_nvc: Vec<String>,
    pub last_name_nvc: Vec<String>,
    pub book_title_nvc: Vec<String>,
}
impl RootContext {
    pub fn new() -> Self {
        Self {
            first_name_nvc: Vec::new(),
            middle_name_nvc: Vec::new(),
            last_name_nvc: Vec::new(),
            book_title_nvc: Vec::new(),
        }
    }
}
