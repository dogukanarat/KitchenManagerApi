use actix_web::web::{Data, Path};
use actix_web::Responder;
use actix_web::{web, HttpResponse};
use chrono::{DateTime, NaiveDateTime, TimeZone, Utc};
use diesel::expression::AsExpression;
use diesel::{ExpressionMethods, Insertable, Queryable, RunQueryDsl};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::response::Response;
use crate::schema::products;
use crate::console::{info, self};
use crate::{DbPool, DbPooledConnection};

use diesel::query_dsl::methods::{FilterDsl, OrderDsl};
use diesel::sql_types::*;
use diesel::result::Error;
use std::str::FromStr;
use diesel::prelude::*;
#[cfg(test)]
use diesel::debug_query;
use diesel::insert_into;
use bigdecimal::BigDecimal;

pub type Products = Response<Product>;

#[derive(Debug, Deserialize, Serialize, Queryable)]
pub struct Product {
    pub id: i32,
    pub product_name: String,
    pub product_price: f32,
}

#[derive(Insertable)]
#[diesel(table_name = products)]
pub struct NewProduct {
    pub product_name: String,
    pub product_price: BigDecimal,
}

fn ll_product_create(new_product: &NewProduct, conn: &mut DbPooledConnection) -> Result<Product, Error> {

    let product = Product {
        id: 0,
        product_name: new_product.product_name.clone(),
        product_price: 10.0,
    };
    
    let _ = diesel::insert_into(products::table)
    .values(new_product.clone())
    .execute(conn)
    .expect("Error saving new product");

    Ok(product)
}

pub async fn products_create(pool: web::Data<DbPool>, name: web::Path<String>) -> impl Responder {

    console::info("Creating product...").await;

    let new_product = NewProduct { 
        product_name: name.to_string(), 
        product_price: BigDecimal::from(10)
    };

    let mut conn = pool.get().expect("CONNECTION_POOL_ERROR");

    let product = web::block(move || {
        ll_product_create(&new_product, &mut conn)
    }).await;

    console::info("Created product...").await;

    HttpResponse::Ok().json(Product {
        id: 1,
        product_name: "test".to_string(),
        product_price: 1.0,
    })
}

pub async fn products_list(pool: web::Data<DbPool>) -> impl Responder {

    console::info("Listing product...").await;

    // use crate::schema::products;

    // let new_post = NewPost { title, body };

    // diesel::insert_into(products::table)
    //     .values(&new_post)
    //     .get_result(conn)
    //     .expect("Error saving new post")

    HttpResponse::Ok().json(Product {
        id: 1,
        product_name: "test".to_string(),
        product_price: 1.0,
    })
}