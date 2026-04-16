use serde::{Deserialize, Serialize};
use crate::inputs::CreateOrderInput;

#[derive(Serialize)]
pub struct OrderResponse {
    pub message: String,
    pub order: CreateOrderInput,
}

#[derive(Serialize, Deserialize)]
pub struct DeleteOrderResponse{
    pub failed_qty: u32,
    pub average_price:u32,
}

#[derive(Serialize, Deserialize)]
pub struct Depth{
    pub bids: Vec<[u32;2]>,
    pub asks: Vec<[u32;2]>,
    pub lastUpdateId:String
}
