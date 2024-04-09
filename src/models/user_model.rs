use serde::{ Serialize, Deserialize };

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct User{
    username: String,
    email: String,
    phone: i32,
    password: String,
    isadmin: Bool,
}
