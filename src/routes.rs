use actix_web::{delete, get, post, web::Json, HttpResponse, Responder};

use crate::inputs::{CreateOrderInput, DeleteOrder};
use crate::output::{DeleteOrderResponse, Depth, OrderResponse};


#[post("/order")]
pub async fn create_order(input: Json<CreateOrderInput>) -> impl Responder {
    let response = OrderResponse {
        message: "order created".to_string(),
        order: input.into_inner(),
    };

    //implements orderbook logic
    

    HttpResponse::Ok().json(response)
}

#[delete("/order/{id}")]
pub async fn delete_order(Json(body): Json<DeleteOrder>) -> impl Responder {
    let order_id = body.order_id;
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
