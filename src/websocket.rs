use tokio::net::TcpListener;

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
