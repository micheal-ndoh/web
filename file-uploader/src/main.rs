use pbr::ProgressBar;
use reqwest::multipart;
use reqwest::Client;
use std::thread;
use std::{
    env,
    fs::File,
    io::{self, Read},
    path::Path,
};

async fn read_file(file_path: &str) -> io::Result<(String, Vec<u8>)> {
    let file_name = Path::new(file_path)
        .file_name()
        .and_then(|name| name.to_str())
        .map(String::from)
        .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidInput, "Invalid file name"))?;

    let mut buffer = Vec::new();
    File::open(file_path)?.read_to_end(&mut buffer)?;

    Ok((file_name, buffer))
}

async fn upload_file(
    client: &Client,
    url: &str,
    file_name: String,
    file_data: Vec<u8>,
    compression: Option<&str>,
) -> Result<(), reqwest::Error> {
    let part = multipart::Part::bytes(file_data)
        .file_name(file_name.clone())
        .mime_str("application/octet-stream")
        .unwrap_or_else(|_| panic!("Failed to set MIME type")); // Should not fail in practice

    let mut form = multipart::Form::new().part("file", part);
    if let Some(compression) = compression {
        form = form.text("compression", compression.to_string());
    }

    let response = client.post(url).multipart(form).send().await?;

    if response.status().is_success() {
        println!("  File '{}' uploaded successfully", file_name);
    } else {
        eprintln!("Failed to upload file: {:?}", response.status());
    }

    Ok(())
}
#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        eprintln!(
            "Usage: {} --compression <type> <file_path1> <file_path2> ...",
            args[0]
        );
        return;
    }

    let compression = if args[1] == "--compression" {
        Some(&args[2])
    } else {
        eprintln!("Specify compression type using --compression");
        return;
    };

    let client = Client::new();
    let url = "http://localhost:3000/upload-files";

    // Initialize the progress bar
    let total_files = args.len() - 3;
    let mut pb = ProgressBar::new(total_files as u64);
    pb.format("╢▌▌░╟\n");

    for (index, file_path) in args[3..].iter().enumerate() {
        println!("\nUploading file: {}", file_path);

        pb.set(index as u64 + 1);

        // Read the file and upload
        match read_file(file_path).await {
            Ok((file_name, file_data)) => {
                if let Err(err) = upload_file(
                    &client,
                    url,
                    file_name,
                    file_data,
                    compression.map(|x| x.as_str()),
                )
                .await
                {
                    eprintln!("Error uploading file: {}", err);
                }
            }
            Err(err) => eprintln!("Error reading file: {}", err),
        }

        thread::sleep(std::time::Duration::from_millis(100));
    }

    pb.finish_print("\nAll files uploaded successfully!\n");
}
