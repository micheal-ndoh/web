use axum::http::StatusCode;
use axum::{extract::Extension, response::IntoResponse};
use sqlx::{FromRow, PgPool};
use std::{fs, io::Write, path::Path};
use tokio::task;

// Add this struct to represent the query results
#[derive(FromRow)]
struct CompressionTask {
    id: i32,
    file_name: String,
}

use utoipa::ToSchema;

#[derive(ToSchema)]
pub struct CompressionResponse {
    pub message: String,
    pub file_count: usize,
}

#[utoipa::path(
    post,
    path = "/compressor/compress",
    responses(
        (status = 200, description = "Compression started", body = CompressionResponse),
        (status = 500, description = "Internal server error")
    ),
    tag = "file-service"
)]

pub async fn compress_all_files(Extension(pool): Extension<PgPool>) -> impl IntoResponse {
    // Ensure compressed directory exists
    if let Err(e) = fs::create_dir_all("compressed") {
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Failed to create compressed directory: {}", e),
        );
    }

    // Get all pending files from database
    let pending_files = match sqlx::query_as::<_, CompressionTask>(
        "SELECT id, file_name FROM compression_tasks WHERE status = 'pending'",
    )
    .fetch_all(&pool)
    .await
    {
        Ok(files) => files,
        Err(e) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Database error: {}", e),
            );
        }
    };

    let file_count = pending_files.len();

    if file_count == 0 {
        return (StatusCode::OK, "No files to compress".to_string());
    }

    // Process each file in background
    for task in pending_files {
        let pool = pool.clone();
        let input_path = format!("uploads/{}", task.file_name);
        let output_path = format!("compressed/{}.gz", task.file_name);

        task::spawn(async move {
            // Update status to processing
            let _ = sqlx::query("UPDATE compression_tasks SET status = 'processing' WHERE id = $1")
                .bind(task.id)
                .execute(&pool)
                .await;

            // Perform compression
            let result = compress_file(&input_path, &output_path);

            // Update status based on result
            let status = if result.is_ok() {
                "completed"
            } else {
                "failed"
            };

            let _ = sqlx::query("UPDATE compression_tasks SET status = $1 WHERE id = $2")
                .bind(status)
                .bind(task.id)
                .execute(&pool)
                .await;
        });
    }

    (
        StatusCode::OK,
        format!("Started compressing {} files in background", file_count),
    )
}

fn compress_file(input_path: &str, output_path: &str) -> std::io::Result<()> {
    // Read input file
    let mut input = std::fs::File::open(input_path)?;
    let mut buffer = Vec::new();
    std::io::Read::read_to_end(&mut input, &mut buffer)?;

    // Create output file and compress
    let output = std::fs::File::create(output_path)?;
    let mut encoder = flate2::write::GzEncoder::new(output, flate2::Compression::default());
    encoder.write_all(&buffer)?;
    encoder.finish()?;

    Ok(())
}
