use super::collection::*;
use super::model::*;
use super::stream;
use crate::broadcast;
use crate::common_model::CommonResponse;
use crate::console;
use crate::database::Database;
use actix_web::Responder;
use actix_web::{web, HttpResponse};
use env_logger::{Builder, Env};
use log;
use serde::{Deserialize, Serialize};

pub async fn create(
    database_data: web::Data<Database>,
    content: web::Json<OrderCreateRequest>,
) -> impl Responder
{
    info!("Create Order requested...");

    let collection = database_data.orders().await;
    let collection_products = database_data.products().await;

    let insertion_result = collection.create(content.into_inner(), collection_products).await;

    match insertion_result
    {
        Ok(result) =>
        {
            let inserted_data = result.inserted_id;
            HttpResponse::Ok().json(inserted_data)
        },
        Err(error) => match error
        {
            OrderCollectionError::OneOfProductsNotFound =>
            {
                let response = CommonResponse::<Order> {
                    message: "One of products not found.".to_string(),
                    data: None,
                };

                HttpResponse::BadRequest().json(response)
            },
            OrderCollectionError::CustomError(message) =>
            {
                let response = CommonResponse::<Order> {
                    message,
                    data: None,
                };

                HttpResponse::BadRequest().json(response)
            },
            _ =>
            {
                let response = CommonResponse::<Order> {
                    message: "Unknown error.".to_string(),
                    data: None,
                };
                HttpResponse::BadRequest().json(response)
            },
        
        },
    }
}

pub async fn list(
    database_data: web::Data<Database>,
    query: web::Query<OrderListQuery>,
) -> impl Responder
{
    info!("[Service] List Order requested...");

    let offset = query.offset.unwrap_or(0);
    let limit = query.limit.unwrap_or(0);

    let collection = database_data.orders().await;

    let result = collection.list(offset, limit).await;

    match result
    {
        Ok(orders) => HttpResponse::Ok().json(orders),
        Err(error) =>
        {
            error!("[Service] Failed to get order. Error: {:?}", error);
            HttpResponse::InternalServerError().finish()
        },
    }
}


pub async fn update(
    database_data: web::Data<Database>,
    broadcaster: web::Data<std::sync::Mutex<broadcast::Broadcaster>>,
    id: web::Path<String>,
    content: web::Json<OrderUpdateRequest>,
) -> impl Responder
{
    info!("Update Product requested...");

    let internal_id = id.into_inner();

    let collection = database_data.orders().await;

    let update_result = collection
        .update(internal_id.clone(), content.into_inner())
        .await;

    broadcast::broadcast("counter".to_string(), internal_id.clone(), broadcaster);

    match update_result
    {
        Ok(result) =>
        {
            let updated_data = result.modified_count;
            let response = CommonResponse::<Order> {
                message: format!("{} order updated.", updated_data),
                data: None,
            };
            HttpResponse::Ok().json(response)
        },
        Err(error) =>
        {
            match error
            {
                OrderCollectionError::OrderNotModified =>
                {
                    let response = CommonResponse::<Order> {
                        message: "Order Not Modified".to_string(),
                        data: None,
                    };
                    HttpResponse::NotFound().json(response)
                },
                OrderCollectionError::CustomError(message) =>
                {
                    let response = CommonResponse::<Order> {
                        message,
                        data: None,
                    };
                    HttpResponse::BadRequest().json(response)
                },
                _ =>
                {
                    let response = CommonResponse::<Order> {
                        message: "Unknown error.".to_string(),
                        data: None,
                    };
                    HttpResponse::BadRequest().json(response)
                },
            }
        },
    }
}

pub async fn delete(database_data: web::Data<Database>, id: web::Path<String>) -> impl Responder
{
    let collection = database_data.orders().await;

    let result = collection.delete(id.into_inner()).await;

    match result
    {
        Ok(result) =>
        {
            if result.deleted_count == 0
            {
                let response = CommonResponse::<Order> {
                    message: "Order not found.".to_string(),
                    data: None,
                };
                HttpResponse::NotFound().json(response)
            }
            else
            {
                let response = CommonResponse::<Order> {
                    message: format!("{} orders deleted.", result.deleted_count),
                    data: None,
                };
                HttpResponse::Ok().json(response)
            }
        },
        Err(error) =>
        {
            let response = CommonResponse::<String> 
            {
                message: "Unknown error.".to_string(),
                data: None,
            };

            error!("Failed to delete order. Error: {:?}", error);

            HttpResponse::InternalServerError().json(response)
        },
    }
}

pub async fn get(database_data: web::Data<Database>, id: web::Path<String>) -> impl Responder
{
    info!("Get Product by ID requested...");

    let collection = database_data.orders().await;

    let result = collection.get(id.into_inner()).await;

    match result
    {
        Ok(product) => HttpResponse::Ok().json(product),
        Err(error) =>
        {
            error!("Failed to get product. Error: {:?}", error);
            HttpResponse::InternalServerError().finish()
        },
    }
}
