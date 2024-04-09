use std::env;
extern crate dotenv;

use dotenv::dotenv;

use futures::stream::TryStreamExt;

use mongodb::{
    bson::{doc, extjson::de::Error, oid::ObjectId},
    results::InsertOneResult, 
    Client, Collection,
};

use chrono::{ DateTime, Utc };

use crate::models::product_model::Product;
pub struct ProductRepo {
    col: Collection<Product>
}

impl ProductRepo {

    pub async fn init() -> Self {

        dotenv().ok();

        let uri = match env::var("MONGODB_URI"){
            Ok(v) => v.to_string(),
            Err(_) => format!("Error on loading env"),
        };

        let client = Client::with_uri_str(uri)
            .await
            .expect("Error on connecting Database");

        let db = client.database("ecommerceDATA");

        let col: Collection<Product> = db.collection("Product");

        ProductRepo { col }
    }

    pub async fn createProduct(&self, new_product: Product) -> Result<InsertOneResult, Error> {
        let product = Product {
            id: None, 
            name: new_product.name,
            image: new_product.image,
            brand: new_product.brand,
            category: new_product.category,
            price: new_product.price,
            quantity: new_product.quantity,
            rating: new_product.rating,
            description: new_product.description,
            countinstock: new_product.countinstock,
            createdat:  Some(Utc::now()),
            updatedat:  Some(Utc::now()),
        };


        let produc = self 
            .col
            .insert_one(product, None)
            .await
            .ok()
            .expect("Error in Creating Product");

        Ok(produc)



    }
}
