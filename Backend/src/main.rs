// disable rust compiler warnings
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(dead_code)]

use actix_web::{guard, middleware, web, App, HttpResponse, HttpServer};
use std::{env, io};
use dotenv::dotenv;

mod console;
mod constants;
mod products;
mod models;
mod database;

#[actix_rt::main]
async fn main() -> io::Result<()> {
    // load .env file
    dotenv().ok();

    // get required environment variables
    env::set_var("RUST_LOG", "actix_web=debug,actix_server=info");

    console::info("Initializing environment variables...").await;
    // start logger
    env_logger::init();

    let database_manager = database::Database::init().await;

    // info message for listing server address and port
    console::info(&format!("Listening on {}...", constants::SERVER_PORT)).await;

    // start http server
    HttpServer::new(move || {
        App::new()
            .wrap(middleware::Logger::default())
            .app_data(web::Data::new(database_manager.clone()))
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
