use serde::{Deserialize, Serialize};

pub type OrderId = u64;

#[derive(Debug, Clone, Serialize, Deserialize, Copy)]
pub struct Order {
    pub id: OrderId,
    pub user_id: u64,
    pub side: Side,
    pub price: u64,
    pub quantity: u64,
    pub timestamp: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Copy)]
pub enum Side {
    Bid, //Price at which buyer is willing to buy an asset
    Ask, //Price at which seller is willing to sell an asset
}
