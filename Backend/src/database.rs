extern crate dotenv;
use dotenv::dotenv;
use futures::StreamExt;
use mongodb::{
    bson::{doc, extjson::de::Error},
    results::{DeleteResult, InsertOneResult, UpdateResult},
    Client, Collection,
};
use std::env;

use crate::console;
use crate::orders::{self};
use crate::products::{self};

#[derive(Clone)]
pub struct Database
{
    collection_products: products::collection::ProductCollection,
    collection_orders: orders::collection::OrderCollection,
}

impl Database
{
    pub async fn init() -> Self
    {
        dotenv().ok();

        let uri = match env::var("MONGO_URI")
        {
            Ok(key) => key.to_string(),
            Err(_) => format!("Error loading env variable"),
        };

        let client = Client::with_uri_str(uri).await.unwrap();

        let database = client.database("KitchenManager");

        let collection_products =
            products::collection::ProductCollection::init(database.clone()).await;
        let collection_orders = orders::collection::OrderCollection::init(database.clone()).await;

        Database {
            collection_products,
            collection_orders,
        }
    }

    pub async fn products(&self) -> &products::collection::ProductCollection
    {
        &self.collection_products
    }

    pub async fn orders(&self) -> &orders::collection::OrderCollection
    {
        &self.collection_orders
    }
}
