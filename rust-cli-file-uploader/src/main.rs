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
            eprintln!("Failed to connect to the server: {}", e);
            return Ok(());
        }
    };

    if response.status().is_success() {
        println!("File was uploaded successfully!");
    }
    Ok(())
}
