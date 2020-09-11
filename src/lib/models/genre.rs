use std::convert::From;
use crate::schema::genres;
use super::*;

#[derive(Insertable)]
#[table_name="genres"]
#[derive(Eq, PartialEq, Hash, Clone, Debug)]
pub struct Genre{
    pub name: String
}
impl From<&fb2parser::Genre> for Genre {
    fn from(src: &fb2parser::Genre) -> Self {
        Self {
            name: src.text.clone(),
        }
    }
}

#[derive(Insertable, Queryable)]
#[table_name="genres"]
pub struct GenreRecord {
    pub id: Id,
    pub name: String,
}
type Base = Genre;
type Record = GenreRecord;
impl Load<Record> for Record {
    fn load(conn: &SqliteConnection, id: Id) -> QueryResult<Self> {
        use crate::schema::genres::dsl::genres;
        use crate::diesel::RunQueryDsl;
        use crate::diesel::QueryDsl;
        genres.find(id).first(conn)
    }
}
impl Find<Base> for Record {
    fn find(conn: &SqliteConnection, value: &Base) -> QueryResult<Id> {
        use crate::schema::genres::dsl::*;
        use crate::diesel::ExpressionMethods;
        use crate::diesel::RunQueryDsl;
        use crate::diesel::QueryDsl;
        genres.filter(name.eq(&value.name)).select(id).first(conn)
    }
}
impl Save<Base> for Record {
    fn save(conn: &SqliteConnection, value: &Base) -> QueryResult<usize> {
        use crate::diesel::RunQueryDsl;       
        diesel::insert_into(genres::table).values(value).execute(conn)
    }
}
