extern crate dotenv;
use dotenv::dotenv;
use futures::StreamExt;
use mongodb::{
    bson::{doc, extjson::de::Error},
    results::{InsertOneResult, UpdateResult, DeleteResult},
    Client, Collection,
};
use std::env;

use crate::products::{self, collection::ProductCollection};
use crate::console;

#[derive(Clone)]
pub struct Database
{
    collection_products: ProductCollection,
}

impl Database
{
    pub async fn init() -> Self
    {
        dotenv().ok();

        let uri = match env::var("MONGO_URI") {
            Ok(key) => key.to_string(),
            Err(_) => format!("Error loading env variable"),
        };

        let client = Client::with_uri_str(uri).await.unwrap();

        let database = client.database("KitchenManager");

        let collection_products = ProductCollection::init(database.clone()).await;

        Database { collection_products }
    }

    pub async fn products(&self) -> &ProductCollection
    {
        &self.collection_products
    }
}
