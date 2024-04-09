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

use crate::models::user_model::User;

pub struct UserRepo {
    col: Collection<User>
}

impl UserRepo {

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

        let col: Collection<User> = db.collection("User");

        UserRepo { col }
    }

    // pub async register() {

    // }
}