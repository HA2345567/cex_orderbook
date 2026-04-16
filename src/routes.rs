use actix_web::{delete, get, post, web::{Data, Json}, HttpResponse, Responder};

use crate::inputs::{CreateOrderInput, DeleteOrder};
use crate::orderbook::Orderbook;
use crate::output::{DeleteOrderResponse, Depth, OrderResponse};
use std::sync::{Arc, Mutex};


#[post("/order")]
pub async fn create_order(
    input: Json<CreateOrderInput>,
    _orderbook: Data<Arc<Mutex<Orderbook>>>,
) -> impl Responder {
    let body = input.into_inner();
    let price = body.price;
    let quantity = body.quantity;
    let user_id = body.user_id;
    let side = body.side.clone();

    let mut orderbook = _orderbook.lock().unwrap();
    orderbook.create_order(price, quantity, user_id, side);

    let response = OrderResponse {
        message: String::from("Order created"),
        order: body,
    };

    HttpResponse::Ok().json(response)
}

#[delete("/order/{id}")]
pub async fn delete_order(Json(body): Json<DeleteOrder>) -> impl Responder {
    let _order_id = body.order_id;
    HttpResponse::Ok().json(DeleteOrderResponse{
        failed_qty:0,
        average_price:100
    })
}

#[get("/depth")]
pub async fn get_depth() -> impl Responder {
    HttpResponse::Ok().json(Depth{
        bids:vec![],
        asks:vec![],
        lastUpdateId:String::from("dsfdkf")
    })
}
