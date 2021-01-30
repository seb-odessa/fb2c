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

pub fn get_next_valid_chars(conn: &SqliteConnection, table: &str, column: &str, chars: &String) -> QueryResult<Vec<String>>
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

pub fn get_next_valid_authors(conn: &SqliteConnection, column: &str, mask: &AuthorMask) -> QueryResult<Vec<String>>
{
    #[derive(QueryableByName, Debug, Clone)]
    pub struct DbString {
        #[sql_type = "Text"] pub content: String,
    }
    let query = format!(r#"
        SELECT DISTINCT substr({column}, 1, {len}) AS content
        FROM authors {where_clause}
        ORDER BY content"#,
            column = column,
            where_clause = mask.get_where_clause(),
            len = mask.get_length_by_name(column)
    );

    sql_query(&query)
        .load::<DbString>(conn)
        .map(|list|
            list.iter().map(|s|
                s.content.clone()).collect()
            )
}


pub fn search_authors(conn: &SqliteConnection, mask: &AuthorMask) -> QueryResult<Vec<AuthorMask>>
{
    let query = format!(
        r#"
            SELECT DISTINCT first_name, middle_name, last_name
            FROM authors {where_clause}
            ORDER BY last_name, first_name, middle_name
            LIMIT 100
        "#,
        where_clause = mask.get_where_clause());

    println!("{}", query);
    sql_query(&query).load(conn)
}


#[derive(QueryableByName, Debug, Clone, Serialize)]
pub struct AuthorMask {
    #[sql_type = "Text"] pub first_name: String,
    #[sql_type = "Text"] pub middle_name: String,
    #[sql_type = "Text"] pub last_name: String,
}
impl AuthorMask {
    pub fn decode(mask: String) -> String {
        if mask.starts_with("-") {
            String::new()
        } else {
            mask
        }
    }

    pub fn encode(val: String) -> String {
        if val.is_empty() {
            String::from("-")
        } else {
            val
        }
    }

    pub fn get_length_by_name(&self, filed_name: &str) -> usize {
        match filed_name {
            "first_name" => self.first_name.len(),
            "middle_name" => self.middle_name.len(),
            "last_name" => self.last_name.len(),
            _ => 0
        }
    }


    pub fn new(first_name: String, middle_name: String, last_name: String) -> Self {
        Self {
            first_name: Self::decode(first_name),
            middle_name: Self::decode(middle_name),
            last_name: Self::decode(last_name),
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


pub fn get_author_ctx(conn: &SqliteConnection, mask: &AuthorMask) -> QueryResult<AuthorsContext> {

    let mut ctx = AuthorsContext::new(mask.first_name.clone(), mask.middle_name.clone(), mask.last_name.clone());
    ctx.authors = search_authors(conn, &mask)?;

    ctx.load_first_name_nvc(get_next_valid_authors(conn, "first_name", &mask)?);
    ctx.load_middle_name_nvc(get_next_valid_authors(conn, "middle_name", &mask)?);
    ctx.load_last_name_nvc(get_next_valid_authors(conn, "last_name", &mask)?);

    return Ok(ctx);
}


#[derive(Debug, Clone, Serialize)]
pub struct AuthorsContext {
    pub first_name: String,
    pub middle_name: String,
    pub last_name: String,

    pub authors: Vec<AuthorMask>,
    pub first_name_nvc: Vec<String>,
    pub middle_name_nvc: Vec<String>,
    pub last_name_nvc: Vec<String>,
}
impl AuthorsContext {

    pub fn new(first_name: String, middle_name: String, last_name: String) -> Self {
        Self {
            first_name: first_name,
            middle_name: middle_name,
            last_name: last_name,
            authors: Vec::new(),
            first_name_nvc: Vec::new(),
            middle_name_nvc: Vec::new(),
            last_name_nvc: Vec::new(),
        }
    }

    pub fn load_first_name_nvc(&mut self, nvc: Vec<String>) {
        self.first_name_nvc = nvc.iter().map(|first|
            format!("<a href='/authors/{}/{}/{}/'>{}</a>",
                AuthorMask::encode(first.clone()),
                AuthorMask::encode(self.middle_name.clone()),
                AuthorMask::encode(self.last_name.clone()),
                first)).collect();
    }

    pub fn load_middle_name_nvc(&mut self, nvc: Vec<String>) {
        self.middle_name_nvc = nvc.iter().map(|middle|
            format!("<a href='/authors/{}/{}/{}/'>{}</a>",
                AuthorMask::encode(self.first_name.clone()),
                AuthorMask::encode(middle.clone()),
                AuthorMask::encode(self.last_name.clone()),
                middle)).collect();
    }

    pub fn load_last_name_nvc(&mut self, nvc: Vec<String>) {
        self.last_name_nvc = nvc.iter().map(|last|
            format!("<a href='/authors/{}/{}/{}/'>{}</a>",
                AuthorMask::encode(self.first_name.clone()),
                AuthorMask::encode(self.middle_name.clone()),
                AuthorMask::encode(last.clone()),
                last)).collect();
    }

}


#[derive(Debug, Clone, Serialize)]
pub struct RootContext {
    pub first_name_nvc: Vec<String>,
    pub middle_name_nvc: Vec<String>,
    pub last_name_nvc: Vec<String>,
}
impl RootContext {
    pub fn new() -> Self {
        Self {
            first_name_nvc: Vec::new(),
            middle_name_nvc: Vec::new(),
            last_name_nvc: Vec::new(),
        }
    }

    pub fn load_first_name_nvc(&mut self, nvc: Vec<String>) {
        self.first_name_nvc = nvc.iter().map(|value|
            format!("<a href='/authors/{}/-/-/'>{}</a>",
                AuthorMask::encode(value.clone()),
                value)).collect();
    }

    pub fn load_middle_name_nvc(&mut self, nvc: Vec<String>) {
        self.middle_name_nvc = nvc.iter().map(|value|
            format!("<a href='/authors/-/{}/-/'>{}</a>",
                AuthorMask::encode(value.clone()),
                value)).collect();
    }

    pub fn load_last_name_nvc(&mut self, nvc: Vec<String>) {
        self.last_name_nvc = nvc.iter().map(|value|
            format!("<a href='/authors/-/-/{}/'>{}</a>",
                AuthorMask::encode(value.clone()),
                value)).collect();
    }
}

pub fn get_root_ctx(conn: &SqliteConnection) -> QueryResult<RootContext> {

    let mut ctx = RootContext::new();

    ctx.load_first_name_nvc(get_next_valid_chars(conn, "authors", "first_name", &String::new())?);
    ctx.load_middle_name_nvc(get_next_valid_chars(conn, "authors", "middle_name", &String::new())?);
    ctx.load_last_name_nvc(get_next_valid_chars(conn, "authors", "last_name", &String::new())?);

    return Ok(ctx);
}
