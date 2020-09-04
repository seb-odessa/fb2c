use std::convert::From;
use super::schema::authors;
use fb2parser;

pub type Id = i32;
pub type QueryResult<T> = std::result::Result<T, diesel::result::Error>;
pub use diesel::sqlite::SqliteConnection as Connection;

#[derive(Insertable, Queryable)]
#[table_name="authors"]
pub struct AuthorRecord {
    pub id: Id,
    pub first_name: String,
    pub middle_name: String,
    pub last_name: String,
    pub nickname: String,
    pub uuid: String,
}
impl AuthorRecord {
    pub fn load(conn: &Connection, id: Id) -> QueryResult<Self> {
        use crate::schema::authors::dsl::authors;
        use crate::diesel::RunQueryDsl;
        use crate::diesel::QueryDsl;
        authors.find(id).first(conn)
    }

    pub fn find_id(conn: &Connection, name: &AuthorName) -> QueryResult<Id> {
        use crate::schema::authors::dsl::*;
        use crate::diesel::ExpressionMethods;
        use crate::diesel::RunQueryDsl;
        use crate::diesel::QueryDsl;
        authors
            .filter(first_name.eq(&name.first_name))
            .filter(middle_name.eq(&name.middle_name))
            .filter(last_name.eq(&name.last_name))
            .filter(nickname.eq(&name.nickname))
            .filter(uuid.eq(&name.uuid))
            .select(id)
            .first(conn)
    }

    pub fn save(conn: &Connection, name: &AuthorName) -> QueryResult<usize> {
        use crate::diesel::RunQueryDsl;       
        diesel::insert_into(authors::table).values(name).execute(conn)
    }

}

#[derive(Insertable)]
#[table_name="authors"]
#[derive(Eq, PartialEq, Hash, Clone, Debug)]
pub struct AuthorName {
    pub first_name: String,
    pub middle_name: String,
    pub last_name: String,
    pub nickname: String,
    pub uuid: String,
}
impl From<&fb2parser::Author> for AuthorName{
    fn from(src: &fb2parser::Author) -> Self {
        Self {
            first_name: src.get_first_name().unwrap_or_default(),
            middle_name: src.get_middle_name().unwrap_or_default(),
            last_name: src.get_last_name().unwrap_or_default(),
            nickname: src.get_nickname().unwrap_or_default(),
            uuid: src.get_id().unwrap_or_default()
        }
    }
}
impl From<&fb2parser::Translator> for AuthorName{
    fn from(src: &fb2parser::Translator) -> Self {
        Self {
            first_name: src.get_first_name().unwrap_or_default(),
            middle_name: src.get_middle_name().unwrap_or_default(),
            last_name: src.get_last_name().unwrap_or_default(),
            nickname: src.get_nickname().unwrap_or_default(),
            uuid: src.get_id().unwrap_or_default()
        }
    }
}

