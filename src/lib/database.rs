
pub use diesel::sqlite::SqliteConnection;
use diesel::prelude::Connection;
use dotenv::dotenv;
use std::env;
use std::collections::HashSet;
use fb2parser::FictionBook;
use crate::models::AuthorNew;

pub type QueryResult<T> = std::result::Result<T, diesel::result::Error>;

fn establish_connection() -> SqliteConnection {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    Connection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

pub fn save(conn: &SqliteConnection, fb2: &FictionBook) -> QueryResult<bool> {
    use crate::schema::authors::table;
    use crate::diesel::RunQueryDsl;
    let authors : Vec<AuthorNew>= fb2.description.title_info.authors
        .iter().map(|a|AuthorNew::from(a)).collect();

    diesel::insert_into(table)
        .values(&authors)
        .execute(conn)
        .and_then(|count| Ok(1 == count))
}

pub struct Manager{
    connection: SqliteConnection,
    authors: HashSet<AuthorNew>,
}
impl Manager {
    pub fn new() -> Self {
        Self{
            connection: establish_connection(),
            authors: HashSet::new(),

        }
    }

    pub fn add(&mut self, fb2: &FictionBook) {
        for author in &fb2.description.title_info.authors {
            self.authors.insert(AuthorNew::from(author));
        }
    }

    pub fn flush(self)  -> QueryResult<usize> {
        use crate::schema::authors::table;
        use crate::diesel::RunQueryDsl;
        let authors : Vec<&AuthorNew>= self.authors.iter().collect();
        diesel::insert_into(table).values(authors).execute(&self.connection)
    }

}