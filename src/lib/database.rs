
use std::env;
use std::collections::HashMap;
use std::hash::Hash;
use std::fmt::Debug;

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

pub struct Storage<T: Clone+Eq+Hash+Debug> {
    pub map: HashMap<T, Id>,
    pub count: u64,
    pub hits: u64,
    pub quered: u64,
    pub added: u64,
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

    pub fn report(&self) {
        println!("records total/quered/inserted: {}/{}/{}", self.count, self.quered, self.added);
        println!("cache hits/size: {}/{}", self.hits, self.map.len());
    }

    pub fn add<F,S>(&mut self, conn: &SqliteConnection, value: T, find: F, save: S) -> Id 
        where 
            F: Fn(&SqliteConnection, &T) -> QueryResult<Id>, 
            S: Fn(&SqliteConnection, &T) -> QueryResult<usize> 
    {
        self.count += 1;

        if let Some((_, id)) = self.map.get_key_value(&value) {
            self.hits += 1;
            return *id;
        } else if let Some(id) = find(conn, &value).ok() {
            self.map.insert(value, id);
            self.quered += 1;
            return id;
        } else {            
            save(conn, &value).expect(&format!("Failed to save {:?}", value));
            let id = find(conn, &value).expect(&format!("Failed to query id for {:?}", value));
            self.map.insert(value, id);
            self.added += 1;
            return id;
        }
    }
}

pub struct Manager{
    conn: SqliteConnection,
    pub archives: Storage<ArchiveName>,
    pub authors: Storage<AuthorName>,
    pub titles: Storage<BookName>
}
impl Manager {
    pub fn new() -> Self {
        Self{
            conn: establish_connection(),
            archives: Storage::new(),
            authors: Storage::new(),
            titles: Storage::new(),
        }
    }

    pub fn add_archive(&mut self, archive: ArchiveName) -> Id {
        self.archives.add(&self.conn, archive, ArchiveRecord::find, ArchiveRecord::save)
    }

    pub fn add_book(&mut self, &_arc_id: &Id, fb2: &FictionBook) {
        if let Some(ref title) = fb2.description.title_info.book_title {
            let _tid = self.titles.add(&self.conn, BookName::from(title), BookRecord::find, BookRecord::save);
            for author in &fb2.description.title_info.authors {
                let _aid = self.authors.add(&self.conn, AuthorName::from(author), AuthorRecord::find, AuthorRecord::save);
            }
        }
    }

}