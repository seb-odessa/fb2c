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

#[get("/user/{user_id}")]
async fn get_user<'a>(ctx: WebCtx<'a>, id: web::Path<u32>) -> WebResult {
    let id = id.into_inner();
    let conn = ctx.pool.get().expect("couldn't get db connection from pool");
    let user = web::block(move || actions::find_user_by_uid(id, &conn))
        .await
        .map_err(|e| {
            eprintln!("{}", e);
            HttpResponse::InternalServerError().finish()
        })?;
    if let Some(user) = user {
        Ok(HttpResponse::Ok().json(user))
    } else {
        let res = HttpResponse::NotFound()
            .body(format!("No user found with uid: {}", id));
        Ok(res)
    }
}

#[get("/authors/first_name/{chars}/")]
async fn get_author_first_name_nvc<'a>(ctx: WebCtx<'a>, chars: web::Path<String>) -> WebResult {
    let chars = chars.into_inner();
    let conn = ctx.pool.get().expect("couldn't get db connection from pool");
    let data = web::block(move || actions::get_next_valid_chars(&conn, "authors", "first_name", chars))
        .await
        .map_err(|e| { eprintln!("{}", e); HttpResponse::InternalServerError().finish()})?;
    Ok(HttpResponse::Ok().json(data))
}

#[get("/authors/middle_name/{chars}/")]
async fn get_author_middle_name_nvc<'a>(ctx: WebCtx<'a>, chars: web::Path<String>) -> WebResult {
    let chars = chars.into_inner();
    let conn = ctx.pool.get().expect("couldn't get db connection from pool");
    let data = web::block(move || actions::get_next_valid_chars(&conn, "authors", "middle_name", chars))
        .await
        .map_err(|e| { eprintln!("{}", e); HttpResponse::InternalServerError().finish()})?;
    Ok(HttpResponse::Ok().json(data))
}

#[get("/authors/last_name/{chars}/")]
async fn get_author_last_name_nvc<'a>(ctx: WebCtx<'a>, chars: web::Path<String>) -> WebResult {
    let chars = chars.into_inner();
    let conn = ctx.pool.get().expect("couldn't get db connection from pool");
    let data = web::block(move || actions::get_next_valid_chars(&conn, "authors", "last_name", chars))
        .await
        .map_err(|e| { eprintln!("{}", e); HttpResponse::InternalServerError().finish()})?;
    Ok(HttpResponse::Ok().json(data))
}


#[get("/query/{fname}/{mname}/{lname}/{title}/")]
async fn query<'a>(ctx: WebCtx<'a>, args: web::Path<(String, String, String, String)>) -> WebResult      {
    let (first_name, middle_name, last_name, book_title) = args.into_inner();
    let _conn = ctx.pool.get().expect("couldn't get db connection from pool");
    let hb = &ctx.handlebars;
    // let data = web::block(move || actions::get_next_valid_chars(&conn, "authors", "last_name", first_name))
    //     .await
    //     .map_err(|e| { eprintln!("{}", e); HttpResponse::InternalServerError().finish()})?;

    let search = json!({
        "first_name": first_name,
        "middle_name": middle_name,
        "last_name": last_name,
        "book_title": book_title
    });
    let body = hb.render("query", &search).unwrap();


    Ok(HttpResponse::Ok()
        .header("X-TEST", "value")
        .body(body)
    )
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();
    dotenv::dotenv().ok();

    let templates = "./templates";
    let mut handlebars = Handlebars::new();
    handlebars
        .register_templates_directory(".hbs", templates)
        .expect(&format!("Can't register template directory {}", templates));

    let ctx = web::Data::new(Context::new(actions::get_connection_pool(), handlebars));

    let bind = "127.0.0.1:8080";
    println!("Starting server at: {}", &bind);
    HttpServer::new(move || {
        App::new()
            .app_data(ctx.clone())
            .wrap(middleware::Logger::default())
            .service(get_user)
            .service(get_author_first_name_nvc)
            .service(get_author_middle_name_nvc)
            .service(get_author_last_name_nvc)
            .service(query)
    })
    .bind(&bind)?
    .run()
    .await
}