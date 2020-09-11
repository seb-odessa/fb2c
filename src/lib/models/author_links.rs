use crate::schema::author_links;
use super::*;

#[derive(Insertable)]
#[table_name="author_links"]
#[derive(Eq, PartialEq, Hash, Clone, Debug)]
pub struct AuthorLink{
    pub book_id: Id,
    pub author_id: Id,
}
impl AuthorLink{
    pub fn new(book_id: Id, author_id: Id) -> Self {
        Self { book_id, author_id }
    }
}

#[derive(Insertable, Queryable)]
#[table_name="author_links"]
pub struct AuthorLinkRecord {
    pub id: Id,
    pub book_id: Id,
    pub author_id: Id,
}

type Base = AuthorLink;
type Record = AuthorLinkRecord;

impl Load<Record> for Record {
    fn load(conn: &SqliteConnection, id: Id) -> QueryResult<Self> {
        use crate::schema::author_links::dsl::author_links;
        use crate::diesel::RunQueryDsl;
        use crate::diesel::QueryDsl;
        author_links.find(id).first(conn)
    }
}

impl Find<Base> for Record {
    fn find(conn: &SqliteConnection, value: &Base) -> QueryResult<Id> {
        use crate::schema::author_links::dsl::*;
        use crate::diesel::ExpressionMethods;
        use crate::diesel::RunQueryDsl;
        use crate::diesel::QueryDsl;
        author_links
            .filter(book_id.eq(&value.book_id))
            .filter(author_id.eq(&value.author_id))
            .select(id).first(conn)
    }
}
impl Save<Base> for Record {
    fn save(conn: &SqliteConnection, value: &Base) -> QueryResult<usize> {
        use crate::diesel::RunQueryDsl;       
        diesel::insert_into(author_links::table).values(value).execute(conn)
    }
}
