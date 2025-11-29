pub trait ConfigTrait {
    fn get_config() -> Self;
}

pub struct ClobConfig {
    port: String,
}

#[derive(thiserror::Error, Debug)]
enum ConfigError {
    #[error("Invalid Port")]
    InvalidPort,
}

impl ConfigTrait for ClobConfig {
    fn get_config() -> Self {
        let port = std::env::var("PORT").unwrap_or_else(|_| ConfigError::InvalidPort.to_string());

        Self { port }
    }
}
