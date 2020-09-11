
pub type Id = i32;
pub type QueryResult<T> = std::result::Result<T, diesel::result::Error>;
pub use diesel::sqlite::SqliteConnection;

pub trait Find<T> {
    fn find(conn: &SqliteConnection, value: &T) -> QueryResult<Id>;
}

pub trait Load<T> {
    fn load(conn: &SqliteConnection, id: Id) -> QueryResult<T>;
}

pub trait Save<T> {
    fn save(conn: &SqliteConnection, value: &T) -> QueryResult<usize>;
}

pub mod archive;
pub use archive::{Archive, ArchiveRecord};
pub mod book;
pub use book::{Book, BookRecord};
pub mod genre;
pub use genre::{Genre, GenreRecord};
pub mod author;
pub use author::{Author, AuthorRecord};
pub mod title;
pub use title::{Title, TitleRecord};

pub mod title_links;
pub use title_links::*;
pub mod author_links;
pub use author_links::*;
pub mod genre_links;
pub use genre_links::*;

