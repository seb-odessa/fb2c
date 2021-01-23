use std::convert::From;
use crate::schema::genres;
use super::*;

#[derive(Insertable)]
#[table_name="genres"]
#[derive(Eq, PartialEq, Hash, Clone, Debug)]
pub struct Genre{
    pub genre_name: String
}
impl From<&fb2parser::Genre> for Genre {
    fn from(src: &fb2parser::Genre) -> Self {
        Self {
            genre_name: src.text.clone(),
        }
    }
}

#[derive(Insertable, Queryable, Debug, Clone)]
#[table_name="genres"]
pub struct GenreRecord {
    pub id: Id,
    pub genre_name: String,
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
        genres.filter(genre_name.eq(&value.genre_name)).select(id).first(conn)
    }
}
impl Save<Base> for Record {
    fn save(conn: &SqliteConnection, value: &Base) -> QueryResult<usize> {
        use crate::diesel::RunQueryDsl;       
        diesel::insert_into(genres::table).values(value).execute(conn)
    }
}

#[derive(Queryable)]
pub struct GenreView {
    pub id: Id,
    pub code: String,
    pub name: String,
    pub group: String,
}
impl Load<GenreView> for GenreView {
    fn load(conn: &SqliteConnection, id: Id) -> QueryResult<Self> {
        use crate::schema_views::genres_view::dsl::genres_view;
        use crate::diesel::RunQueryDsl;
        use crate::diesel::QueryDsl;
        genres_view.find(id).first(conn)
    }
}