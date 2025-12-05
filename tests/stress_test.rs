use std::time::{Duration, SystemTime, UNIX_EPOCH};

use clob::{
    order::{Order, Side},
    order_book::OrderBook,
};
use futures_util::{SinkExt, StreamExt};
use serde_json::json;
use tokio::time::Instant;
use tokio_tungstenite::connect_async;

#[tokio::test]
pub async fn stress_test_over_ws() {
    let (mut socket, _) = connect_async("ws://localhost:8000")
        .await
        .expect("Failed to connect to websocket");

    log::info!("Connected to the server");

    println!("Starting stress test over websocket....");

    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();

    let duration = Duration::from_secs(1);
    let start = Instant::now();

    let mut total_requests = 0;

    for i in 0..1000000 {
        if start.elapsed() < duration {
            let msg = json!({
                "id": i,
                "user_id": 1,
                "side": "Bid",
                "price": 100,
                "quantity": 1,
                "timestamp": timestamp
            })
            .to_string();

            let single_start = Instant::now();

            socket.send(msg.into()).await.unwrap();

            let _resp = socket.next().await.unwrap();

            let rtt = single_start.elapsed();

            println!("RTT over ws: {:?}", rtt);

            total_requests += 1;
        }
    }

    println!("Total requests over ws: {}", total_requests);
}

#[tokio::test]
pub async fn stress_test() {
    println!("Starting native stress test....");

    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();

    let duration = Duration::from_secs(1);
    let start = Instant::now();

    let mut total_requests = 0;

    let mut order_book = OrderBook::new();

    for i in 0..10000000 {
        if start.elapsed() < duration {
            let single_start = Instant::now();
            let side = {
                if total_requests & 1 != 0 {
                    Side::Bid
                } else {
                    Side::Ask
                }
            };

            order_book.submit_limit_order(Order {
                id: i,
                user_id: 1,
                side,
                price: 100,
                quantity: 10,
                timestamp,
            });

            let rtt = single_start.elapsed();

            println!("Native RTT: {:?}", rtt);

            total_requests += 1;
        }
    }

    println!("Total native requests: {}", total_requests);
}
