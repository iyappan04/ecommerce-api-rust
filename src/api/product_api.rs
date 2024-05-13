use crate::{ models::product_model::Product, repository::product_repository::ProductRepo };
use actix_web::{
    post, get, delete,put,
    web::{ Data, Json, Path },
    HttpResponse
};

// use chrono::{ DateTime, Utc };
// use mongodb::bson::oid::ObjectId;


#[post("/createproduct")]
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


#[get("/getproduct/{id}")]
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


#[put("/user/{id}")]
pub async fn update_user(
    db: Data<ProductRepo>,
    path: Path<String>,
    new_user: Json<Product>,
) -> HttpResponse {
    let id = path.into_inner();
    
    if id.is_empty() {
        return HttpResponse::BadRequest().body("invalid ID");
    };

    let data = User {
        id: Some(ObjectId::parse_str(&id).unwrap()),
        name: new_user.name.to_owned(),
        location: new_user.location.to_owned(),
        title: new_user.title.to_owned(),
    };

    let update_result = db.update_user(&id, data).await;

    match update_result {
        Ok(update) => {
            if update.matched_count == 1 {
                let updated_user_info = db.get_user(&id).await;

                return match updated_user_info {
                    Ok(user) => HttpResponse::Ok().json(user),
                    Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
                };
            } else {
                return HttpResponse::NotFound().body("No user found with specified ID");
            }
        }
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}


#[delete("/deleteproduct/{id}")]
pub async fn delete_product(db: Data<ProductRepo>, path: Path<String>) -> HttpResponse {

    let id: String = path.into_inner();
    if id.is_empty() {
        return HttpResponse::BadRequest().body("Invalid ID");
    };
    let result = db.delete_product(&id).await;

    match result {
        Ok(res) => {
            if res.deleted_count == 1 {
                return HttpResponse::Ok().json("Product successfully deleted!");
            } else {
                return HttpResponse::NotFound().json("Product with specified ID not found!");
            }
        }
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

#[get("/getproducts")]
pub async fn get_all_products(db: Data<ProductRepo>) -> HttpResponse {
    let products = db.get_all_products().await;

    match products {
        Ok(products) => HttpResponse::Ok().json(products),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}