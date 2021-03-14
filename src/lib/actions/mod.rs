use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};
use diesel::sql_query;
use diesel::sql_types::Text;

pub type QueryResult<T> = std::result::Result<T, diesel::result::Error>;
pub type ConnectionPool = r2d2::Pool<ConnectionManager<SqliteConnection>>;

pub mod author_mask;
pub use author_mask::{AuthorMask, NvcMethods};
pub mod author_context;
pub use author_context::{FindAuthorContext, AuthorContext};
pub mod root_context;
pub use root_context::RootContext;
pub mod title_context;
pub use title_context::{TitleMask, TitleContext, FindTitleContext};
pub mod book_record;
pub use book_record::{BookRecord, BookStringified};
pub mod download_context;
pub use download_context::DownloadContext;



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

pub fn get_next_valid(conn: &SqliteConnection, table: &str, column: &str, mask: &dyn NvcMethods) -> QueryResult<Vec<String>>
{
    #[derive(QueryableByName, Debug, Clone)]
    pub struct DbString {
        #[sql_type = "Text"] pub content: String,
    }
    let query = format!(r#"
        SELECT DISTINCT substr({column}, 1, {len}) AS content
        FROM {table} {where_clause}
        ORDER BY content"#,
            table = table,
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

pub fn get_authors(conn: &SqliteConnection, mask: &dyn NvcMethods) -> QueryResult<Vec<AuthorMask>>
{
    let query = format!(r#"
            SELECT DISTINCT first_name, middle_name, last_name
            FROM authors {where_clause}
            ORDER BY last_name, first_name, middle_name"#,
        where_clause = mask.get_where_like_clause()
    );

    sql_query(&query).load(conn)
}

pub fn get_titles_by_authors(conn: &SqliteConnection, mask: &dyn NvcMethods) -> QueryResult<Vec<String>>
{
    #[derive(QueryableByName, Debug, Clone)]
    pub struct DbString {
        #[sql_type = "Text"] pub content: String,
    }
    let query = format!(r#"
        SELECT DISTINCT book_title as content
        FROM author_links
        JOIN title_links ON (author_links.book_id = title_links.book_id)
        LEFT JOIN authors ON (author_links.author_id = authors.id)
        LEFT JOIN titles ON (title_links.title_id = titles.id)
        {where_clause}
        ORDER BY content"#,
        where_clause = mask.get_where_explicit_clause()
    );

    sql_query(&query)
        .load::<DbString>(conn)
        .map(|list|
            list.iter().map(|s|
                s.content.clone()).collect()
            )
}

pub fn get_titles_with_author(conn: &SqliteConnection, mask: &dyn NvcMethods) -> QueryResult<Vec<TitleMask>>
{
    #[derive(QueryableByName, Debug, Clone)]
    pub struct DbString {
        #[sql_type = "Text"] pub content: String,
    }
    let query = format!(r#"
        SELECT DISTINCT book_title, last_name, first_name, middle_name
        FROM author_links
        JOIN title_links ON (author_links.book_id = title_links.book_id)
        LEFT JOIN authors ON (author_links.author_id = authors.id)
        LEFT JOIN titles ON (title_links.title_id = titles.id)
        {where_clause}
        ORDER BY book_title, last_name, first_name, middle_name"#,
        where_clause = mask.get_where_like_clause()
    );

    sql_query(&query).load::<TitleMask>(conn)
}

pub fn urify_authors(url: &str, authors: Vec<AuthorMask>) -> Vec<String> {
    authors.iter().map(|author|
        format!("<a href='/{}/{}/{}/{}/'>{}</a>",
            url,
            AuthorMask::encode(author.first_name.clone()),
            AuthorMask::encode(author.middle_name.clone()),
            AuthorMask::encode(author.last_name.clone()),
            author.get_full_name())
        ).collect()
}

pub fn get_authors_page(conn: &SqliteConnection, url: &str, mask: &AuthorMask) -> QueryResult<FindAuthorContext> {

    let mut ctx = FindAuthorContext::new(url, mask);
    if !mask.is_empty()
    {
        ctx.authors = urify_authors("author", get_authors(conn, mask)?);
    }

    ctx.load_first_name_nvc(get_next_valid(conn, "authors", "first_name", mask)?);
    ctx.load_middle_name_nvc(get_next_valid(conn, "authors", "middle_name", mask)?);
    ctx.load_last_name_nvc(get_next_valid(conn, "authors", "last_name", mask)?);

    return Ok(ctx);
}

pub fn urify_titles_by_authors(url: &str, masks: &Vec<TitleMask>) -> Vec<String> {
    masks.iter().map(|mask|
        format!("<a href='/{}/{}/{}/{}/'>{} ({} {} {})</a>",
            url,
            AuthorMask::encode(mask.first_name.clone()),
            AuthorMask::encode(mask.middle_name.clone()),
            AuthorMask::encode(mask.last_name.clone()),
            mask.book_title,
            mask.last_name, mask.first_name, mask.middle_name
        )
    ).collect()
}

pub fn load_titles_page(conn: &SqliteConnection, url: &str, mask: &TitleMask) -> QueryResult<FindTitleContext> {

    let mut ctx = FindTitleContext::new(url, &mask);
    ctx.title_and_author = get_titles_with_author(conn, mask)?;
    if !mask.is_empty()
    {
        ctx.titles = urify_titles_by_authors("author", &ctx.title_and_author);
    }
    ctx.load_title_nvc(get_next_valid(conn, "titles", "book_title", mask)?);
    return Ok(ctx);
}

pub fn urify_titles(url: &str, author: &AuthorMask, titles: Vec<String>) -> Vec<String> {
    titles.iter().map(|title|
        format!("<a href='/{url}/{author}/{title}/'>{title}</a>",
            url = url,
            author = author.get_uri(),
            title = title)
        ).collect()
}

pub fn get_author_ctx(conn: &SqliteConnection, url: &str, author: &AuthorMask) -> QueryResult<AuthorContext> {

    let mut ctx = AuthorContext::new(url, author);
    ctx.titles = urify_titles("title", author, get_titles_by_authors(conn, author)?);

    return Ok(ctx);
}

pub fn get_root_page(conn: &SqliteConnection) -> QueryResult<RootContext> {

    let empty = String::new();
    let mut ctx = RootContext::new();
    ctx.first_name_nvc = get_next_valid_chars(conn, "authors", "first_name", &empty)?
        .iter()
        .map(|value| format!("<a href='/authors/-/-/{}/'>{}</a>", AuthorMask::encode(value.clone()),value)).collect();

    ctx.middle_name_nvc = get_next_valid_chars(conn, "authors", "middle_name", &empty)?
        .iter()
        .map(|value| format!("<a href='/authors/-/{}/-/'>{}</a>", AuthorMask::encode(value.clone()),value)).collect();

    ctx.last_name_nvc = get_next_valid_chars(conn, "authors", "last_name", &empty)?
        .iter()
        .map(|value| format!("<a href='/authors/-/-/{}/'>{}</a>", AuthorMask::encode(value.clone()),value)).collect();

    ctx.book_title_nvc = get_next_valid_chars(conn, "titles", "book_title", &empty)?
        .iter()
        .map(|value| format!("<a href='/titles/{}/'>{}</a>", AuthorMask::encode(value.clone()),value)).collect();

    return Ok(ctx);
}

pub fn load_author_title_ctx(conn: &SqliteConnection, author: &AuthorMask, title: &String)-> QueryResult<TitleContext> {

    let mut ctx = TitleContext::new(author, title.clone());
    ctx.books = BookStringified::transform(BookRecord::load_by_author_and_title(conn, author, &title)?);
    return Ok(ctx);
}

pub fn load_download_ctx(conn: &SqliteConnection, workdir: String, archive: &String, book: &String)-> QueryResult<DownloadContext> {

    let record = BookRecord::load_by_archive_and_book(conn, archive, book)?;
    return Ok(DownloadContext::new(&workdir, record));
}