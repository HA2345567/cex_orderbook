use serde::{Deserialize, Serialize};


#[derive(Debug, Serialize, Deserialize)]
pub struct CreateOrderInput {
    pub price: u32,
    pub quantity: u32,
    pub user_id: u32,
    pub side: Side,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
pub enum Side {
    Buy,
    Sell
}



#[derive(Debug, Serialize, Deserialize)]
pub  struct DeleteOrder{
   pub order_id: String
}
