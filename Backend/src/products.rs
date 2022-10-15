use crate::console;
use crate::database::Database;
use crate::models::*;
use actix_web::Responder;
use actix_web::{web, HttpResponse};
use serde::{Deserialize, Serialize};

pub async fn create_product(database_data: web::Data<Database>,
                             request: web::Json<ProductCreateRequest>)
                             -> impl Responder
{
    console::info("Create Product requested...").await;

    let content = request.into_inner();

    let insertion_result = database_data.products_create(content).await;

    let inserted_data = insertion_result.unwrap().inserted_id;

    HttpResponse::Ok().json(inserted_data)
}

pub async fn get_products(database_data: web::Data<Database>, query: web::Query<ProductListQuery>) -> impl Responder
{
    console::info("List Products requested...").await;

    let start = query.start.unwrap_or(0);
    let count = query.count.unwrap_or(0);

    let products = database_data.products_list(start, count)
                                .await
                                .expect("Failed to get products.");

    HttpResponse::Ok().json(products)
}

pub async fn update_product(database_data: web::Data<Database>,
                             request: web::Json<ProductUpdateRequest>)
                             -> impl Responder
{
    console::info("Update Product requested...").await;

    let filter = request.filter.clone();
    let content = request.content.clone();

    let update_result = database_data.products_update(filter, content).await;

    let updated_data = update_result.unwrap().modified_count;

    HttpResponse::Ok().json(updated_data)
}

pub async fn delete_product(database_data: web::Data<Database>,
                             request: web::Json<ProductDeleteRequest>)
                             -> impl Responder
{
    console::info("Delete Product requested...").await;

    let filter = request.filter.clone();

    let delete_result = database_data.products_delete(filter).await;

    let deleted_data = delete_result.unwrap().deleted_count;

    HttpResponse::Ok().json(deleted_data)
}


pub async fn get_product_by_id(database_data: web::Data<Database>,
                               id: web::Path<String>)
                               -> impl Responder
{
    console::info("Get Product by ID requested...").await;

    let product = database_data.products_get_by_id(id.into_inner())
                               .await
                               .expect("Failed to get product.");

    HttpResponse::Ok().json(product)
}

pub async fn delete_product_by_id(database_data: web::Data<Database>,
                                  id: web::Path<String>)
                                  -> impl Responder
{
    console::info("Delete Product by ID requested...").await;

    let delete_result = database_data.products_delete_by_id(id.into_inner()).await;

    let deleted_data = delete_result.unwrap().deleted_count;

    HttpResponse::Ok().json(deleted_data)
}

pub async fn update_product_by_id(database_data: web::Data<Database>,
                                  id: web::Path<String>,
                                  request: web::Json<ProductUpdateRequestContent>)
                                  -> impl Responder
{
    console::info("Update Product by ID requested...").await;

    let content = request.into_inner();

    let update_result = database_data.products_update_by_id(id.into_inner(), content).await;

    let updated_data = update_result.unwrap().modified_count;

    HttpResponse::Ok().json(updated_data)
}