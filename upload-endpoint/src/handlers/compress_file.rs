use std::path::PathBuf;

use axum::response::IntoResponse;
use upload_endpoint::compress_file;

//// Endpoint to trigger file compression on demand
pub async fn compress_all_files() -> impl IntoResponse {
    let uploads_dir = PathBuf::from("uploads");
    let compressed_dir = PathBuf::from("compressed");

    // Ensure the compressed directory exists
    if let Err(e) = std::fs::create_dir_all(&compressed_dir) {
        return format!("Failed to create compressed directory: {}", e);
    }

    // Define the compression level (e.g., 6 for default compression)
    let compression_level = 6;

    // Iterate over files in the uploads directory
    if let Ok(entries) = uploads_dir.read_dir() {
        for entry in entries.flatten() {
            let input_path = entry.path();
            let output_path = compressed_dir.join(entry.file_name()).with_extension("gz");

            // Skip already compressed files
            if input_path.extension().map_or(false, |ext| ext == "gz") {
                continue;
            }

            // Compress the file and handle errors
            if let Err(e) = compress_file(
                input_path.to_str().unwrap(),
                output_path.to_str().unwrap(),
                compression_level,
            ) {
                return format!("Failed to compress file {}: {}", input_path.display(), e);
            }
        }
    }

    String::from("Files compressed successfully")
}