use std::collections::{BTreeMap, VecDeque};

use crate::order::{Order, Side};

//Stores all of the order at this price level
pub struct PriceLevel {
    pub price: u64,
    pub orders: VecDeque<Order>
}

pub struct BookSide {
    pub levels: BTreeMap<u64, PriceLevel>
}

impl PriceLevel {
    pub fn new(price: u64) -> Self {
        Self { price, orders: VecDeque::new() }
    }

    pub fn add_orders(&mut self, order: Order) {
        self.orders.push_back(order);
    }

    pub fn pop_order(&mut self) -> Option<Order> {
        self.orders.pop_front()
    }
}

impl BookSide {
    pub fn new() -> Self {
        Self { levels: BTreeMap::new() }
    }

    pub fn insert(&mut self, order: Order) {
        let level = self.levels.entry(order.price).or_insert_with(|| PriceLevel::new(order.price));
        level.add_orders(order);
    }

    //Remove a whole price level if empty
    pub fn remove_level_if_empty(&mut self, price: u64) {
        if let Some(level) = self.levels.get(&price) {
            if level.orders.is_empty() {
                self.levels.remove(&price);
            }
        }
    }

    //Get best price(highest for bids, lowest for ask)
    pub fn best_price(&self, side: Side) -> Option<u64> {

        match side {
            Side::Bid => self.levels.keys().rev().next().cloned(),
            Side::Ask => self.levels.keys().next().cloned(),
        }


    }
}