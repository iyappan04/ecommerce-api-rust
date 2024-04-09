use serde::{ Serialize, Deserialize };

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    sub: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct User{
    pub id: Option<ObjectId>, 
    pub username: String,
    pub email: String,
    pub phone: i32,
    pub password: String,
    pub isadmin: Bool,
}
