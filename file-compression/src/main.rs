use rust_file_compression::compress_file;
use std::path::Path;

fn main() {
    let input_path = Path::new("input.txt");
    let output_path = Path::new("output.gz");

    match compress_file(input_path, output_path) {
        Ok(()) => println!("File compressed successfully!"),
        Err(e) => eprintln!("Error compressing file: {}", e),
    }
}
