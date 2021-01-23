use crate::zip::read::ZipFile;
use crate::schema::books;
use super::*;

#[derive(Insertable)]
#[table_name="books"]
#[derive(Eq, PartialEq, Hash, Clone, Debug)]
pub struct Book{
    pub arch_id: Id,
    pub book_file: String,
    pub book_zip_size: i64,
    pub book_size: i64,
    pub book_crc32: i64,
    pub book_offset: i64,
}
impl Book {
    pub fn new(archive_id: Id, book: &ZipFile) -> Self {
        Self {
            arch_id: archive_id,
            book_file: String::from(book.name()),
            book_zip_size: book.compressed_size() as i64,
            book_size: book.size() as i64,
            book_crc32: book.crc32() as i64,
            book_offset: book.data_start() as i64,
        }
    }
}

#[derive(Insertable, Queryable, Debug, Clone)]
#[table_name="books"]
pub struct BookRecord {
    pub id: Id,
    pub arch_id: Id,
    pub book_file: String,
    pub book_zip_size: i64,
    pub book_size: i64,
    pub book_crc32: i64,
    pub book_offset: i64,
}
impl BookRecord {
    pub fn find_uniq(conn: &SqliteConnection, aid: Id, book: &str, crc: i64) -> Option<Id> {
        pub use crate::schema::books::dsl::*;
        use crate::diesel::ExpressionMethods;
        use crate::diesel::RunQueryDsl;
        use crate::diesel::QueryDsl;
        books
            .filter(arch_id.eq(&aid))
            .filter(book_file.eq(book))
            .filter(book_crc32.eq(&crc))
            .select(id)
            .first(conn).ok()
    }
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
            .filter(arch_id.eq(&value.arch_id))
            .filter(book_file.eq(&value.book_file))
            .filter(book_zip_size.eq(&value.book_zip_size))
            .filter(book_size.eq(&value.book_size))
            .filter(book_crc32.eq(&value.book_crc32))
            .filter(book_offset.eq(&value.book_offset))
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
        let archive = ArchiveRecord::load(conn, book.arch_id) ?;
        let title = TitleView::load(conn, id) ?;
        let authors = AuthorRecord::load_for_book(conn, id) ?;
        
        Ok(Self { book, archive, title, authors})
    }
}