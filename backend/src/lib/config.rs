use dotenv::dotenv;
use lazy_static::lazy_static;
use serde::Deserialize;

lazy_static! {
    pub static ref CONFIG: Config = {
        dotenv().ok();
        Config::new().unwrap()
    };
}

#[derive(Deserialize)]
pub struct Config {
    pub host: String,
    pub port: String,
    pub database_url: String,
    pub frontend_origin: String,
}

impl Config {
    fn new() -> Result<Self, config::ConfigError> {
        let environment = config::Environment::default().try_parsing(true);
        let config = config::Config::builder()
            .set_default("host", "127.0.0.1")?
            .set_default("port", "8080")?
            .set_default("frontend_origin", "http://127.0.0.1:5173")?
            .add_source(environment)
            .build()?;
        config.try_deserialize()
    }
}
