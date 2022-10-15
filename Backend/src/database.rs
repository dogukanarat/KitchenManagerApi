use std::env;
extern crate dotenv;
use dotenv::dotenv;

use futures::StreamExt;
use mongodb::{
    bson::{extjson::de::Error},
    results::{ InsertOneResult},
    Client, Collection,
};

use crate::{models::Product};
use crate::console;

#[derive(Clone)]
pub struct Database {
    collection_products: Collection<Product>,
}

impl Database {
    pub async fn init() -> Self 
    {
        dotenv().ok();

        let uri = match env::var("MONGOURI") {
            Ok(v) => v.to_string(),
            Err(_) => format!("Error loading env variable"),
        };

        let client = Client::with_uri_str(uri).await.unwrap();

        let database = client.database("KitchenManager");

        let collection_products: Collection<Product> = database.collection("Products");

        Database { collection_products }
    }

    pub async fn products_create(&self, new_product: Product) -> Result<InsertOneResult, Error> 
    {
        console::info("Creating product...").await;

        let new_product = Product {
            id: None,
            name: new_product.name,
            price: new_product.price,
        };

        let product = self
            .collection_products
            .insert_one(new_product, None)
            .await
            .ok()
            .expect("Failed to insert document.");

        console::success("Created product...").await;

        Ok(product)
    }

    pub async fn products_list(&self) -> Result<Vec<Product>, Error> 
    {
        console::info("Listing product...").await;

        let mut products_cursor = self
            .collection_products
            .find(None, None)
            .await
            .expect("Failed to execute find.");

        let mut products_list: Vec<Product> = Vec::new();

        while let Some(product) = products_cursor.next().await {
            products_list.push(product.unwrap());
        }

        // Iterate over the results of the cursor.

        console::success("Listed product...").await;

        Ok(products_list)
    }
}