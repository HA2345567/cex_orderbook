use serde::{Deserialize, Serialize};


#[derive(Debug, Serialize, Deserialize)]
pub struct CreateOrderInput {
    pub price: f64,
    pub quantity: f64,
    pub user_id: String,
    pub side: Side,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
pub enum Side {
    Buy,
    Sell
}
#[derive(Debug, Serialize, Deserialize)]
pub  struct DeleteOrderInput{
   pub order_id: String,
   pub user_id:  String
}
