mod api;
mod models;
mod repository;

use actix_web::{ web::Data, App, HttpServer };

use api::product_api::create_product;
use repository::product_repository::ProductRepo;



#[actix_web::main]
async fn main() -> std::io::Result<()>{

    let db = ProductRepo::init().await;
    let db_data = Data::new(db);

    HttpServer::new(move || {
        App::new()
        .app_data(db_data.clone())
        .service(create_product)
    })
    .bind(("127.0.0.1", 8000))?
    .run()
    .await
}