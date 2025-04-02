// upload-endpoint/src/db.rs
use dotenv::dotenv;
use sqlx::{postgres::PgPoolOptions, PgPool};
use std::env;

pub async fn establish_connection() -> Result<PgPool, String> {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .map_err(|_| "DATABASE_URL must be set in .env file".to_string())?;

    println!("🛠️  RAW DATABASE_URL: {}", database_url);

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .map_err(|e| format!("Connection failed with URL '{}': {}", database_url, e))?;

    println!("✅ Connection pool created successfully");

    Ok(pool)
}
