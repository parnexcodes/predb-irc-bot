mod irc_client;
mod database;
mod message_parser;
mod utils;

use dotenvy::dotenv;
use sqlx::PgPool;
use std::env;
use log::info;
use env_logger::init;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    init(); // Use the imported function here

    info!("Starting application");

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = PgPool::connect(&database_url).await?;

    // Initialize the database
    database::init_db(&pool).await?;

    irc_client::run_irc_client(pool).await?;

    Ok(())
}
