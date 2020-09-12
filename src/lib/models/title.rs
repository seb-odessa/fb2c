use std::convert::From;
use crate::schema::titles;
use super::*;

#[derive(Insertable)]
#[table_name="titles"]
#[derive(Eq, PartialEq, Hash, Clone, Debug)]
pub struct Title {
    pub book_title: String,
}
impl From<&fb2parser::BookTitle> for Title{
    fn from(src: &fb2parser::BookTitle) -> Self {
        Self {
            book_title: src.text.clone()
        }
    }
}

#[derive(Insertable, Queryable, Debug, Clone)]
#[table_name="titles"]
pub struct TitleRecord {
    pub id: Id,
    pub book_title: String,
}

type Base = Title;
type Record = TitleRecord;
impl Load<Record> for Record {
    fn load(conn: &SqliteConnection, id: Id) -> QueryResult<Self> {
        use crate::schema::titles::dsl::titles;
        use crate::diesel::RunQueryDsl;
        use crate::diesel::QueryDsl;
        titles.find(id).first(conn)
    }
}
impl Find<Base> for Record {
    fn find(conn: &SqliteConnection, value: &Base) -> QueryResult<Id> {
        use crate::schema::titles::dsl::*;
        use crate::diesel::ExpressionMethods;
        use crate::diesel::RunQueryDsl;
        use crate::diesel::QueryDsl;
        titles
            .filter(book_title.eq(&value.book_title))
            .select(id)
            .first(conn)
    }
}
impl Save<Base> for Record {
    fn save(conn: &SqliteConnection, value: &Base) -> QueryResult<usize> {
        use crate::diesel::RunQueryDsl;       
        diesel::insert_into(titles::table).values(value).execute(conn)
    }
}

#[derive(Queryable, Debug, Clone)]
pub struct TitleView {
    pub id: Id,
    pub title: String,
}
impl Load<TitleView> for TitleView {
    fn load(conn: &SqliteConnection, id: Id) -> QueryResult<Self> {
        use crate::schema_views::titles_view::dsl::titles_view;
        use crate::diesel::RunQueryDsl;
        use crate::diesel::QueryDsl;
        titles_view.find(id).first(conn)
    }
}