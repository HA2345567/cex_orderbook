use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub struct CreateOrderResponse {
   pub order_id: String,
   pub filled_quantity: f64,
   pub remaining_quantity:f64,
   pub average_price: f64,
}

#[derive(Serialize, Deserialize)]
pub struct DeleteOrderResponse{
    pub success: bool,
    pub remaining_quantity:f64,
    pub filled_quantity:f64,
    pub average_price:f64,
}

#[derive(Serialize, Deserialize)]
pub struct Depth{
    pub bids: Vec<[u32;2]>,
    pub asks: Vec<[u32;2]>,
    #[serde(rename = "lastUpdateId")]
    pub last_update_id: String,
}
