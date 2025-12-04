use anyhow::Error;
use clob::{
    config::{ClobConfig, ConfigTrait}, db::{Tables, connect_db, get_all, insert}, order::Order, order_book::OrderBook, websocket
};
use tokio::{signal::{self}, sync::mpsc::{self, UnboundedSender}};

use crate::websocket::handle_request;


async fn create_ws_connection(port: String, order_book: OrderBook, db_tx: UnboundedSender<Order>) {
    log::info!("Connecting to websocket at port: {:?}", port);
    
    if let Ok(server) = websocket::create_connection(port).await {
        while let Ok((stream, _)) = server.accept().await {
            handle_request(stream, order_book.clone(), db_tx.clone()).await;
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

    let (db_tx,mut db_rx) = mpsc::unbounded_channel::<Order>();

    let order_book = OrderBook::new();

    //in here load them into memory after getting bids and asks side and then getting levels and everything
    get_all(db.clone()).await;

    tokio::spawn(async move {
        let db_clone = db.clone();
        while let Some(unwrapped_data) = db_rx.recv().await {
            insert(db_clone.clone(), unwrapped_data.id, unwrapped_data.price, serde_json::to_string(&unwrapped_data).unwrap()).await;
        }

        log::info!("DB worker: receiver closed, exiting.");
    });

    //spawn a new thread that will push the new orders to the DB
    // rw_txn.put("table", "price", "data", flags);

    //fill the orders here
    //example -
    // let mut book = OrderBook::new();
    // let order = Order { id: 1, user_id: 1, price: 100, quantity: 1, timestamp: 1, side: order::Side::Bid};
    // book.submit_limit_order(order);


    tokio::select! {
        _ = create_ws_connection(config.port, order_book, db_tx) => {}
        _ = signal::ctrl_c() => {
            log::info!("Shutdown singal received. Stopping...");
        }
    };

    Ok(())

}
