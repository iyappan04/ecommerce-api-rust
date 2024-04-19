use crate::{ models::product_model::Product, repository::product_repository::ProductRepo };
use actix_web::{
    post, get,
    web::{ Data, Json, Path },
    HttpResponse
};

// use chrono::{ DateTime, Utc };
// use mongodb::bson::oid::ObjectId;


#[post("/product")]
pub async fn create_product(db: Data<ProductRepo>, mut new_product: Json<Product>) -> HttpResponse {

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
    
    let product_detail = db.createProduct(new_product.into_inner()).await;

    match product_detail {
        Ok(product) => HttpResponse::Ok().json(product),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }

}


#[get("/product/{id}")]
pub async fn get_product_by_id(db: Data<ProductRepo>, path: Path<String>) -> HttpResponse {

    let id = path.into_inner();
    
    if id.is_empty() {
        return HttpResponse::BadRequest().body("Invalid ID");
    }

    let product_detail = db.get_product(&id).await;

    match product_detail {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

