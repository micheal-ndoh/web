// upload-endpoint/src/db.rs
use sqlx::{PgPool, postgres::PgPoolOptions};
use dotenvy::dotenv;

pub async fn establish_connection() -> Result<PgPool, String> {
    dotenv().ok();
    dotenvy::from_path("upload-endpoint/.env").ok();
   
      // Fallback URL if .env loading fails
      let database_url = std::env::var("DATABASE_URL")
      .unwrap_or_else(|_| "postgres://micheal:password@10.38.229.35:5432/compression_tasks?sslmode=disable".to_string());

    
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .map_err(|e| format!("Failed to connect to database: {}", e))?;
    
    sqlx::migrate!()
        .run(&pool)
        .await
        .map_err(|e| format!("Failed to run migrations: {}", e))?;
    
    Ok(pool)
}