extern crate dotenv;
use dotenv::dotenv;
use futures::StreamExt;
use mongodb::{
    bson::{doc, extjson::de::Error, oid::ObjectId, self},
    results::{DeleteResult, InsertOneResult, UpdateResult},
    Client, Collection,
};
use std::{env, str::FromStr};

use super::model::*;
use crate::console;
use crate::products;

#[derive(Clone)]
pub struct OrderCollection
{
    collection_order: Collection<Order>,
}

#[derive(Debug)]
pub enum OrderCollectionError
{
    OneOfProductsNotFound,
    OrderNotFound,
    OrderNotModified,
    CustomError(String),
}

impl OrderCollection
{
    /// Creates a new instance of the OrderCollection
    ///
    /// # Arguments
    ///
    /// * `database` - The database to use
    ///
    ///
    pub async fn init(database: mongodb::Database) -> Self
    {
        let collection_order: Collection<Order> = database.collection("Orders");

        OrderCollection { collection_order }
    }

    /// Create new order
    ///
    /// # Arguments
    ///
    /// * `content` - OrderCreateRequest
    /// * `products` - ProductCollection
    ///
    /// # Examples
    ///
    /// ```
    /// // TODO
    ///
    /// ```
    pub async fn create(
        &self,
        content: OrderCreateRequest,
        collection_products: &products::collection::ProductCollection,
    ) -> Result<InsertOneResult, OrderCollectionError>
    {
        info!("Creating order...");

        let mut total_price = 0.0;

        let mut status: OrderStatus = OrderStatus::Completed;

        for product_view in content.clone().products
        {
            let product_id = product_view.id;

            let product_result = collection_products.get(product_id).await;

            match product_result
            {
                Ok(product) =>
                {
                    total_price += product.price * product_view.quantity as f32;

                    if product.kind != products::model::ProductKind::ReadyMade
                    {
                        status = OrderStatus::Pending;
                    }
                },
                Err(error) => return Err(OrderCollectionError::OneOfProductsNotFound),
            }
        }

        let new_order = Order {
            id: None,
            order_id: 0,
            products: content.products,
            total_price: total_price,
            status: status,
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
        };

        let result = self.collection_order.insert_one(new_order, None).await;

        match result
        {
            Ok(result) => Ok(result),
            Err(error) => Err(OrderCollectionError::CustomError(error.to_string())),
        }
    }

    /// Get all orders
    ///
    ///
    pub async fn list(&self, offset: u64, limit: i64) -> Result<Vec<Order>, OrderCollectionError>
    {
        info!("Getting all orders...");

        let find_options = mongodb::options::FindOptions::builder()
            .skip(offset)
            .limit(limit)
            .build();

        let mut cursor = self.collection_order.find(None, find_options).await.unwrap();

        let mut orders: Vec<Order> = Vec::new();

        while let Some(result) = cursor.next().await
        {
            match result
            {
                Ok(document) =>
                {
                    orders.push(document);
                },
                Err(error) => return Err(OrderCollectionError::CustomError(error.to_string())),
            }
        }

        Ok(orders)
    }

    /// Get a single order
    ///
    /// # Arguments
    ///
    /// * `id` - ObjectId
    /// * `order_id` - String
    ///
    /// # Examples
    ///
    /// ```
    /// // TODO
    ///
    /// ```
    pub async fn get(&self, req_id: String) -> Result<Order, OrderCollectionError>
    {
        info!("Getting order...");

        let id = ObjectId::from_str(&req_id).unwrap();

        let filter = doc! { "_id": id };

        let result = self.collection_order.find_one(filter, None).await;

        match result
        {
            Ok(result) => match result
            {
                Some(order) => Ok(order),
                None => Err(OrderCollectionError::OrderNotFound),
            },
            Err(error) => Err(OrderCollectionError::CustomError(error.to_string())),
        }
    }

    /// Update a single order
    ///
    /// # Arguments
    ///
    /// * `id` - ObjectId
    /// * `order_id` - String
    /// * `content` - OrderUpdateRequest
    ///
    /// ```
    /// # Examples
    ///
    /// ```
    pub async fn update(
        &self,
        req_id: String,
        content: OrderUpdateRequest,
    ) -> Result<UpdateResult, OrderCollectionError>
    {
        info!("Updating order...");

        let id = ObjectId::from_str(&req_id).unwrap();

        let filter = doc! { "_id": id };

        let status_as_string = bson::to_bson(&content.status).unwrap();
        let status = status_as_string.as_str().unwrap();

        let update = doc! { "$set": { "status": status } };

        let result = self.collection_order.update_one(filter, update, None).await;

        match result
        {
            Ok(result) => match result.modified_count
            {
                1 => Ok(result),
                _ => Err(OrderCollectionError::OrderNotModified),
            },
            Err(error) => Err(OrderCollectionError::CustomError(error.to_string())),
        }
    }

    /// Delete a single order
    /// 
    /// # Arguments
    /// 
    /// * `req_id` - ObjectId
    /// 
    /// ```
    /// # Examples
    /// 
    /// ```
    pub async fn delete(&self, req_id: String) -> Result<DeleteResult, OrderCollectionError>
    {
        info!("Deleting order...");

        let id = ObjectId::from_str(&req_id).unwrap();

        let filter = doc! { "_id": id };

        let result = self.collection_order.delete_one(filter, None).await;

        match result
        {
            Ok(result) => match result.deleted_count
            {
                1 => Ok(result),
                _ => Err(OrderCollectionError::OrderNotFound),
            },
            Err(error) => Err(OrderCollectionError::CustomError(error.to_string())),
        }
    }
}
