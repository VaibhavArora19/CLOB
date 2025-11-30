use crate::{
    order::{Order, OrderId, Side},
    price_level::BookSide,
};

#[derive(Debug, Clone)]
pub struct OrderBook {
    bids: BookSide,
    asks: BookSide,
    next_order_id: OrderId,
}

impl OrderBook {
    pub fn new() -> Self {
        Self {
            bids: BookSide::new(),
            asks: BookSide::new(),
            next_order_id: 1,
        }
    }

    //submit a limit order and return the remaining quantity if any
    pub fn submit_limit_order(&mut self, mut order: Order) -> u64 {
        let (own_side, other_side) = match order.side {
            Side::Bid => (&mut self.bids, &mut self.asks),
            Side::Ask => (&mut self.asks, &mut self.bids),
        };

        while order.quantity > 0 {
            if let Some(best_price) = other_side.best_price(match order.side {
                Side::Ask => Side::Ask,
                Side::Bid => Side::Bid,
            }) {
                let should_match = match order.side {
                    Side::Bid => order.price >= best_price,
                    Side::Ask => order.price <= best_price,
                };

                if !should_match {
                    break;
                }

                if let Some(level) = other_side.levels.get_mut(&best_price) {
                    while let Some(mut resting) = level.orders.pop_front() {
                        let traded = resting.quantity.min(order.quantity);
                        resting.quantity -= traded;
                        order.quantity -= traded;

                        //Notify trade events here
                        if resting.quantity > 0 {
                            level.orders.push_front(resting);
                            break;
                        }

                        if order.quantity == 0 {
                            break;
                        }
                    }

                    other_side.remove_level_if_empty(best_price);
                }
            } else {
                break;
            }
        }

        if order.quantity > 0 {
            own_side.insert(order);
        }

        order.quantity
    }
}
