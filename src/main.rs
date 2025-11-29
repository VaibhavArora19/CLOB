use crate::{order::Order, order_book::OrderBook};

mod config;
mod order;
mod websocket;
mod price_level;
mod order_book;

async fn create_ws_connection(port: String) {
    if let Ok(server) = websocket::create_connection(port).await {
        while let Ok((stream, address)) = server.accept().await {
            //do something here
        }
    } else {
        //shutdown here
    }
}

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    tracing_subscriber::fmt::init();
    
    //fill the orders here
    //example -
    // let mut book = OrderBook::new();
    // let order = Order { id: 1, user_id: 1, price: 100, quantity: 1, timestamp: 1, side: order::Side::Bid};
    // book.submit_limit_order(order);
}
