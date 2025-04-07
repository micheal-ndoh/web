mod db;
mod handlers;
mod openapi;

use axum::{
    extract::{Path, Query},
    response::IntoResponse,
    routing::{get, post},
    Extension, Router,
};
use db::establish_connection;
use handlers::{check, compress_file, upload_file};
use openapi::ApiDoc;
use serde::Deserialize;
use std::net::SocketAddr;
use tokio::net::TcpListener;
use tower_http::{services::ServeDir, trace::TraceLayer};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();

    // Initialize database connection
    let pool = match establish_connection().await {
        Ok(pool) => pool,
        Err(e) => {
            eprintln!("Failed to connect to database: {}", e);
            return;
        }
    };

    // Compression service routes
    let compressor = Router::new()
        .route("/compress", post(compress_file::compress_all_files))
        .nest_service("/files", ServeDir::new("compressed"))
        .layer(Extension(pool.clone()));

    // Upload service routes
    let uploads = Router::new()
        .route("/upload", post(upload_file::upload_files))
        .nest_service("/files", ServeDir::new("uploads"))
        .layer(Extension(pool.clone()));

    let status_check: Router = Router::new()
        .route("/{task_id}", get(check::check_status))
        .layer(Extension(pool.clone()));

    // Main API router
    let app = Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .route("/path-examples/{parameter}", get(path_example_handler))
        .nest("/uploader", uploads)
        .nest("/compressor", compressor)
        .nest("/check", status_check)
        .merge(SwaggerUi::new("/swagger-ui").url("/api-doc/openapi.json", ApiDoc::openapi()))
        .fallback(|| async { r#"{"status":404,"message":"Resource Not Found"}"# })
        .layer(TraceLayer::new_for_http())
        .layer(Extension(pool));

    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    println!("Server running on http://{}", addr);

    // Start server and await it
    let listener = TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

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
