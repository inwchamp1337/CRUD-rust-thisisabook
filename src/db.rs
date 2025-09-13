use crate::config::Config;
use mongodb::{options::ClientOptions, Client, Database};
use mongodb::bson::doc;
use std::error::Error;

pub async fn init_db(cfg: &Config) -> Result<Database, Box<dyn Error>> {
    let mut client_options = ClientOptions::parse(&cfg.mongodb_uri).await?;
    client_options.app_name = Some("dearthpirate".to_string());

    let client = Client::with_options(client_options)?;
    let db = client.database(&cfg.mongodb_db);

    // ping เพื่อตรวจสอบการเชื่อมต่อ
    db.run_command(doc! { "ping": 1 }).await?;

    println!("Connected to MongoDB at '{}' (db='{}')", cfg.mongodb_uri, cfg.mongodb_db);
    Ok(db)
}