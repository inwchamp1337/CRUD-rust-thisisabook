mod config;
mod db;
mod handlers;
mod models;
mod routes;

use config::Config;
use routes::create_router;
use tracing_subscriber;
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let cfg = match Config::from_env() {
        Ok(c) => c,
        Err(e) => {
            eprintln!("Config error: {}", e);
            std::process::exit(1);
        }
    };

    let db = match db::init_db(&cfg).await {
        Ok(d) => d,
        Err(e) => {
            eprintln!("Failed to connect to MongoDB: {}", e);
            std::process::exit(1);
        }
    };

    let app = create_router(db);

    // อ่านพอร์ตจาก env PORT หรือใช้ default 3000
    let port: u16 = std::env::var("PORT")
        .ok()
        .and_then(|s| s.parse::<u16>().ok())
        .unwrap_or(3000);

    let addr = SocketAddr::from(([127, 0, 0, 1], port));
    println!("Server running on http://{}", addr);

    // ใช้ tokio TcpListener และ axum::serve
    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .expect("Failed to bind TCP listener");
    axum::serve(listener, app).await.unwrap();
}