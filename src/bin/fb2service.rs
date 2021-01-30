extern crate env_logger;
#[macro_use]
extern crate serde_json;


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


#[get("/")]
async fn root<'a>(ctx: WebCtx<'a>) -> WebResult {
    let conn = ctx.pool.get().expect("couldn't get db connection from pool");
    let page = web::block(move|| actions::get_root_ctx(&conn))
        .await
        .map_err(|e| {
            eprintln!("{}", e);
            HttpResponse::InternalServerError().finish()})?;

    let hb = &ctx.handlebars;
    let body = hb.render("root", &json!(&page))
                 .expect("couldn't render template");

    Ok(HttpResponse::Ok().body(body))
}


#[get("/authors/{fname}/{mname}/{lname}/")]
async fn authors<'a>(ctx: WebCtx<'a>, args: web::Path<(String, String, String)>) -> WebResult {
    let (first_name, middle_name, last_name) = args.into_inner();
    let pattern = actions::AuthorMask::new(first_name, middle_name, last_name);
    let conn = ctx.pool.get().expect("couldn't get db connection from pool");
    let page = web::block(move|| actions::get_author_ctx(&conn, &pattern))
        .await
        .map_err(|e| {
            eprintln!("{}", e);
            HttpResponse::InternalServerError().finish()})?;

    let hb = &ctx.handlebars;
    let body = hb.render("authors", &json!(&page))
                 .expect("couldn't render template");

    Ok(HttpResponse::Ok().body(body))
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

    let bind = "127.0.0.1:8080";
    println!("Starting server at: {}", &bind);
    HttpServer::new(move || {
        App::new()
            .app_data(ctx.clone())
            .wrap(middleware::Logger::default())
            .service(root)
            .service(authors)
    })
    .bind(&bind)?
    .run()
    .await
}