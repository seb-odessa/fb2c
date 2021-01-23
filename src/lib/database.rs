
use std::env;
use std::collections::HashMap;
use std::hash::Hash;
use std::fmt::Debug;

use crate::zip::read::ZipFile;
use dotenv::dotenv;
use fb2parser::FictionBook;

use crate::models::*;

fn establish_connection() -> SqliteConnection {
    use diesel::prelude::Connection;
    use crate::diesel::connection::SimpleConnection;

    dotenv().ok();
    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    let conn: SqliteConnection = Connection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url));

    let queries = vec![
        "PRAGMA cache_size = -262144;   /* 256 * 1024 Kb = 256 Mb */",
        "PRAGMA journal_mode = MEMORY;  /* Fast but unsave journal */ ",
        "PRAGMA synchronous = OFF ",
    ];    
    for query in &queries {
        conn.batch_execute(query).expect(&format!("Can't execute: {}", query));
    }

    return conn;
}

pub enum SaveResult {
    CacheHit(Id),
    Quered(Id),
    Added(Id),
}
impl SaveResult{
    pub fn get_id(&self) -> Id {
        *match self {
            SaveResult::CacheHit(id) => id,
            SaveResult::Quered(id) => id,
            SaveResult::Added(id) =>id,
        }
    }    
}

pub struct Storage<T: Clone+Eq+Hash+Debug> {
    pub map: HashMap<T, Id>,
    pub count: u64,
    pub hits: u64,
    pub quered: u64,
    pub added: u64,
}
impl <T: Clone+Eq+Hash+Debug> Drop for Storage<T> {
    fn drop(&mut self) {
        let inner_type = std::any::type_name::<T>().rsplit(':').take(1).next();
        println!("Storage<{:>12}> hits/size: {:>5}/{:>6}, quered/inserted/total: {:>6}/{:>6}/{:>6}", 
            inner_type.unwrap(),
            self.hits, self.map.len(),
            self.quered, self.added, self.count
        );
    }
}
impl <T: Clone+Eq+Hash+Debug> Storage<T> {
    pub fn new() -> Self {
        Self{
            map: HashMap::new(),
            count: 0,
            hits: 0,
            quered:0,
            added: 0,
        }
    }

    pub fn save<Record>(&mut self, conn: &SqliteConnection, value: T) -> SaveResult 
    where 
        Record: Find<T> + Save<T>
    {
        self.count += 1;

        if let Some((_, id)) = self.map.get_key_value(&value) {
            self.hits += 1;
            return SaveResult::CacheHit(*id);
        } else if let Some(id) = Record::find(conn, &value).ok() {
            self.map.insert(value, id);
            self.quered += 1;
            return SaveResult::Quered(id);
        } else {            
            Record::save(conn, &value).expect(&format!("Failed to save {:?}", value));
            let id = Record::find(conn, &value).expect(&format!("Failed to query id for {:?}", value));
            self.map.insert(value, id);
            self.added += 1;
            return SaveResult::Added(id);
        }
    }

}

pub struct Manager{
    conn: SqliteConnection,
    pub archives: Storage<Archive>,
    pub books: Storage<Book>,
    pub authors: Storage<Author>,
    pub author_links: Storage<AuthorLink>,
    pub titles: Storage<Title>,
    pub title_links: Storage<TitleLink>,
    pub genres: Storage<Genre>,
    pub genre_links: Storage<GenreLink>,
}
impl Manager {
    pub fn new() -> Self {
        Self{
            conn: establish_connection(),
            archives: Storage::new(),
            books: Storage::new(),
            authors: Storage::new(),
            author_links: Storage::new(),
            titles: Storage::new(),
            title_links: Storage::new(),
            genres: Storage::new(),
            genre_links: Storage::new(),
        }
    }

    pub fn find_archive(&self, uuid: &String) -> Option<Id> {
        ArchiveRecord::find_uniq(&self.conn, uuid)
    }

    pub fn find_book(&self, archive_id: Id, name: &str, crc: i64) -> Option<Id> {
        BookRecord::find_uniq(&self.conn, archive_id, name, crc)
    }

    pub fn save_archive(&mut self, archive: Archive) -> SaveResult {
        self.archives.save::<ArchiveRecord>(&self.conn, archive)
    }

    pub fn save_book(&mut self, arc_id: Id, file: &ZipFile) -> SaveResult {
        self.books.save::<BookRecord>(&self.conn, Book::new(arc_id, file))
    }

    pub fn save_content(&mut self, book_id: Id, fb2: &FictionBook) {
        let conn = &self.conn;
        if let Some(ref title) = fb2.description.title_info.book_title {
            {
                let id = self.titles.save::<TitleRecord>(conn, Title::from(title)).get_id();
                self.title_links.save::<TitleLinkRecord>(conn, TitleLink::new(book_id, id));    
            }
            for author in &fb2.get_authors() {
                let id = self.authors.save::<AuthorRecord>(&self.conn, Author::from(author)).get_id();
                self.author_links.save::<AuthorLinkRecord>(conn, AuthorLink::new(book_id, id));
            }
            for genre in &fb2.get_genres() {
                let id = self.genres.save::<GenreRecord>(conn, Genre::from(genre)).get_id();
                self.genre_links.save::<GenreLinkRecord>(conn, GenreLink::new(book_id, id));
            }
        }
    }

}