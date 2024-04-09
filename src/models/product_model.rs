use mongodb::bson::oid::ObjectId;
use serde::{ Serialize, Deserialize };
use chrono::{ DateTime, Utc };


#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Product{ 
    pub id: Option<ObjectId>, 
    pub name: String,
    pub image: String,
    pub brand: String,
    pub category: String,
    pub price: i32,
    pub quantity: i32,
    pub rating: i32,
    pub description: String,
    pub countinstock: i32,
    pub createdat:  Option<DateTime<Utc>>,
}