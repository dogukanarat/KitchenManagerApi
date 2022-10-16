#![allow(non_snake_case)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(dead_code)]

use actix_web::{guard, middleware, web, App, HttpResponse, HttpServer};
use std::{env, io};
use dotenv::dotenv;

mod products;
mod console;
mod constants;
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
            .service(
                web::scope("/v1")
                    .service(
                        web::resource("/products")
                            .route(web::get().to(products::service::list))
                            .route(web::post().to(products::service::create))
                            .route(web::delete().to(products::service::delete))
                            .route(web::put().to(products::service::update)),
                    )
                    .service(
                        web::resource("/products/{id}")
                            .route(web::get().to(products::service::get_by_id))
                            .route(web::delete().to(products::service::delete_by_id))
                            .route(web::put().to(products::service::update_by_id)),
                    ),
            )
    })
    .bind(("0.0.0.0", constants::SERVER_PORT))?
    .run()
    .await
}
