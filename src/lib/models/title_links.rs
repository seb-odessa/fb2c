use crate::schema::title_links;
use super::*;

#[derive(Insertable)]
#[table_name="title_links"]
#[derive(Eq, PartialEq, Hash, Clone, Debug)]
pub struct TitleLink{
    pub book_id: Id,
    pub title_id: Id,
}
impl TitleLink{
    pub fn new(book_id: Id, title_id: Id) -> Self {
        Self { book_id, title_id }
    }
}

#[derive(Insertable, Queryable)]
#[table_name="title_links"]
pub struct TitleLinkRecord {
    pub id: Id,
    pub book_id: Id,
    pub title_id: Id,
}

type Base = TitleLink;
type Record = TitleLinkRecord;

impl Load<Record> for Record {
    fn load(conn: &SqliteConnection, id: Id) -> QueryResult<Self> {
        use crate::schema::title_links::dsl::title_links;
        use crate::diesel::RunQueryDsl;
        use crate::diesel::QueryDsl;
        title_links.find(id).first(conn)
    }
}

impl Find<Base> for Record {
    fn find(conn: &SqliteConnection, value: &Base) -> QueryResult<Id> {
        use crate::schema::title_links::dsl::*;
        use crate::diesel::ExpressionMethods;
        use crate::diesel::RunQueryDsl;
        use crate::diesel::QueryDsl;
        title_links
            .filter(book_id.eq(&value.book_id))
            .filter(title_id.eq(&value.title_id))
            .select(id).first(conn)
    }
}
impl Save<Base> for Record {
    fn save(conn: &SqliteConnection, value: &Base) -> QueryResult<usize> {
        use crate::diesel::RunQueryDsl;       
        diesel::insert_into(title_links::table).values(value).execute(conn)
    }
}
