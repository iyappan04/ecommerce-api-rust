use mongodb::bson::oid::ObjectId;
use serde::{ Serialize, Deserialize };
use chrono::{ DateTime, Utc };

use product_model::Product;

pub struct Order {
    pub user:  Option<ObjectId>,
    pub id: Option<ObjectId>, 
    pub paymentmethod: String,
    pub taxprice: i64,
    pub shippingprice: i64,
    pub totalprice: i64,
    pub ispaid: Bool,
    pub paidat: Option<DateTime<Utc>>,
    pub isdelivered: Bool,
    pub deliveredat: Option<DateTime<Utc>>,
    pub createdat:  Option<DateTime<Utc>>,
}

pub struct OrderItem {
    pub product: Option<ObjectId>,
    pub order: Option<ObjectId>,
    pub name: String,
    pub image: String,    
    pub price: i32,
    pub quantity: i32,
}

pub struct ShippingAddress {
    pub order: Option<ObjectId>,
    pub address: String,
    pub city: String,    
    pub postalcode: i64,
    pub country: String,
}



