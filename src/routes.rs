use std::sync::{Arc, Mutex};

use actix_web::{delete, get, post, web::{Data, Json}, HttpResponse, Responder};

use crate::{
    inputs::{CreateOrderInput, DeleteOrderInput},
    orderbook::Orderbook,
    output::{CreateOrderResponse, DeleteOrderResponse},
};






#[post("/order")]
pub async fn create_order(orderbook: Data<Arc<Mutex<Orderbook>>>,order: Json<CreateOrderInput>)-> impl Responder {
    let order = order.into_inner();
    let mut orderbook = orderbook.lock().unwrap();
    let order_id = orderbook.order_id_index.to_string();
    let price = order.price;
    let quantity = order.quantity;
    orderbook.create_order(order);

    HttpResponse::Ok().json(CreateOrderResponse {
        order_id,
        filled_quantity: 0.0,
        remaining_quantity: quantity,
        average_price: price,
    })

}

#[delete("/order/{id}")]
pub async  fn delete_order(orderbook: Data<Arc<Mutex<Orderbook>>>, order: Json<DeleteOrderInput>)-> impl Responder{
    let mut orderbook = orderbook.lock().unwrap();
    orderbook.delete_order(order.into_inner());

    HttpResponse::Ok().json(DeleteOrderResponse {
        success: true,
        remaining_quantity: 0.0,
        filled_quantity: 0.0,
        average_price: 0.0,
    })
}

#[get("/depth")]
pub async fn get_depth(orderbook: Data<Arc<Mutex<Orderbook>>>) -> impl Responder{
    let orderbook = orderbook.lock().unwrap();
    let depth = orderbook.get_depth();
    HttpResponse::Ok().json(depth)


}
