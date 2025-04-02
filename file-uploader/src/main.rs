use pbr::ProgressBar;
use reqwest::multipart;
use reqwest::Client;
use std::{
    env,
    fs::File,
    io::{self, Read},
    path::Path,
    time::Duration,
};

const FILE_FIELD: &str = "file";
const COMPRESSION_LEVEL_FIELD: &str = "compression_level";

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
    compression_level: Option<u32>,
) -> Result<String, reqwest::Error> {
    let part = multipart::Part::bytes(file_data)
        .file_name(file_name.clone())
        .mime_str("application/octet-stream")?;

    let mut form = multipart::Form::new().part(FILE_FIELD, part);
    
    if let Some(level) = compression_level {
        form = form.text(COMPRESSION_LEVEL_FIELD, level.to_string());
    }

    let response = client.post(url).multipart(form).send().await?;
    response.text().await
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!(
            "Usage: {} [--compression-level <1-9>] <file_path1> <file_path2> ...",
            args[0]
        );
        return Ok(());
    }

    let (compression_level, files) = if args[1] == "--compression-level" && args.len() > 3 {
        match args[2].parse::<u32>() {
            Ok(level @ 1..=9) => (Some(level), &args[3..]),
            Ok(_) => {
                eprintln!("Compression level must be between 1 and 9");
                return Ok(());
            }
            Err(_) => {
                eprintln!("Invalid compression level");
                return Ok(());
            }
        }
    } else {
        (None, &args[1..])
    };

    let client = Client::new();
    let url = "http://localhost:3000/uploader/upload";

    println!("Uploading {} files...", files.len());
    let mut pb = ProgressBar::new(files.len() as u64);
    pb.format("╢▌▌░╟");

    let mut success_count = 0;
    let mut failed_files = Vec::new();

    for (index, file_path) in files.iter().enumerate() {
        pb.set(1 + index as u64);
        println!("\nProcessing: {}", file_path);

        match read_file(file_path).await {
            Ok((file_name, file_data)) => {
                match upload_file(&client, url, file_name, file_data, compression_level).await {
                    Ok(response) => {
                        println!("Server response: {}", response);
                        success_count += 1;
                    }
                    Err(e) => {
                        eprintln!("Upload failed: {}", e);
                        failed_files.push(file_path.to_string());
                    }
                }
            }
            Err(e) => {
                eprintln!("Error reading file: {}", e);
                failed_files.push(file_path.to_string());
            }
        }

        tokio::time::sleep(Duration::from_millis(100)).await;
    }

    pb.finish();
    
    if failed_files.is_empty() {
        println!("✅ All files uploaded successfully!");
        if let Some(level) = compression_level {
            println!("📦 Compression level: {}", level);
        }
    } else {
        println!("⚠️  Upload completed with {} success(es) and {} failure(s)", 
                success_count, 
                failed_files.len());
        println!("Failed files:");
        for file in failed_files {
            println!("- {}", file);
        }
    }

    Ok(())
}