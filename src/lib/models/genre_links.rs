use crate::schema::genre_links;
use super::*;

#[derive(Insertable)]
#[table_name="genre_links"]
#[derive(Eq, PartialEq, Hash, Clone, Debug)]
pub struct GenreLink{
    pub book_id: Id,
    pub genre_id: Id,
}
impl GenreLink{
    pub fn new(book_id: Id, genre_id: Id) -> Self {
        Self { book_id, genre_id }
    }
}

#[derive(Insertable, Queryable)]
#[table_name="genre_links"]
pub struct GenreLinkRecord {
    pub id: Id,
    pub book_id: Id,
    pub genre_id: Id,
}

type Base = GenreLink;
type Record = GenreLinkRecord;

impl Load<Record> for Record {
    fn load(conn: &SqliteConnection, id: Id) -> QueryResult<Self> {
        use crate::schema::genre_links::dsl::genre_links;
        use crate::diesel::RunQueryDsl;
        use crate::diesel::QueryDsl;
        genre_links.find(id).first(conn)
    }
}

impl Find<Base> for Record {
    fn find(conn: &SqliteConnection, value: &Base) -> QueryResult<Id> {
        use crate::schema::genre_links::dsl::*;
        use crate::diesel::ExpressionMethods;
        use crate::diesel::RunQueryDsl;
        use crate::diesel::QueryDsl;
        genre_links
            .filter(book_id.eq(&value.book_id))
            .filter(genre_id.eq(&value.genre_id))
            .select(id).first(conn)
    }
}
impl Save<Base> for Record {
    fn save(conn: &SqliteConnection, value: &Base) -> QueryResult<usize> {
        use crate::diesel::RunQueryDsl;       
        diesel::insert_into(genre_links::table).values(value).execute(conn)
    }
}
