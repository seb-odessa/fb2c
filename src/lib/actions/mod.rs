use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};

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
    use diesel::sql_query;
    use diesel::sql_types::Text;

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

pub fn find_user_by_uid(id: u32, _conn: &SqliteConnection) -> QueryResult<Option<String>>
{
    let r = match id {
        42 => "42",
        24 => "24",
        _ => "Unknown",
    };
    Ok(Some(String::from(r)))
}

