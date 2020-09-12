use crate::zip::read::ZipFile;
use crate::schema::books;
use super::*;

#[derive(Insertable)]
#[table_name="books"]
#[derive(Eq, PartialEq, Hash, Clone, Debug)]
pub struct Book{
    pub archive_id: Id,
    pub name: String,
    pub compressed_size: i64,
    pub size: i64,
    pub crc32: i64,
    pub offset: i64,
}
impl Book {
    pub fn new(archive_id: Id, book: &ZipFile) -> Self {
        Self {
            archive_id: archive_id,
            name: String::from(book.name()),
            compressed_size: book.compressed_size() as i64,
            size: book.size() as i64,
            crc32: book.crc32() as i64,
            offset: book.data_start() as i64,
        }
    }
}

#[derive(Insertable, Queryable, Debug, Clone)]
#[table_name="books"]
pub struct BookRecord {
    pub id: Id,
    pub archive_id: Id,
    pub name: String,
    pub compressed_size: i64,
    pub size: i64,
    pub crc32: i64,
    pub offset: i64,
}
type Base = Book;
type Record = BookRecord;
impl Load<Record> for Record {
    fn load(conn: &SqliteConnection, id: Id) -> QueryResult<Self> {
        use crate::schema::books::dsl::books;
        use crate::diesel::RunQueryDsl;
        use crate::diesel::QueryDsl;
        books.find(id).first(conn)
    }
}
impl Find<Base> for Record {
    fn find(conn: &SqliteConnection, value: &Base) -> QueryResult<Id> {
        use crate::schema::books::dsl::*;
        use crate::diesel::ExpressionMethods;
        use crate::diesel::RunQueryDsl;
        use crate::diesel::QueryDsl;
        books
            .filter(archive_id.eq(&value.archive_id))
            .filter(name.eq(&value.name))
            .select(id)
            .first(conn)
    }
}
impl Save<Base> for Record {
    fn save(conn: &SqliteConnection, value: &Base) -> QueryResult<usize> {
        use crate::diesel::RunQueryDsl;       
        diesel::insert_into(books::table).values(value).execute(conn)
    }
}

#[derive(Clone, Debug)]
pub struct BookDescription{
    pub book: BookRecord,
    pub archive: ArchiveRecord,
    pub title: TitleView,
    pub authors: Vec<AuthorRecord>,
    
}
impl Load<BookDescription> for BookDescription {
    fn load(conn: &SqliteConnection, id: Id) -> QueryResult<Self> {
        let book = BookRecord::load(conn, id) ?;
        let archive = ArchiveRecord::load(conn, book.archive_id) ?;
        let title = TitleView::load(conn, id) ?;
        let authors = AuthorRecord::load_for_book(conn, id) ?;
        
        Ok(Self { book, archive, title, authors})
    }
}