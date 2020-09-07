
pub type Id = i32;
pub type QueryResult<T> = std::result::Result<T, diesel::result::Error>;
pub use diesel::sqlite::SqliteConnection;

pub mod archive;
pub use archive::{ArchiveName, ArchiveRecord};

pub mod author;
pub use author::{AuthorName, AuthorRecord};
pub mod book;
pub use book::{BookName, BookRecord};


