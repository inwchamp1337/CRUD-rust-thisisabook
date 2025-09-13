use dotenv::dotenv;
use std::env;
use std::error::Error;

pub struct Config {
    pub mongodb_uri: String,
    pub mongodb_db: String
}

impl Config {
    pub fn from_env() -> Result<Self, Box<dyn Error>> {
        dotenv().ok();
        let mongodb_uri = env::var("MONGODB_URI").map_err(|_| "MONGODB_URI not set")?;
        let mongodb_db = env::var("MONGODB_DB").map_err(|_| "MONGODB_DB not set")?;
        Ok(Self { mongodb_uri, mongodb_db })
    }
}