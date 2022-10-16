extern crate dotenv;
use dotenv::dotenv;
use futures::StreamExt;
use mongodb::{
    bson::{doc, extjson::de::Error},
    results::{InsertOneResult, UpdateResult, DeleteResult},
    Client, Collection,
};
use std::env;

use crate::console;
use super::model::*;

#[derive(Clone)]
pub struct ProductCollection
{
    collection_products: Collection<Product>,
}

impl ProductCollection
{
    pub async fn init(database: mongodb::Database) -> Self
    {
        let collection_products: Collection<Product> = database.collection("Products");

        ProductCollection { collection_products }
    }

    pub async fn products_create(&self, content: ProductCreateRequest) -> Result<InsertOneResult, Error>
    {
        console::info("Creating product...").await;

        let new_product = Product { id: None,
                                    name: content.name,
                                    price: content.price };

        let product = self.collection_products
                          .insert_one(new_product, None)
                          .await
                          .ok()
                          .expect("Failed to insert document.");

        console::success("Created product...").await;

        Ok(product)
    }

    pub async fn products_list(&self, start: u64, count: i64) -> Result<Vec<Product>, Error>
    {
        console::info("Listing product...").await;

        let find_options = mongodb::options::FindOptions::builder().skip(start)
                                                                   .limit(count)
                                                                   .build();

        let mut products_cursor = self.collection_products
                                      .find(None, find_options)
                                      .await
                                      .ok()
                                      .expect("Failed to execute find.");

        let mut products_list: Vec<Product> = Vec::new();

        while let Some(product) = products_cursor.next().await {
            products_list.push(product.unwrap());
        }

        console::success("Listed product...").await;

        Ok(products_list)
    }

    pub async fn products_update(&self,
                                 filter: ProductRequestFilter,
                                 content: ProductUpdateRequestContent)
                                 -> Result<UpdateResult, Error>
    {
        console::info("Updating product...").await;

        let filter = doc! { "name": filter.name };
        let update = doc! { "$set": { "name": content.name, "price": content.price } };

        let result = self.collection_products
                         .update_one(filter, update, None)
                         .await
                         .expect("Failed to update document.");

        console::success("Updated product...").await;

        Ok(result)
    }

    pub async fn products_find(&self, filter: ProductRequestFilter) -> Result<Product, Error>
    {
        console::info("Finding product...").await;

        let filter = doc! { "name": filter.name };

        let product = self.collection_products
                          .find_one(filter, None)
                          .await
                          .ok()
                          .expect("Failed to execute find.");

        console::success("Found product...").await;

        Ok(product.unwrap())
    }

    pub async fn products_delete(&self, filter: ProductRequestFilter) -> Result<DeleteResult, Error>
    {
        console::info("Deleting product...").await;

        let filter = doc! { "name": filter.name };

        let result = self.collection_products
                         .delete_one(filter, None)
                         .await
                         .expect("Failed to delete document.");

        console::success("Deleted product...").await;

        Ok(result)
    }

    pub async fn products_get_by_id(&self, id: String) -> Result<Product, Error>
    {
        console::info("Getting product by id...").await;

        let filter = doc! { "_id": id };

        let product = self.collection_products
                          .find_one(filter, None)
                          .await
                          .ok()
                          .expect("Failed to execute find.");

        console::success("Got product by id...").await;

        Ok(product.unwrap())
    }

    pub async fn products_delete_by_id(&self, id: String) -> Result<DeleteResult, Error>
    {
        console::info("Deleting product by id...").await;

        let filter = doc! { "_id": id };

        let result = self.collection_products
                         .delete_one(filter, None)
                         .await
                         .expect("Failed to delete document.");

        console::success("Deleted product by id...").await;

        Ok(result)
    }

    pub async fn products_update_by_id(&self,
                                       id: String,
                                       content: ProductUpdateRequestContent)
                                       -> Result<UpdateResult, Error>
    {
        console::info("Updating product by id...").await;

        let filter = doc! { "_id": id };
        let update = doc! { "$set": { "name": content.name, "price": content.price } };

        let result = self.collection_products
                         .update_one(filter, update, None)
                         .await
                         .expect("Failed to update document.");

        console::success("Updated product by id...").await;

        Ok(result)
    }
}
