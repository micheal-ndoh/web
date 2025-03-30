use axum::{extract::Extension, response::IntoResponse, http::StatusCode};
use sqlx::PgPool;
use std::fs;
use std::path::{Path, PathBuf};
use tokio::task;
use upload_endpoint::compress_file;

/// Compress files and update database status
pub async fn compress_all_files(Extension(pool): Extension<PgPool>) -> impl IntoResponse {
    let compressed_dir = Path::new("compressed");

    // Ensure compressed directory exists
    if let Err(e) = fs::create_dir_all(compressed_dir) {
        eprintln!("Failed to create compressed directory: {e}");
        return (StatusCode::INTERNAL_SERVER_ERROR, "Directory creation failed");
    }

    // Fetch all pending compression tasks
    let files = match sqlx::query!("SELECT id, file_name FROM compression_tasks WHERE status = 'pending'")
        .fetch_all(&pool)
        .await
    {
        Ok(files) => files,
        Err(e) => {
            eprintln!("Database error: {e}");
            return (StatusCode::INTERNAL_SERVER_ERROR, "Failed to fetch files");
        }
    };

    if files.is_empty() {
        return (StatusCode::OK, "No files to compress");
    }

    // Process each file asynchronously
    for file in files {
        let file_id = file.id;
        let input_path = PathBuf::from("uploads").join(&file.file_name);
        let output_path = compressed_dir.join(format!("{}.gz", file.file_name));

        // Clone the database pool for each task
        let pool_clone = pool.clone();

        // Spawn a background task
        task::spawn(async move {
            let compression_level = 6;

            let status = match compress_file(input_path.to_str().unwrap(), output_path.to_str().unwrap(), compression_level) {
                Ok(_) => "completed",
                Err(e) => {
                    eprintln!("Compression error for {}: {e}", file.file_name);
                    "failed"
                }
            };

            // Update the database with the compression result
            let _ = sqlx::query!("UPDATE compression_tasks SET status = $1 WHERE id = $2", status, file_id)
                .execute(&pool_clone)
                .await;
        });
    }

    (StatusCode::OK, "Compression started in background")
}
