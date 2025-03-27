mod handlers;
use axum::{
    extract::{Path, Query},
    response::IntoResponse,
    routing::{get, post},
    Router,
};
use handlers::{compress_file, upload_file};
use serde::Deserialize;
use tokio::net::TcpListener;
use tower_http::{services::ServeDir, trace::TraceLayer};

#[tokio::main]
async fn main() {
    let compressed = Router::new()
        .route("/compress", post(compress_file::compress_all_files))
        .nest_service("/files", ServeDir::new("compressed"));

    let uploads = Router::new()
        .route("/upload", post(upload_file::upload_files))
        .nest_service("/files", ServeDir::new("uploads"));

    // Create a new Axum router
    let app = Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .route("/path-examples/:parameter", get(path_example_handler))
        .nest("/uploader", uploads)
        .nest("/compressor", compressed)
        .fallback(|| async { r#"{"status":404,"message":"Resource Not Found"}"# })
        .layer(TraceLayer::new_for_http());

    // Define the address for the server to listen on
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
