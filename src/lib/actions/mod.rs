use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};
use diesel::sql_query;
use diesel::sql_types::Text;
use serde::Serialize;

pub type QueryResult<T> = std::result::Result<T, diesel::result::Error>;
pub type ConnectionPool = r2d2::Pool<ConnectionManager<SqliteConnection>>;

pub fn get_connection_pool() -> ConnectionPool {
    let connspec = std::env::var("DATABASE_URL").expect("DATABASE_URL");
    let manager = ConnectionManager::<SqliteConnection>::new(connspec);
    r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.")
}

pub fn get_next_valid_chars(conn: &SqliteConnection, table: &str, column: &str, chars: String) -> QueryResult<Vec<String>>
{
    #[derive(QueryableByName, Debug, Clone)]
    pub struct DbString {
        #[sql_type = "Text"] pub content: String,
    }
    let query = format!(r#"
        SELECT DISTINCT substr({column}, 1, {len}) AS content
        FROM {table}
        WHERE {column} LIKE '{chars}%' ORDER BY content"#,
            table = table,
            column = column,
            chars = chars,
            len = 1 + chars.chars().count());

    sql_query(&query)
        .load::<DbString>(conn)
        .map(|list|
            list.iter().map(|s|
                s.content.clone()).collect()
            )
}


#[derive(QueryableByName, Debug, Clone, Serialize)]
pub struct AuthorMask {
    #[sql_type = "Text"] pub first_name: String,
    #[sql_type = "Text"] pub middle_name: String,
    #[sql_type = "Text"] pub last_name: String,
}
impl AuthorMask {
    pub fn new(first_name: String, middle_name: String, last_name: String) -> Self {
        Self {
            first_name: Self::decode(first_name),
            middle_name: Self::decode(middle_name),
            last_name: Self::decode(last_name)
        }
    }

    fn decode(mask: String) -> String {
        if mask.starts_with("-") {
            String::new()
        } else {
            mask
        }
    }

    pub fn get_where_clause(&self) -> String {
        let mut clauses = Vec::new();
        if !self.first_name.is_empty()
        {
            clauses.push(format!("first_name LIKE '{}%'", self.first_name));
        }
        if !self.middle_name.is_empty() {
            if !clauses.is_empty() {
                clauses.push("AND".to_owned());
            }
            clauses.push(format!("middle_name LIKE '{}%'", self.middle_name));
        }
        if !self.last_name.is_empty() {
            if !clauses.is_empty() {
                clauses.push("AND".to_owned());
            }
            clauses.push(format!("last_name LIKE '{}%'", self.last_name));
        }

        return if clauses.is_empty() {
           String::new()
        } else {
            "WHERE ".to_owned() + &clauses.join(" ")
        }
    }
}


pub fn search_authors(conn: &SqliteConnection, author: &AuthorMask) -> QueryResult<Vec<AuthorMask>>
{
    let query = format!(
        r#"
            SELECT DISTINCT first_name, middle_name, last_name
            FROM authors {where_clause}
            ORDER BY last_name, first_name, middle_name;
        "#,
        where_clause = author.get_where_clause());

    println!("{}", query);
    sql_query(&query).load(conn)
}


pub fn find_user_by_uid(id: u32, _conn: &SqliteConnection) -> QueryResult<Option<String>>
{
    let r = match id {
        42 => "42",
        24 => "24",
        _ => "Unknown",
    };
    Ok(Some(String::from(r)))
}

