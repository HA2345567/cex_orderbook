use std::sync::Arc;

use actix_web::{web::Data, App, HttpServer};
use std::sync::Mutex;

use crate::orderbook::Orderbook;
use crate::routes::{create_order, delete_order, get_depth};

pub mod routes;
pub mod output;
pub mod inputs;
pub mod orderbook;

#[actix_web::main]
async fn main() -> Result<(), std::io::Error> {
    let orderbook = Arc::new(Mutex::new(Orderbook::default()));

    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(orderbook.clone()))
            .service(create_order)
            .service(delete_order)
            .service(get_depth)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

