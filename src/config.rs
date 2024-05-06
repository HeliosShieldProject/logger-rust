use dotenvy::dotenv;
use once_cell::sync::Lazy;
use std::env;

#[derive(Debug, Clone)]
pub struct Config {
    pub logger_uri: String,
    pub mongo_url: String,
}

pub static ENV: Lazy<Config> = Lazy::new(|| {
    dotenv().ok();
    Config {
        logger_uri: env::var("LOGGER_URI").expect("LOGGER_URI must be set"),
        mongo_url: env::var("MONGO_URL").expect("MONGO_URL must be set"),
    }
});
