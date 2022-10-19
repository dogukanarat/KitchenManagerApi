use mongodb::bson::oid::ObjectId;
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum ProductKind {
    Food = 0,
    Coctail,
    ReadyMade,
    Other,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Product {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub name: String,
    pub price: f32,
    pub kind: ProductKind,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ProductListQuery
{
    pub offset: Option<u64>,
    pub limit: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ProductCreateRequest
{
    pub name: String,
    pub price: f32,
    pub kind: ProductKind,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ProductUpdateRequest
{
    pub name: Option<String>,
    pub price: Option<f32>,
    pub kind: Option<ProductKind>,
}