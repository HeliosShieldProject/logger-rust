use dotenvy::dotenv;
use once_cell::sync::Lazy;
use std::env;

#[derive(Debug, Clone)]
pub struct Config {
    pub logger_port: u16,
    pub mongo_url: String,
}

pub static ENV: Lazy<Config> = Lazy::new(|| {
    dotenv().ok();
    Config {
        logger_port: env::var("LOGGER_PORT")
            .expect("LOGGER_PORT must be set")
            .parse()
            .expect("LOGGER_PORT must be a number"),
        mongo_url: env::var("MONGO_URL").expect("MONGO_URL must be set"),
    }
});
