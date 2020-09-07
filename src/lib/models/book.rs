use std::convert::From;
use crate::schema::books;
use super::*;


#[derive(Insertable, Queryable)]
#[table_name="books"]
pub struct BookRecord {
    pub id: Id,
    pub book_title: String,
}
impl BookRecord {
    pub fn load(conn: &SqliteConnection, id: Id) -> QueryResult<Self> {
        use crate::schema::books::dsl::books;
        use crate::diesel::RunQueryDsl;
        use crate::diesel::QueryDsl;
        books.find(id).first(conn)
    }

    pub fn find(conn: &SqliteConnection, value: &BookName) -> QueryResult<Id> {
        use crate::schema::books::dsl::*;
        use crate::diesel::ExpressionMethods;
        use crate::diesel::RunQueryDsl;
        use crate::diesel::QueryDsl;
        books
            .filter(book_title.eq(&value.book_title))
            .select(id)
            .first(conn)
    }

    pub fn save(conn: &SqliteConnection, value: &BookName) -> QueryResult<usize> {
        use crate::diesel::RunQueryDsl;       
        diesel::insert_into(books::table).values(value).execute(conn)
    }
}

#[derive(Insertable)]
#[table_name="books"]
#[derive(Eq, PartialEq, Hash, Clone, Debug)]
pub struct BookName {
    pub book_title: String,
}
impl From<&fb2parser::BookTitle> for BookName{
    fn from(src: &fb2parser::BookTitle) -> Self {
        Self {
            book_title: src.text.clone()
        }
    }
}

