use flate2::write::GzEncoder;
use flate2::Compression;
use std::fs::File;
use std::io::{self, Read, Write};
use std::path::Path;



fn main() {
    let input_path = Path::new("README.md");
    let output_path = Path::new("README.md.gz");

    match compress_file(input_path, output_path) {
        Ok(()) => println!("File compressed successfully!"),
        Err(e) => eprintln!("Error compressing file: {}", e),
    }
}

pub fn compress_file(input_path: &Path, output_path: &Path) -> io::Result<()> {
    
    let mut input_file = File::open(input_path)?;

    let output_file = File::create(output_path)?;

    let mut encoder = GzEncoder::new(output_file, Compression::default());

    let mut buffer = Vec::new();
    input_file.read_to_end(&mut buffer)?;
    encoder.write_all(&buffer)?;

    encoder.finish()?;

    Ok(())
}
