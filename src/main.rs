mod api;
mod models;
mod repository;

use actix_web::{ web::Data, App, HttpServer };

use api::product_api::create_product;
use repository::product_repository::ProductRepo;

use actix_web::{get, HttpRequest, Responder};


#[get("/")]
async fn index(_req: HttpRequest) -> impl Responder {
    "Welcome!"
}

#[actix_web::main]
async fn main() -> std::io::Result<()>{

    let db = ProductRepo::init().await;
    let db_data = Data::new(db);

    HttpServer::new(move || {
        App::new()
        .app_data(db_data.clone())
        .service(create_product)
        .service(index)
    })
    // .bind_uds("/home/iyappan/app.sock")?
    .bind(("0.0.0.0", 8000))?
    .run()
    .await
}