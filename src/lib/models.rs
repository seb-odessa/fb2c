use std::convert::From;
use super::schema::authors;
use super::schema::emails;
//use super::schema::homepages;
use fb2parser::Author;
use fb2parser::Translator;

#[derive(Queryable)]
pub struct AuthorQuery {
    pub id: i64,
    pub first_name: Option<String>,
    pub middle_name: Option<String>,
    pub last_name: Option<String>,
    pub nickname: Option<String>,
    pub lib_id: Option<String>,
}


#[derive(Insertable, PartialEq, Eq, Hash)]
#[table_name="authors"]
pub struct AuthorNew {
    pub first_name: Option<String>,
    pub middle_name: Option<String>,
    pub last_name: Option<String>,
    pub nickname: Option<String>,
    pub lib_id: Option<String>,
}
impl From<&Author> for AuthorNew{
    fn from(src: &Author) -> Self {
        Self {
            first_name: src.get_first_name(),
            middle_name: src.get_middle_name(),
            last_name: src.get_last_name(),
            nickname: src.get_nickname(),
            lib_id: src.get_id()
        }
    }
}
impl From<&Translator> for AuthorNew{
    fn from(src: &Translator) -> Self {
        Self {
            first_name: src.get_first_name(),
            middle_name: src.get_middle_name(),
            last_name: src.get_last_name(),
            nickname: src.get_nickname(),
            lib_id: src.get_id()
        }
    }
}


#[derive(Queryable)]
pub struct EmailQuery {
    pub id: i64,
    pub owner: i64,
    pub email: String,
}
#[derive(Insertable)]
#[table_name="emails"]
pub struct NewEmail<'a> {
    pub owner: i32,
    pub email: &'a str,
}

#[derive(Queryable)]
pub struct HomepageQuery {
    pub id: i32,
    pub owner: i32,
    pub homepage: String,
}
// #[derive(Insertable)]
// #[table_name="homepages"]
// pub struct NewHomePage<'a> {
//     pub owner: i32,
//     pub homepage: &'a str,
// }