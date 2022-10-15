use mongodb::bson::oid::ObjectId;
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Product {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub name: String,
    pub price: f32
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ProductListQuery
{
    pub start: Option<u64>,
    pub count: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ProductUpdateRequestContent
{
    pub name: String,
    pub price: f32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ProductCreateRequest
{
    pub name: String,
    pub price: f32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ProductRequestFilter
{
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ProductUpdateRequest
{
    pub filter: ProductRequestFilter,
    pub content: ProductUpdateRequestContent,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ProductDeleteRequest
{
    pub filter: ProductRequestFilter
}