use std::env;
extern crate dotenv;

use dotenv::dotenv;

use futures::stream::TryStreamExt;

use mongodb::{
    bson::{doc, extjson::de::Error, oid::ObjectId},
    results::InsertOneResult, 
    Client, Collection,
};

// extern crate bcrypt;
// use bcrypt::{DEFAULT_COST, hash, verify};


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


    pub async fn registerUser(&self, user: User) -> Result<InsertOneResult, Error> {

        let user = User {
            id: None, 
            email: new_product.email,
            password: new_product.password,
            isadmin: new_product.isadmin,
        };

        // let hashed = hash("hunter2", DEFAULT_COST)?;
        // let valid = verify("hunter2", &hashed)?;

        let user = self 
            .col
            .insert_one(user, None)
            .await
            .ok()
            .expect("Error in Creating User");

        Ok(user)
    }

    pub async fn get_user(&self, email: &String) -> Result<User, Error> {
        
        // let email = ObjectId::parse_str(email).unwrap();

        let filter = doc! {"email": email};

        let get_user = self
            .col
            .find_one(filter, None)
            .await 
            .ok()
            .expect("Error getting email detail");

        Ok(get_user.unwrap())   

    }

    async fn get_token_handler(Json(user): Json<User>) -> HttpResponse {
        let token = jwt_lib::get_jwt(user);
      
        match token {
          Ok(token) => HttpResponse::Ok().json(json!({
            "success": true,
            "data": {
              "token": token
            }
          })),
      
          Err(error) => HttpResponse::BadRequest().json(json!({
            "success": false,
            "data": {
              "message": error
            }
          })),
        }
      }
   

  
}