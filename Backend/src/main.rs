#[macro_use]
extern crate actix_web;
#[macro_use]
extern crate diesel;
extern crate termion;

use actix_web::{guard, middleware, web, App, HttpResponse, HttpServer, Responder, Result, Error};
use diesel::r2d2::ConnectionManager;
use diesel::PgConnection;
use r2d2::{Pool, PooledConnection};
use serde::{Deserialize, Serialize};
use std::{env, io};

mod console;
mod constants;
mod response;
mod schema;

pub type DbPool = Pool<ConnectionManager<PgConnection>>;
pub type DbPooledConnection = PooledConnection<ConnectionManager<PgConnection>>;

#[derive(Debug, Deserialize, Serialize)]
struct Response {
    id: i32,
    name: String,
    email: String,
    password: String,
}

async fn index(pool: web::Data<DbPool>, name: web::Path<String>) -> impl Responder {
    let name = name.into_inner();

    let conn = pool.get().expect("couldn't get db connection from pool");

    let response = Response {
        id: 1,
        name: "test".to_string(),
        email: "test@test.com".to_string(),
        password: "test".to_string()
    };

    HttpResponse::Ok().json(response)
}

#[actix_rt::main]
async fn main() -> io::Result<()> {
    env::set_var("RUST_LOG", "actix_web=debug,actix_server=info");
    env_logger::init();

    // set up database connection pool
    let database_url: String = String::from(constants::DATABASE_URL);

    let manager = ConnectionManager::<PgConnection>::new(database_url);

    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool");

    // info message for listing server address and port
    console::info(&format!("Listening on {}", constants::SERVER_PORT)).await;

    HttpServer::new(move || {
        App::new()
            .wrap(middleware::Logger::default())
            .app_data(web::Data::new(pool.clone()))
            .service(web::resource("/prefix").to(index))
            .service(
                web::resource("/user/{name}")
                    .name("user_detail")
                    .guard(guard::Header("content-type", "application/json"))
                    .route(web::get().to(HttpResponse::Ok))
                    .route(web::put().to(HttpResponse::Ok)),
            )
    })
    .bind(("0.0.0.0", constants::SERVER_PORT))?
    .run()
    .await
}
