#![allow(non_snake_case)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(dead_code)]

use actix_web::{guard, middleware, web, App, HttpResponse, HttpServer};
use dotenv::dotenv;
use env_logger::{Builder, Env};
use std::{env, io};

#[macro_use]
extern crate log;

mod common_model;
mod console;
mod constants;
mod database;
mod products;

#[actix_rt::main]
async fn main() -> io::Result<()>
{
    // load .env file
    dotenv().ok();

    // get required environment variables
    env::set_var("RUST_LOG", "actix_web=debug,actix_server=info,products=error");

    info!("Initializing environment variables...");
    // start logger
    env_logger::init();

    let database_manager = database::Database::init().await;

    // info message for listing server address and port
    info!("Listening on {}...", constants::SERVER_PORT);

    // start http server
    HttpServer::new(move || {
        App::new()
            .wrap(middleware::Logger::default())
            .app_data(web::Data::new(database_manager.clone()))
            .service(web::scope("/v1").configure(products::config))
    })
    .bind(("0.0.0.0", constants::SERVER_PORT))?
    .run()
    .await
}
