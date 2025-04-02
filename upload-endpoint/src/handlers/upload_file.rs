use axum::{
    extract::{Extension, Multipart},
    http::StatusCode,
    response::IntoResponse,
};
use dotenv::{dotenv, var};
use sqlx::Row;
use sqlx::{postgres::PgQueryResult, PgPool};
use std::{
    fs::{self, File},
    io::{self, Write},
    path::PathBuf,
    time::{SystemTime, UNIX_EPOCH},
};
pub async fn upload_files(
    mut multipart: Multipart,
    // Extension(pool): Extension<PgPool>,
) -> impl IntoResponse {
    dotenv().ok();
    let url = var("DATABASE_URL").unwrap();
    let pool = PgPool::connect(&url).await.unwrap();
    let mut uploaded_files = Vec::new();
    let mut errors = Vec::new();

    while let Ok(Some(field)) = multipart.next_field().await {
        let file_name = match field.file_name() {
            Some(name) => {
                let timestamp = SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .unwrap()
                    .as_secs();
                format!("{}_{}", timestamp, name.replace(" ", "_"))
            }
            None => {
                errors.push("Skipped field without filename".to_string());
                continue;
            }
        };

        match field.bytes().await {
            Ok(data) => {
                // 1. Save file to disk
                let save_path = PathBuf::from("uploads").join(&file_name);
                if let Err(e) = fs::create_dir_all("uploads")
                    .and_then(|_| File::create(&save_path))
                    .and_then(|mut file| file.write_all(&data))
                {
                    errors.push(format!("Failed to save {}: {}", file_name, e));
                    continue;
                }

                // 2. Register in database
                let row = match sqlx::query(
                    "
                    INSERT INTO compression_tasks (file_name, status)
                    VALUES ($1, 'pending')
                    RETURNING id
                    ",
                )
                .bind(&file_name)
                .fetch_one(&pool)
                .await
                {
                    Ok(record) => {
                        let id: i32 = record.get("id");
                        uploaded_files.push(format!("{} (ID: {})", file_name, id));
                    }
                    Err(e) => {
                        errors.push(format!("Failed to register {}: {}", file_name, e));
                        // Clean up the file if DB registration failed
                        let _ = fs::remove_file(save_path);
                    }
                };
            }
            Err(e) => {
                errors.push(format!("Failed to read file data: {}", e));
            }
        }
    }

    // Prepare response
    let mut response = String::new();
    if !uploaded_files.is_empty() {
        response.push_str(&format!(
            "Successfully uploaded:\n{}\n",
            uploaded_files.join("\n")
        ));
    }
    if !errors.is_empty() {
        response.push_str(&format!("\nErrors occurred:\n{}", errors.join("\n")));
    }

    if uploaded_files.is_empty() && errors.is_empty() {
        (
            StatusCode::BAD_REQUEST,
            "No files were uploaded".to_string(),
        )
    } else if uploaded_files.is_empty() {
        (StatusCode::PARTIAL_CONTENT, response)
    } else {
        (StatusCode::OK, response)
    }
}
