pub mod collection;
pub mod model;
pub mod service;

use actix_web::{web, App, HttpServer, Scope};

pub fn config(config: &mut web::ServiceConfig)
{
    config.service(
        web::scope("/products")
        .service(
            web::resource("")
            .route(web::post().to(service::create))
            .route(web::get().to(service::list))
        )
        .service(
            web::resource("/{id}")
            .route(web::get().to(service::get))
            .route(web::put().to(service::update))
            .route(web::delete().to(service::delete))
        )
    );
}
