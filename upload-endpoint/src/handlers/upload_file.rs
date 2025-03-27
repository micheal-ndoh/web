use axum::{extract::Multipart, response::IntoResponse};
use std::{
    fs::File,
    io::{self, Write},
    path::PathBuf,
    time::{SystemTime, UNIX_EPOCH},
};

/// Saves the uploaded file to the `uploads` directory
async fn save_file(file_name: String, data: &[u8]) -> io::Result<()> {
    let save_path = PathBuf::from("uploads").join(file_name);
    let mut file = File::create(save_path)?; // Create a new file in the uploads directory
    file.write_all(data) // Write the received data into the file
}

/// Handles file uploads from a multipart form request
pub async fn upload_files(mut multipart: Multipart) -> impl IntoResponse {
    let mut uploaded_file_urls = vec![];
    while let Ok(Some(field)) = multipart.next_field().await {
        // Extract file name and content if available
        let file_name = match extract_filename(&field) {
            Ok(value) => value,
            Err(value) => return value,
        };

        match field.bytes().await {
            Err(_) => return format!("Failed to read chunk!"),
            Ok(data) => {
                // Attempt to save the file
                if let Err(e) = save_file(file_name.to_owned(), &data).await {
                    return format!("Failed to save file: {}", e);
                }
            }
        };

        uploaded_file_urls.push(format!("http://localhost:3000/uploader/files/{file_name}"));
    }

    String::from(format!(
        r#"{{"status": 201,"message": "File uploaded successfully.","files":{:?}}}"#,
        uploaded_file_urls
    ))
}

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
