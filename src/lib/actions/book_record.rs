use diesel::prelude::*;
use diesel::sql_query;
use diesel::sql_types::Text;
use diesel::sql_types::Integer;
use num_format::{Locale, ToFormattedString};

use serde::Serialize;
use super::QueryResult;
use super::AuthorMask;
use super::SqliteConnection;


#[derive(QueryableByName, Debug, Clone, Serialize)]
pub struct BookRecord {
    #[sql_type = "Text"] pub book_title: String,
    #[sql_type = "Text"] pub book_file: String,

    #[sql_type = "Integer"] pub book_size: i32,
    #[sql_type = "Integer"] pub book_crc32: i32,

    #[sql_type = "Text"] pub arch_name: String,
    #[sql_type = "Text"] pub arch_home: String,
}
impl BookRecord {
    pub fn load_by_author_and_title(conn: &SqliteConnection, author: &AuthorMask, title: &String) -> QueryResult<Vec<Self>>{
        let query = format!(
            r#"
            SELECT book_title, book_file, book_size, book_crc32, arch_name, arch_home
            FROM title_links
            JOIN author_links ON (author_links.book_id = title_links.book_id)
            LEFT JOIN authors ON (author_links.author_id = authors.id)
            LEFT JOIN titles ON (title_links.title_id = titles.id)
            LEFT JOIN books ON (title_links.book_id = books.id)
            LEFT JOIN archives ON (books.arch_id = archives.id)
            {where_clause}
            AND book_title = '{title}'
            "#,
            where_clause = author.get_where_explicit_clause(),
            title = title
        );

        sql_query(&query).load(conn)
    }

    pub fn load_by_archive_and_book(conn: &SqliteConnection, archive: &String, book: &String)-> QueryResult<BookRecord> {
        let query = format!(
            r#"
            SELECT book_title, book_file, book_size, book_crc32, arch_name, arch_home
            FROM title_links
            LEFT JOIN titles ON (title_links.title_id = titles.id)
            LEFT JOIN books ON (title_links.book_id = books.id)
            LEFT JOIN archives ON (books.arch_id = archives.id)
            WHERE arch_name = '{archive}' and book_file = '{file}'
            "#,
            archive = archive,
            file = book
        );

        let records: Vec<BookRecord> = sql_query(&query).load(conn)?;
        if let Some(record) = records.first() {
            Ok(record.clone())
        } else {
            use diesel::result::Error;
            QueryResult::Err(Error::NotFound)
        }

    }

}

#[derive(Debug, Clone, Serialize)]
pub struct BookStringified {
    pub book_url: String,
    pub book_zip_url: String,
    pub book_title: String,
    pub book_file: String,
    pub book_size: String,
    pub book_crc32: String,
    pub arch_name: String,
    pub arch_home: String,
}
impl BookStringified {
    pub fn transform(books: Vec<BookRecord>) -> Vec<Self> {
        let mut result = Vec::new();
        for book in books {
            result.push(Self {
                book_url: format!("<a href='/download/{}/{}'>fb2</a>", book.arch_name, book.book_file),
                book_zip_url: format!("<a href='/download_zip/{}/{}'>fb2.zip</a>", book.arch_name, book.book_file),
                book_title: book.book_title,
                book_file: book.book_file,
                book_size: format!("{}", book.book_size.to_formatted_string(&Locale::fr)),
                book_crc32: format!("{:#02X}", book.book_crc32),
                arch_name: book.arch_name,
                arch_home: book.arch_home
            });
        }
        return result;
    }
}
