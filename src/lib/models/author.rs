use std::convert::From;
use crate::schema::authors;
use super::*;

#[derive(Insertable)]
#[table_name="authors"]
#[derive(Eq, PartialEq, Hash, Clone, Debug)]
pub struct Author {
    pub first_name: String,
    pub middle_name: String,
    pub last_name: String,
    pub nickname: String,
    pub uuid: String,
}
impl From<&fb2parser::Author> for Author{
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
impl From<&fb2parser::Translator> for Author{
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

#[derive(Insertable, Queryable, Debug, Clone)]
#[table_name="authors"]
pub struct AuthorRecord {
    pub id: Id,
    pub first_name: String,
    pub middle_name: String,
    pub last_name: String,
    pub nickname: String,
    pub uuid: String,
}

type Base = Author;
type Record = AuthorRecord;
impl Load<Record> for Record {
    fn load(conn: &SqliteConnection, id: Id) -> QueryResult<Self> {
        use crate::schema::authors::dsl::authors;
        use crate::diesel::RunQueryDsl;
        use crate::diesel::QueryDsl;
        authors.find(id).first(conn)
    }
}
impl ForBook<Record> for Record {
    fn load_for_book(conn: &SqliteConnection, book: Id) -> QueryResult<Vec<Self>>
    {
        use crate::schema_views::authors_view::dsl::*;
        use crate::diesel::RunQueryDsl;
        use crate::diesel::QueryDsl;
        use crate::diesel::ExpressionMethods;
        authors_view
            .filter(book_id.eq(&book))
            .select((id, first_name, middle_name, last_name, nickname, uuid))
            .load(conn)
    }
}

impl Find<Base> for Record {
    fn find(conn: &SqliteConnection, value: &Base) -> QueryResult<Id> {
        use crate::schema::authors::dsl::*;
        use crate::diesel::ExpressionMethods;
        use crate::diesel::RunQueryDsl;
        use crate::diesel::QueryDsl;
        authors
            .filter(first_name.eq(&value.first_name))
            .filter(middle_name.eq(&value.middle_name))
            .filter(last_name.eq(&value.last_name))
            .filter(nickname.eq(&value.nickname))
            .filter(uuid.eq(&value.uuid))
            .select(id)
            .first(conn)
    }
}
impl Save<Base> for Record {
    fn save(conn: &SqliteConnection, value: &Base) -> QueryResult<usize> {
        use crate::diesel::RunQueryDsl;       
        diesel::insert_into(authors::table).values(value).execute(conn)
    }
}
