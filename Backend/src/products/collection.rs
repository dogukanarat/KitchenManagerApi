extern crate dotenv;
use dotenv::dotenv;
use futures::StreamExt;
use log;
use mongodb::bson::oid::ObjectId;
use mongodb::{
    bson::{doc, extjson::de::Error},
    results::{DeleteResult, InsertOneResult, UpdateResult},
    Client, Collection,
};
use std::{env, str::FromStr};

use super::model::*;
use crate::console;

#[derive(Clone)]
pub struct ProductCollection
{
    collection_products: Collection<Product>,
}

#[derive(Debug)]
pub enum ProductCollectionError
{
    ProductNameExists,
    ProductNotFound,
    ProductNotMofified,
    CustomError(String),
}

impl ProductCollection
{
    pub async fn init(database: mongodb::Database) -> Self
    {
        let collection_products: Collection<Product> = database.collection("Products");

        ProductCollection {
            collection_products,
        }
    }

    pub async fn create(
        &self,
        content: ProductCreateRequest,
    ) -> Result<InsertOneResult, ProductCollectionError>
    {
        info!("Creating product...");

        let is_exist = self.is_name_exist(content.name.clone()).await;

        match is_exist
        {
            Ok(true) =>
            {
                error!("Product name already exist.");
                Err(ProductCollectionError::ProductNameExists)
            },
            Ok(false) =>
            {
                let product = Product {
                    id: None,
                    name: content.name,
                    price: content.price,
                    kind: content.kind,
                };

                let result = self.collection_products.insert_one(product, None).await;

                match result
                {
                    Ok(result) => Ok(result),
                    Err(_) => Err(ProductCollectionError::CustomError(
                        "Failed to create product.".to_string(),
                    )),
                }
            },
            Err(_) =>
            {
                error!("Failed to check product name.");
                Err(ProductCollectionError::CustomError(
                    "Failed to check product name.".to_string(),
                ))
            },
        }
    }

    pub async fn list(
        &self,
        offset: u64,
        limit: i64,
    ) -> Result<Vec<Product>, ProductCollectionError>
    {
        info!("Listing product...");

        let find_options = mongodb::options::FindOptions::builder()
            .skip(offset)
            .limit(limit)
            .build();

        let mut products_cursor = self
            .collection_products
            .find(None, find_options)
            .await
            .ok()
            .expect("Failed to execute find.");

        let mut products_list: Vec<Product> = Vec::new();

        while let Some(product) = products_cursor.next().await
        {
            products_list.push(product.unwrap());
        }

        info!("Listed product...");

        Ok(products_list)
    }

    pub async fn update(
        &self,
        req_id: String,
        content: ProductUpdateRequest,
    ) -> Result<UpdateResult, ProductCollectionError>
    {
        info!("Updating product...");

        let id = ObjectId::from_str(&req_id).unwrap();

        let filter = doc! { "_id": id };
        let update = doc! { "$set": { "name": content.name, "price": content.price } };

        let result = self
            .collection_products
            .update_one(filter, update, None)
            .await;

        match result
        {
            Ok(result) =>
            {
                if result.matched_count == 0
                {
                    error!("Product not found.");
                    return Err(ProductCollectionError::ProductNotFound);
                }

                if result.modified_count == 0
                {
                    error!("Product not modified.");
                    return Err(ProductCollectionError::ProductNotMofified);
                }

                Ok(result)
            },
            Err(error) =>
            {
                error!("Failed to update product. Error: {:?}", error);
                Err(ProductCollectionError::CustomError(
                    format!("Failed to update product. Error: {:?}", error).to_string(),
                ))
            },
        }
    }

    pub async fn get(&self, id: String) -> Result<Product, ProductCollectionError>
    {
        info!("Getting product by id...");

        let filter = doc! { "_id": id };

        let result = self.collection_products.find_one(filter, None).await;

        match result
        {
            Ok(Some(product)) => Ok(product),
            Ok(None) =>
            {
                error!("Product not found.");
                Err(ProductCollectionError::ProductNotFound)
            },
            Err(_) =>
            {
                error!("Failed to get product by id.");
                Err(ProductCollectionError::CustomError(
                    "Failed to get product by id.".to_string(),
                ))
            },
        }
    }

    pub async fn delete(&self, req_id: String) -> Result<DeleteResult, ProductCollectionError>
    {
        info!("Deleting product by id...");

        let id = ObjectId::from_str(&req_id).unwrap();

        let filter = doc! { "_id": id };

        let result = self.collection_products.delete_one(filter, None).await;

        match result
        {
            Ok(result) => Ok(result),
            Err(_) =>
            {
                error!("Failed to delete product by id.");
                Err(ProductCollectionError::CustomError(
                    "Failed to delete product by id.".to_string(),
                ))
            },
        }
    }

    pub async fn is_name_exist(&self, name: String) -> Result<bool, ProductCollectionError>
    {
        info!("Checking if product name is exist...");

        let filter = doc! { "name": name };

        let product = self.collection_products.find_one(filter, None).await;

        match product
        {
            Ok(Some(_)) => Ok(true),
            Ok(None) => Ok(false),
            Err(_) =>
            {
                error!("Failed to check product name.");
                Err(ProductCollectionError::CustomError(
                    "Failed to check product name.".to_string(),
                ))
            },
        }
    }
}
