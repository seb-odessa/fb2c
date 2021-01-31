use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};
use diesel::sql_query;
use diesel::sql_types::Text;

pub type QueryResult<T> = std::result::Result<T, diesel::result::Error>;
pub type ConnectionPool = r2d2::Pool<ConnectionManager<SqliteConnection>>;

pub mod author_mask;
pub use author_mask::AuthorMask;
pub mod author_context;
pub use author_context::{FindAuthorContext, AuthorContext};
pub mod root_context;
pub use root_context::RootContext;

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
            where_clause = mask.get_where_like_clause(),
            len = mask.get_length_by_name(column) + 1
    );

    sql_query(&query)
        .load::<DbString>(conn)
        .map(|list|
            list.iter().map(|s|
                s.content.clone()).collect()
            )
}

pub fn get_authors(conn: &SqliteConnection, mask: &AuthorMask) -> QueryResult<Vec<AuthorMask>>
{
    let query = format!(
        r#"
            SELECT DISTINCT first_name, middle_name, last_name
            FROM authors {where_clause}
            ORDER BY last_name, first_name, middle_name
            LIMIT 100
        "#,
        where_clause = mask.get_where_like_clause()
    );

    sql_query(&query).load(conn)
}

pub fn get_titles(conn: &SqliteConnection, author: &AuthorMask) -> QueryResult<Vec<String>>
{
    #[derive(QueryableByName, Debug, Clone)]
    pub struct DbString {
        #[sql_type = "Text"] pub content: String,
    }
    let query = format!(r#"
        SELECT book_title as content
        FROM author_links
        JOIN title_links ON (author_links.book_id = title_links.book_id)
        LEFT JOIN authors ON (author_links.author_id = authors.id)
        LEFT JOIN titles ON (title_links.title_id = titles.id)
        {where_clause}
        ORDER BY content"#,
        where_clause = author.get_where_explicit_clause()
    );

    sql_query(&query)
        .load::<DbString>(conn)
        .map(|list|
            list.iter().map(|s|
                s.content.clone()).collect()
            )
}

pub fn urify_authors(url: &str, authors: Vec<AuthorMask>) -> Vec<String> {
    authors.iter().map(|author|
        {
            let s = format!("<a href='/{}/{}/{}/{}/'>{}</a>",
            url,
            AuthorMask::encode(author.first_name.clone()),
            AuthorMask::encode(author.middle_name.clone()),
            AuthorMask::encode(author.last_name.clone()),
            author.get_full_name());
        println!("{}", s);
        return s;
        }
        ).collect()
}

pub fn get_find_authors_ctx(conn: &SqliteConnection, url: &str, mask: &AuthorMask) -> QueryResult<FindAuthorContext> {

    let mut ctx = FindAuthorContext::new(url, mask);
    ctx.authors = urify_authors("author", get_authors(conn, &mask)?);
    ctx.load_first_name_nvc(get_next_valid_authors(conn, "first_name", &mask)?);
    ctx.load_middle_name_nvc(get_next_valid_authors(conn, "middle_name", &mask)?);
    ctx.load_last_name_nvc(get_next_valid_authors(conn, "last_name", &mask)?);

    return Ok(ctx);
}


pub fn get_author_ctx(conn: &SqliteConnection, url: &str, author: &AuthorMask) -> QueryResult<AuthorContext> {

    let mut ctx = AuthorContext::new(url, author);
    ctx.titles = get_titles(conn, author)?;

    return Ok(ctx);
}


pub fn get_root_ctx(conn: &SqliteConnection) -> QueryResult<RootContext> {

    let mut ctx = RootContext::new();

    ctx.load_first_name_nvc(get_next_valid_chars(conn, "authors", "first_name", &String::new())?);
    ctx.load_middle_name_nvc(get_next_valid_chars(conn, "authors", "middle_name", &String::new())?);
    ctx.load_last_name_nvc(get_next_valid_chars(conn, "authors", "last_name", &String::new())?);

    return Ok(ctx);
}
