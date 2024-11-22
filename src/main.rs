mod config;
mod controllers;
mod models;
mod routes;
mod db;

use routes::user_routes;
use actix_web::{App, HttpServer, web};
use config::Config;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let config = Config::from_env()
        .expect("Failed to load configuration");
    
    // สร้าง connection pool
    let pool = db::create_pool(&config.database_url)
        .await
        .expect("Failed to create database pool");
    
    println!("Server running at http://{}", config.server_address());

    HttpServer::new(move || {
        App::new()
        .app_data(web::Data::new(pool.clone())) // แชร์ pool ให้กับ handlers
            .configure(user_routes::config)
    })
    .bind(config.server_address())?
    .run()
    .await
}