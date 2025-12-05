use anyhow::Error;
use futures_util::{SinkExt, stream::StreamExt};
use tokio::{
    net::{TcpListener, TcpStream},
    sync::mpsc::UnboundedSender,
};
use tokio_tungstenite::{accept_async, tungstenite::Message};

use crate::{order::Order, order_book::OrderBook};

pub async fn create_connection(port: String) -> Result<TcpListener, anyhow::Error> {
    let server = TcpListener::bind(format!("127.0.0.1:{}", port))
        .await
        .map_err(|e| {
            log::error!(
                "Error creating a TCP connection. Failed with error: {:?}",
                e
            );

            anyhow::Error::msg("Failed to create a TCP connection")
        })?;

    Ok(server)
}

pub async fn handle_request(
    raw_stream: TcpStream,
    mut order_book: OrderBook,
    db_tx: UnboundedSender<Order>,
) {
    if let Ok(stream) = accept_async(raw_stream).await.map_err(|e| {
        log::error!("Websocket handshake error. Failed with error: {:?}", e);
        Error::msg("Error during websocket handshake")
    }) {
        let (mut outgoing, mut incoming) = stream.split();

        while let Some(broadcasted_message) = incoming.next().await {
            if let Ok(message) = broadcasted_message.map_err(|e| {
                log::error!(
                    "Failed to get broadcasted message. Failed with error: {:?}",
                    e
                );

                Error::msg("Failed to get broadcasted message")
            }) {
                log::info!("Message: {:?}", message);
                match message {
                    Message::Text(text) => {
                        let text = text.as_str();

                        if let Ok(order) = serde_json::from_str::<Order>(text) {
                            //insert order into DB
                            let price = order_book.submit_limit_order(order);

                            if let Err(err) = outgoing
                                .send(Message::Text(serde_json::to_string(&price).unwrap().into()))
                                .await
                            {
                                log::info!("Failed to send response. Failed with error: {:?}", err);
                            }

                            if let Err(err) = db_tx.send(order) {
                                log::error!(
                                    "Failed to send order info to DB. Failed with error: {:?}",
                                    err
                                );
                            }
                        } else {
                            log::error!("Failed to convert message into order");
                        }
                    }
                    _ => { /*add some logs here */ }
                }
            }
        }
    } else {
        //error message here
    }
}
