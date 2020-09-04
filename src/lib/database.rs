
use std::env;
use std::collections::HashMap;
use dotenv::dotenv;
use fb2parser::FictionBook;
use fb2parser::Author;

use crate::models::{AuthorRecord, AuthorName, Connection, Id};

fn establish_connection() -> Connection {
    use diesel::prelude::Connection;
    dotenv().ok();
    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    Connection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

pub struct Manager{
    connection: Connection,
    authors: HashMap<AuthorName, Id>,
    pub author_count: u32,
    pub author_cache_hit: u32,
    pub author_quered: u32,
    pub author_inserted: u32,
}
impl Manager {
    pub fn new() -> Self {
        Self{
            connection: establish_connection(),
            authors: HashMap::new(),
            author_count: 0,
            author_cache_hit: 0,
            author_quered: 0,
            author_inserted: 0,            
        }
    }

    pub fn handle(&mut self, fb2: &FictionBook) {
        for author in &fb2.description.title_info.authors {
            let _id = self.add_author(&author);
        }
    }

    pub fn add_author(&mut self, author: &Author) -> Id {
        self.author_count += 1;
        let name = AuthorName::from(author);
        if let Some((_, id)) = self.authors.get_key_value(&name) {
            self.author_cache_hit += 1;
            return *id;
        } else if let Some(id) = AuthorRecord::find_id(&self.connection, &name).ok() {
            self.authors.insert(name, id);
            self.author_quered += 1;
            return id;
        } else {            
            AuthorRecord::save(&self.connection, &name)
                .expect(&format!("Failed to save author {:?}", name));
            let id = AuthorRecord::find_id(&self.connection, &name)
                .expect(&format!("Failed to query author id for {:?}", name));
            
            self.author_inserted += 1;
            self.authors.insert(name, id);
            return id;
        }
    }
}