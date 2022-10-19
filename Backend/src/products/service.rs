use super::collection::*;
use super::model::*;
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
    content: web::Json<ProductCreateRequest>,
) -> impl Responder
{
    info!("Create Product requested...");

    let collection = database_data.products().await;

    let insertion_result = collection.create(content.into_inner()).await;

    match insertion_result
    {
        Ok(result) =>
        {
            let inserted_data = result.inserted_id;
            HttpResponse::Ok().json(inserted_data)
        },
        Err(error) => match error
        {
            ProductCollectionError::ProductNameExists =>
            {
                let response = CommonResponse::<Product> {
                    message: "Product name already exist.".to_string(),
                    data: None,
                };
                HttpResponse::BadRequest().json(response)
            },
            ProductCollectionError::CustomError(message) =>
            {
                let response = CommonResponse::<Product> {
                    message,
                    data: None,
                };
                HttpResponse::BadRequest().json(response)
            },
            _ =>
            {
                let response = CommonResponse::<Product> {
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
    query: web::Query<ProductListQuery>,
) -> impl Responder
{
    info!("List Products requested...");

    let offset = query.offset.unwrap_or(0);
    let limit = query.limit.unwrap_or(0);

    let collection = database_data.products().await;

    let result = collection.list(offset, limit).await;

    match result
    {
        Ok(products) => HttpResponse::Ok().json(products),
        Err(error) =>
        {
            error!("Failed to get products. Error: {:?}", error);
            HttpResponse::InternalServerError().finish()
        },
    }
}

pub async fn update(
    database_data: web::Data<Database>,
    id: web::Path<String>,
    content: web::Json<ProductUpdateRequest>,
) -> impl Responder
{
    info!("Update Product requested...");

    let collection = database_data.products().await;

    let update_result = collection
        .update(id.into_inner(), content.into_inner())
        .await;

    match update_result
    {
        Ok(result) =>
        {
            let updated_data = result.modified_count;
            let response = CommonResponse::<Product> {
                message: format!("{} products updated.", updated_data),
                data: None,
            };
            HttpResponse::Ok().json(response)
        },
        Err(error) =>
        {
            match error
            {
                ProductCollectionError::ProductNotFound =>
                {
                    let response = CommonResponse::<Product> {
                        message: "Product not found.".to_string(),
                        data: None,
                    };
                    HttpResponse::NotFound().json(response)
                },
                ProductCollectionError::ProductNotMofified =>
                {
                    let response = CommonResponse::<Product> {
                        message: "Product not modified.".to_string(),
                        data: None,
                    };
                    HttpResponse::BadRequest().json(response)
                },
                ProductCollectionError::CustomError(message) =>
                {
                    let response = CommonResponse::<Product> {
                        message,
                        data: None,
                    };
                    HttpResponse::BadRequest().json(response)
                },
                _ =>
                {
                    let response = CommonResponse::<Product> {
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
    let collection = database_data.products().await;

    let result = collection.delete(id.into_inner()).await;

    match result
    {
        Ok(result) =>
        {
            if result.deleted_count == 0
            {
                let response = CommonResponse::<Product> {
                    message: "Product not found.".to_string(),
                    data: None,
                };
                HttpResponse::NotFound().json(response)
            }
            else
            {
                let response = CommonResponse::<Product> {
                    message: format!("{} products deleted.", result.deleted_count),
                    data: None,
                };
                HttpResponse::Ok().json(response)
            }
        },
        Err(error) =>
        {
            let response = CommonResponse::<String> {
                message: "Unknown error.".to_string(),
                data: None,
            };

            error!("Failed to delete product. Error: {:?}", error);

            HttpResponse::InternalServerError().json(response)
        },
    }
}

pub async fn get(database_data: web::Data<Database>, id: web::Path<String>) -> impl Responder
{
    info!("Get Product by ID requested...");

    let collection = database_data.products().await;

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
