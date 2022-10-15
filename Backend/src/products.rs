use actix_web::Responder;
use actix_web::{web, HttpResponse};
use crate::console;
use crate::database::Database;
use crate::models::Product;

pub async fn products_create(database_data: web::Data<Database>, name: web::Path<String>) -> impl Responder 
{
    console::info("Create Product requested...").await;

    let new_product = Product {
        id: None,
        name: name.to_string(),
        price: 0.0,
    };

    let insertion_result = database_data.products_create(new_product).await;

    let inserted_data =  insertion_result.unwrap().inserted_id;

    HttpResponse::Ok().json(inserted_data)
}

pub async fn products_list(database_data: web::Data<Database>) -> impl Responder 
{
    console::info("List Products requested...").await;

    let products = database_data.products_list().await.expect("Failed to get products.");

    HttpResponse::Ok().json(products)
}
