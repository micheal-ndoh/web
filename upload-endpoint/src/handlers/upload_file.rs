use axum::{extract::Multipart, response::IntoResponse, Extension};
use sqlx::PgPool;
use std::{
    fs::File,
    io::{self, Write},
    path::PathBuf,
    time::{SystemTime, UNIX_EPOCH},
};
use dotenvy::dotenv;
use std::env;

/// Saves the uploaded file to the `uploads` directory
async fn save_file(file_name: &str, data: &[u8]) -> io::Result<()> {
    let save_path = PathBuf::from("uploads").join(file_name);
    let mut file = File::create(save_path)?; // Create a new file in the uploads directory
    file.write_all(data) // Write the received data into the file
}

/// Handles file uploads from a multipart form request
pub async fn upload_files(
    Extension(pool): Extension<PgPool>, // Database connection
    mut multipart: Multipart,
) -> impl IntoResponse {
    let mut uploaded_files = vec![];

    while let Ok(Some(field)) = multipart.next_field().await {
        let file_name = match extract_filename(&field) {
            Ok(value) => value,
            Err(value) => return value,
        };

        match field.bytes().await {
            Err(_) => return format!("Failed to read chunk!"),
            Ok(data) => {
                if let Err(e) = save_file(&file_name, &data).await {
                    return format!("Failed to save file: {}", e);
                }

    

                // Insert file into database
                let result = sqlx::query!(
                    "INSERT INTO compression_tasks (file_name, status) VALUES ($1, 'pending') RETURNING id, status",
                    file_name
                )
                .fetch_one(&pool)
                .await;

                match result {
                    Ok(record) => {
                        uploaded_files.push(format!(
                            "{{\"id\": {}, \"file\": \"{}\", \"status\": \"{}\"}}",
                            record.id, file_name, record.status
                        ));
                    }
                    Err(e) => {
                        return format!("Failed to insert file into database: {}", e);
                    }
                }
            }
        };
    }

    format!(
        r#"{{"status": 201, "message": "Files uploaded successfully.", "files": [{}]}}"#,
        uploaded_files.join(", ")
    )
}

/// Generates a unique filename to prevent overwriting
fn extract_filename(field: &axum::extract::multipart::Field<'_>) -> Result<String, String> {
    let mut file_name: String = String::from(field.file_name().unwrap_or("unnamed"))
        .split(&[' ', '-', ':', '\''])
        .collect();

    let now = SystemTime::now();
    match now.duration_since(UNIX_EPOCH) {
        Ok(duration) => file_name.insert_str(0, &format!("{}_", duration.as_secs())),
        Err(_) => return Err(format!("Time went backward!")),
    };

    Ok(file_name)
}
