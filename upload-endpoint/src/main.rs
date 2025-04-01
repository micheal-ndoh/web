mod db;
mod handlers;

use axum::{
    extract::{Path, Query},
    response::IntoResponse,
    routing::{get, post},
    Extension, Router,
};
use db::establish_connection;
use handlers::{compress_file, upload_file};
use serde::Deserialize;
use tokio::net::TcpListener;
use tower_http::{services::ServeDir, trace::TraceLayer};

use dotenvy::dotenv;
// use std::env;

#[tokio::main]
async fn main() {
    dotenv().ok();

    // Initialize database connection
    let pool = establish_connection().await;

    // Compression service routes
    let compressed = Router::new()
        .route("/compress", post(compress_file::compress_all_files))
        .nest_service("/files", ServeDir::new("compressed"))
        .layer(Extension(pool.clone())); // Pass DB connection

    // Upload service routes
    let uploads = Router::new()
        .route("/upload", post(upload_file::upload_files))
        .nest_service("/files", ServeDir::new("uploads"))
        .layer(Extension(pool.clone())); // Pass DB connection

    // Main API router
    let app = Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .route("/path-examples/:parameter", get(path_example_handler))
        .nest("/uploader", uploads)
        .nest("/compressor", compressed)
        .fallback(|| async { r#"{"status":404,"message":"Resource Not Found"}"# })
        .layer(TraceLayer::new_for_http())
        .layer(Extension(pool)); // Make DB accessible to all routes

    // Define server address
    let ip_addr = "0.0.0.0:3000";
    let listener = match TcpListener::bind(ip_addr).await {
        Ok(listener) => listener,
        Err(e) => {
            eprintln!("Failed to bind TcpListener to server {e}");
            return;
        }
    };

    println!("[INFO]: Application is running http://{ip_addr}");

    // Start the server
    if let Err(e) = axum::serve(listener, app).await {
        eprintln!("{e}")
    }
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
struct MyQuery {
    gis: Option<bool>,
    number: usize,
}

async fn path_example_handler(
    Path(parameter): Path<String>,
    Query(query): Query<MyQuery>,
) -> impl IntoResponse {
    format!("The parameter value is {parameter} and query value is {query:?}")
}
