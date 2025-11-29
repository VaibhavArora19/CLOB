use futures_util::stream::StreamExt;
use anyhow::Error;
use tokio::net::{TcpListener, TcpStream};
use tokio_tungstenite::{accept_async, tungstenite::Message};

use crate::order::Order;

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

pub async fn handle_request(raw_stream: TcpStream) {
    if let Ok(stream) = accept_async(raw_stream).await.map_err(|e| {
        log::error!("Websocket handshake error. Failed with error: {:?}", e);
        Error::msg("Error during websocket handshake")
    }) {

        let (outgoing, mut incoming) = stream.split();

        while let Some(broadcasted_message) = incoming.next().await {
            if let Ok(message) = broadcasted_message.map_err(|e| {
                log::error!("Failed to get broadcasted message. Failed with error: {:?}", e);

                Error::msg("Failed to get broadcasted message")
            }) {
                match message {
                        Message::Text(text) => {
                            let text = text.as_str();
                            
                            if let Ok(order) =  serde_json::from_str::<Order>(text) {
                                //insert order into DB
                            }
                        }
                        _ => {/*add some logs here */}
                }
            }
        }
    } else {
        //error message here
    }
    
}
