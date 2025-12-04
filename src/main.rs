use anyhow::Error;
use clob::{
    config::{ClobConfig, ConfigTrait}, db::{Tables, connect_db}, order_book::OrderBook, websocket
};
use tokio::signal::{self};

use crate::websocket::handle_request;


async fn create_ws_connection(port: String, order_book: OrderBook) {
    log::info!("Connecting to websocket at port: {:?}", port);
    
    if let Ok(server) = websocket::create_connection(port).await {
        while let Ok((stream, _)) = server.accept().await {
            handle_request(stream, order_book.clone()).await;
        }
    } else {
        //shutdown here
    }
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    dotenv::dotenv().ok();
    tracing_subscriber::fmt::init();

    let config = ClobConfig::get_config();

    let db = connect_db::<Tables>()?;

    // rw_txn.put("table", "price", "data", flags);
    let order_book = OrderBook::new();

    //fill the orders here
    //example -
    // let mut book = OrderBook::new();
    // let order = Order { id: 1, user_id: 1, price: 100, quantity: 1, timestamp: 1, side: order::Side::Bid};
    // book.submit_limit_order(order);


    tokio::select! {
        _ = create_ws_connection(config.port, order_book) => {}
        _ = signal::ctrl_c() => {
            log::info!("Shutdown singal received. Stopping...");
        }
    };

    Ok(())

}
