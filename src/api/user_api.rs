use crate::{ models::user_model::User, repository::user_repository::UserRepo };
use actix_web::{
    post, get, delete,put,
    web::{ Data, Json, Path },
    HttpResponse
};



#[post("/register")]
pub async fn register(db: Data<UserRepo>, mut new_user: Json<User>) -> HttpResponse {

    // let data = Product {
    //     id: None, 
    //     name: new_product.name.to_owned(),
    //     image: new_product.image.to_owned(),
    //     brand: new_product.brand.to_owned(),
    //     category: new_product.category.to_owned(),
    //     price: new_product.price.to_owned(),
    //     quantity: new_product.quantity.to_owned(),
    //     rating: new_product.rating.to_owned(),
    //     description: new_product.description.to_owned(),
    //     countinstock: new_product.countinstock.to_owned(),
    // };

    new_user.password = Some(Utc::now());
    
    let user_detail = db.createProduct(new_user.into_inner()).await;

    match user_detail {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }

}

#[post("/login")]
pub async fn login(db: Data<UserRepo>, mut new_user: Json<User>) -> HttpResponse {

    // let data = Product {
    //     id: None, 
    //     name: new_product.name.to_owned(),
    //     image: new_product.image.to_owned(),
    //     brand: new_product.brand.to_owned(),
    //     category: new_product.category.to_owned(),
    //     price: new_product.price.to_owned(),
    //     quantity: new_product.quantity.to_owned(),
    //     rating: new_product.rating.to_owned(),
    //     description: new_product.description.to_owned(),
    //     countinstock: new_product.countinstock.to_owned(),
    // };

    // new_product.createdat = Some(Utc::now());
    
    let user_detail = db.createProduct(new_user.into_inner()).await;

    match user_detail {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }

}
