use chrono::{Utc, DateTime};
use mongodb::bson::oid::ObjectId;
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ProductView {
    pub id: String,
    pub quantity: i32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum OrderStatus {
    Pending = 0,
    Completed,
    Cancelled,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Order {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub order_id: i32,
    pub products: Vec<ProductView>,
    pub total_price: f32,
    pub status: OrderStatus,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct OrderListQuery
{
    pub offset: Option<u64>,
    pub limit: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct OrderCreateRequest
{
    pub products: Vec<ProductView>
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct OrderUpdateRequest
{
    pub status: OrderStatus,
}