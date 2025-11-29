#[derive(Debug, Clone)]
pub struct Order {
    pub id: u64,
    pub user_id: u64,
    pub side: Side,
    pub price: u64,
    pub quantity: u64,
    pub timestamp: u64
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Side {
    Buy,
    Sell
}


pub fn insert(order: Order) {
    //Implementation here
}