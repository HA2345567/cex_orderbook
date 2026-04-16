use crate::{inputs::Side, output::Depth};
use std::collections::HashMap;

pub struct Orderbook {
    pub bids: HashMap<u32, Vec<UserOrder>>,
    pub asks: HashMap<u32, Vec<UserOrder>>,
    pub order_id_index : u32
   
}

pub struct UserOrder {
    pub user_id: u32,
    pub qty: u32,
    pub order_id: u32,
}

impl Orderbook {
    pub  fn new() -> Self {
        Self {
            bids: HashMap::new(),
            asks: HashMap::new(),
            order_id_index:0,
        }
    }
}

impl Orderbook {
    pub fn create_order(&mut self, price: u32, quantity: u32, user_id: u32, side: Side) {
        self.order_id_index +=1;
        let order_id = self.order_id_index;
        if side == Side::Buy {
           self.bids
                .entry(price)
                .or_insert(Vec::new())
                .push(UserOrder {
                    user_id,
                    qty: (quantity),
                    order_id: (12322),
                });
        } else {
            self.asks
                .entry(price)
                .or_insert(Vec::new())
                .push(UserOrder {
                    user_id,
                    qty: (quantity),
                    order_id: (12322),
                });
        }
    }
      //implement delete function for orderbook
      // implement get_depth function for orderbook
    

}
