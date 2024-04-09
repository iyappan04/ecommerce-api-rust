use std::env;
extern crate dotenv;

use dotenv::dotenv;

use futures::stream::TryStreamExt;

use mongodb::{
    bson::{doc, extjson::de::Error, oid::ObjectId},
    results::{InsertOneResult, UpdateResult, DeleteResult}, 
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
            updatedat:  None,
        };

        let produc = self 
            .col
            .insert_one(product, None)
            .await
            .ok()
            .expect("Error in Creating Product");

        Ok(produc)
    }


    pub async fn get_product(self, id: &String) -> Result<Product, Error> {
        
        let obj_id = ObjectId::parse_str(id).unwrap();
        let filter = doc! {"_id": obj_id};

        let product_detail = self
            .col
            .find_one(filter, None)
            .await 
            .ok()
            .expect("Error getting product detail");

        Ok(product_detail.unwrap())    
    }

    pub async fn update_product(&self, id: &String, new_product: Product) -> Result<UpdateResult, Error> {
        
        let obj_id = ObjectId::parse_str(id).unwrap();
        let filter = doc! {"_id": obj_id};


        // let new_pro = doc! {
        //     "$set":
        //         {
        //             "id": new_product.id, 
        //             "name": new_product.name,
        //             "image": new_product.image,
        //             "brand": new_product.brand,
        //             "category": new_product.category,
        //             "price": new_product.price,
        //             "quantity": new_product.quantity,
        //             "rating": new_product.rating,
        //             "description": new_product.description,
        //             "countinstock": new_product.countinstock,
        //             "createdat":  None,
        //             "updatedat":  Some(Utc::now())
        //         },
        // };

        let product = doc! {
            "$set": {
                // "id": new_product.id, 
                "name": new_product.name,
                "image": new_product.image,
                "brand": new_product.brand,
                "category": new_product.category,
                "price": new_product.price,
                "quantity": new_product.quantity,
                "rating": new_product.rating,
                "description": new_product.description,
                "countinstock": new_product.countinstock,
                // "createdat":  Utc::now(),
                // "updatedat":  Utc::now(),
            }
        };

        let update_product = self
            .col
            .update_one(filter, product, None)
            .await
            .ok()
            .expect("Error on update product");
        
        Ok((update_product))

    } 


    pub async fn delete_product(&self, id: &String) -> Result<DeleteResult, Error> {
        let obj_id = ObjectId::parse_str(id).unwrap();
        let filter = doc! {"_id": obj_id};
        let product_detail = self
            .col
            .delete_one(filter, None)
            .await
            .ok()
            .expect("Error deleting user");

        Ok(product_detail)
    }

    pub async fn get_all_products(&self) -> Result<Vec<Product>, Error> {
        let mut cursors = self
            .col
            .find(None, None)
            .await
            .ok()
            .expect("Error getting list of products");
        let mut products: Vec<Product> = Vec::new();
        while let Some(product) = cursors
            .try_next()
            .await
            .ok()
            .expect("Error mapping through cursor")
        {
            products.push(product)
        }
        Ok(products)
    }


    
}
