extern crate env_logger;

use lib::actions;
use actix_web::{get, middleware, web, App, Error, HttpResponse, HttpServer};

type ConnPool = web::Data<actions::ConnectionPool>;
type HttpResult = Result<HttpResponse, Error>;

#[get("/user/{user_id}")]
async fn get_user(pool: ConnPool, id: web::Path<u32>) -> HttpResult {
    let id = id.into_inner();
    let conn = pool.get().expect("couldn't get db connection from pool");
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
async fn get_author_first_name_nvc(pool: ConnPool, chars: web::Path<String>) -> HttpResult {
    let chars = chars.into_inner();
    let conn = pool.get().expect("couldn't get db connection from pool");
    let data = web::block(move || actions::get_next_valid_chars(&conn, "authors", "first_name", chars))
        .await
        .map_err(|e| { eprintln!("{}", e); HttpResponse::InternalServerError().finish()})?;
    Ok(HttpResponse::Ok().json(data))
}

#[get("/authors/middle_name/{chars}/")]
async fn get_author_middle_name_nvc(pool: ConnPool, chars: web::Path<String>) -> HttpResult {
    let chars = chars.into_inner();
    let conn = pool.get().expect("couldn't get db connection from pool");
    let data = web::block(move || actions::get_next_valid_chars(&conn, "authors", "middle_name", chars))
        .await
        .map_err(|e| { eprintln!("{}", e); HttpResponse::InternalServerError().finish()})?;
    Ok(HttpResponse::Ok().json(data))
}

#[get("/authors/last_name/{chars}/")]
async fn get_author_last_name_nvc(pool: ConnPool, chars: web::Path<String>) -> HttpResult {
    let chars = chars.into_inner();
    let conn = pool.get().expect("couldn't get db connection from pool");
    let data = web::block(move || actions::get_next_valid_chars(&conn, "authors", "last_name", chars))
        .await
        .map_err(|e| { eprintln!("{}", e); HttpResponse::InternalServerError().finish()})?;
    Ok(HttpResponse::Ok().json(data))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();
    dotenv::dotenv().ok();

    let pool = actions::get_connection_pool();
    let bind = "127.0.0.1:8080";
    println!("Starting server at: {}", &bind);
    HttpServer::new(move || {
        App::new()
            .data(pool.clone())
            .wrap(middleware::Logger::default())
            .service(get_user)
            .service(get_author_first_name_nvc)
            .service(get_author_middle_name_nvc)
            .service(get_author_last_name_nvc)
    })
    .bind(&bind)?
    .run()
    .await
}