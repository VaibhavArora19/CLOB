mod config;
mod websocket;
mod order;

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
}
