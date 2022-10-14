#[macro_use]
extern crate actix_web;
#[macro_use]
extern crate diesel;
extern crate termion;

use actix_web::{guard, middleware, web, App, HttpResponse, HttpServer, Responder, Result, Error};
use console::info;
use diesel::r2d2::ConnectionManager;
use diesel::PgConnection;
use r2d2::{Pool, PooledConnection};
use serde::{Deserialize, Serialize};
use std::{env, io};
use dotenv::dotenv;


mod console;
mod constants;
mod response;
mod schema;
mod products;

// database connection pool
pub type DbPool = Pool<ConnectionManager<PgConnection>>;

// database connection
pub type DbPooledConnection = PooledConnection<ConnectionManager<PgConnection>>;

// #[derive(Debug, Deserialize, Serialize)]
// struct Response {
//     id: i32,
//     name: String,
//     email: String,
//     password: String,
// }

// async fn index(pool: web::Data<DbPool>, name: web::Path<String>) -> impl Responder {
//     let name = name.into_inner();

//     let conn = pool.get().expect("couldn't get db connection from pool");

//     let response = Response {
//         id: 1,
//         name: "test".to_string(),
//         email: "test@test.com".to_string(),
//         password: "test".to_string()
//     };

//     HttpResponse::Ok().json(response)
// }

#[actix_rt::main]
async fn main() -> io::Result<()> {
    // load .env file
    dotenv().ok();

    // get required environment variables
    env::set_var("RUST_LOG", "actix_web=debug,actix_server=info");

    console::info("Initializing environment variables...").await;
    // start logger
    env_logger::init();

    console::info("Getting database url...").await;
    // set up database connection pool
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    console::info("Creating database connection...").await;
    // create connection to postgres database
    let manager = ConnectionManager::<PgConnection>::new(database_url);

    console::info("Creating database connection pool...").await;
    // create connection pool
    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool");

    // info message for listing server address and port
    console::info(&format!("Listening on {}...", constants::SERVER_PORT)).await;

    // start http server
    HttpServer::new(move || {
        App::new()
            .wrap(middleware::Logger::default())
            .app_data(web::Data::new(pool.clone()))
            .service(web::resource("/products").to(products::products_list))
            .service(web::resource("/products/create/{name}").to(products::products_create))
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
