use reqwest::multipart;
use std::path::PathBuf;
use structopt::StructOpt;
use tokio::fs::File;
use tokio::io::AsyncReadExt;

#[derive(Debug, StructOpt)]
pub struct Opt {
    #[structopt(parse(from_os_str))]
    infile: PathBuf,

    #[structopt(short, long, parse(from_os_str))]
    outfile: Option<PathBuf>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let opts = Opt::from_args();

    let mut file = File::open(&opts.infile).await?;
    let mut file_data = Vec::new();
    file.read_to_end(&mut file_data).await?;

    let file_name = opts
        .infile
        .file_name()
        .unwrap()
        .to_string_lossy()
        .to_string();

    let form = multipart::Form::new().part(
        "fileupload",
        multipart::Part::bytes(file_data).file_name(file_name),
    );

    let client = reqwest::Client::new();
    let response = match client
        .post("http://localhost:3000/index")
        .multipart(form)
        .send()
        .await
    {
        Ok(response) => response,
        Err(e) => {
            eprintln!("Failed to connect to server: {}", e);
            return Ok(());
        }
    };

    if response.status().is_success() {
        println!("File uploaded successfully!");
    } else {
        println!("Failed to upload file: {}", response.status());
    }

    Ok(())
}

// use structopt::StructOpt;
// use std::path::{PathBuf, Path};
// use std::fs::{self, File};
// use std::io::{Read, Write};

// #[derive(Debug, StructOpt)]
// struct Opts {
//     #[structopt(parse(from_os_str))]
//     file: PathBuf,
// }

// fn main() -> Result<(), Box<dyn std::error::Error>> {
//     let opts = Opts::from_args();

//     let mut file = File::open(&opts.file)?;
//     let mut contents = Vec::new();
//     file.read_to_end(&mut contents)?;

//     let destination_folder = Path::new("./uploads");
//     fs::create_dir_all(destination_folder)?;

//     let destination_file = destination_folder.join(opts.file.file_name().unwrap());

//     let mut dest_file = File::create(destination_file)?;
//     dest_file.write_all(&contents)?;

//     println!("File uploaded successfully.");

//     Ok(())
// }
