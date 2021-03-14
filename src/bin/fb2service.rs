extern crate env_logger;
#[macro_use]
extern crate serde_json;
use actix_files::NamedFile;

use std::env;
use lib::actions;
use actix_web::{get, middleware, web, App, Error, HttpResponse, HttpServer};
use handlebars::Handlebars;

struct Context<'a> {
    pub pool: actions::ConnectionPool,
    pub handlebars: Handlebars<'a>,
}
impl<'a> Context<'a> {
    pub fn new(pool: actions::ConnectionPool, handlebars: Handlebars<'a>) -> Self {
        Self {
            pool: pool,
            handlebars: handlebars
        }
    }
}

type WebCtx<'a> = web::Data<Context<'a>>;
type WebResult = Result<HttpResponse, Error>;
type FileResult = Result<NamedFile, Error>;

#[get("/")]
async fn root<'a>(ctx: WebCtx<'a>) -> WebResult {
    let conn = ctx.pool.get().expect("couldn't get db connection from pool");
    let page = web::block(move|| actions::get_root_page(&conn))
        .await
        .map_err(|e| {
            eprintln!("{}", e);
            HttpResponse::InternalServerError().finish()})?;

    let body = ctx.handlebars.render("root", &json!(&page))
                             .expect("couldn't render template");

    Ok(HttpResponse::Ok().body(body))
}


#[get("/authors/{fname}/{mname}/{lname}/")]
async fn authors<'a>(ctx: WebCtx<'a>, args: web::Path<(String, String, String)>) -> WebResult {
    let (first_name, middle_name, last_name) = args.into_inner();
    let pattern = actions::AuthorMask::new(first_name, middle_name, last_name);
    let conn = ctx.pool.get().expect("couldn't get db connection from pool");
    let page = web::block(move|| actions::get_authors_page(&conn, "authors", &pattern))
        .await
        .map_err(|e| {
            eprintln!("{}", e);
            HttpResponse::InternalServerError().finish()})?;

    let body = ctx.handlebars.render("authors", &json!(&page))
                             .expect("couldn't render template");

    Ok(HttpResponse::Ok().body(body))
}

#[get("/author/{fname}/{mname}/{lname}/")]
async fn author<'a>(ctx: WebCtx<'a>, args: web::Path<(String, String, String)>) -> WebResult {
    let (first_name, middle_name, last_name) = args.into_inner();
    let author = actions::AuthorMask::new(first_name, middle_name, last_name);
    let conn = ctx.pool.get().expect("couldn't get db connection from pool");
    let page = web::block(move|| actions::get_author_ctx(&conn, "author", &author))
        .await
        .map_err(|e| {
            eprintln!("{}", e);
            HttpResponse::InternalServerError().finish()})?;

    let body = ctx.handlebars.render("author", &json!(&page))
                             .expect("couldn't render template");

    Ok(HttpResponse::Ok().body(body))
}

#[get("/title/{fname}/{mname}/{lname}/{title}/")]
async fn title<'a>(ctx: WebCtx<'a>, args: web::Path<(String, String, String, String)>) -> WebResult {

    let (first_name, middle_name, last_name, title) = args.into_inner();
    let au = actions::AuthorMask::new(first_name, middle_name, last_name);
    let conn = ctx.pool.get().expect("couldn't get db connection from pool");
    let page = web::block(move|| actions::load_author_title_ctx(&conn, &au, &title))
        .await
        .map_err(|e| {
            eprintln!("{}", e);
            HttpResponse::InternalServerError().finish()})?;


    let body = ctx.handlebars.render("title", &json!(&page))
                             .expect("couldn't render template");

    Ok(HttpResponse::Ok().body(body))
}

#[get("/titles/{title}/")]
async fn titles<'a>(ctx: WebCtx<'a>, args: web::Path<String>) -> WebResult {
    let book_title = args.into_inner();
    let pattern = actions::TitleMask::new(book_title);
    let conn = ctx.pool.get().expect("couldn't get db connection from pool");
    let page = web::block(move|| actions::load_titles_page(&conn, "titles", &pattern))
        .await
        .map_err(|e| {
            eprintln!("{}", e);
            HttpResponse::InternalServerError().finish()})?;

    let body = ctx.handlebars.render("titles", &json!(&page))
                             .expect("couldn't render template");

    Ok(HttpResponse::Ok().body(body))
}

#[get("/download/{archive}/{book}")]
async fn download<'a>(ctx: WebCtx<'a>, args: web::Path<(String, String)>) -> FileResult {
    let (archive, book) = args.into_inner();
    let conn = ctx.pool.get().expect("couldn't get db connection from pool");
    let mut page = web::block(move|| actions::load_download_ctx(&conn, String::from("/tmp"), &archive, &book))
        .await
        .map_err(|e| {
            eprintln!("{}", e);
            HttpResponse::InternalServerError().finish()})?;
    Ok(page.get_unzipped_stream()?)
}

#[get("/download_zip/{archive}/{book}")]
async fn download_zip<'a>(ctx: WebCtx<'a>, args: web::Path<(String, String)>) -> FileResult {
    let (archive, book) = args.into_inner();
    let conn = ctx.pool.get().expect("couldn't get db connection from pool");
    let mut page = web::block(move|| actions::load_download_ctx(&conn, String::from("/tmp"), &archive, &book))
        .await
        .map_err(|e| {
            eprintln!("{}", e);
            HttpResponse::InternalServerError().finish()})?;
    Ok(page.get_zipped_stream()?)
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();
    dotenv::dotenv().ok();

    let templates = "./templates";
    let mut handlebars = Handlebars::new();
    handlebars.register_templates_directory(".hbs", templates)
        .expect(&format!("Can't register template directory {}", templates));

    let ctx = web::Data::new(Context::new(actions::get_connection_pool(), handlebars));

    let bind = env::var("INTERFACE").expect("INTERFACE must be set, e.g.: 192.168.0.1:8080");
    //let bind = "home:8080";
    println!("Starting server at: {}", &bind);
    HttpServer::new(move || {
        App::new()
            .app_data(ctx.clone())
            .wrap(middleware::Logger::default())
            .service(root)
            .service(authors)
            .service(author)
            .service(titles)
            .service(title)
            .service(download)
            .service(download_zip)
        })
    .bind(&bind)?
    .run()
    .await
}