use flate2::write::GzEncoder;
use flate2::Compression;
use std::fs::File;
use std::io::{self, Read, Write};
use std::path::Path;

pub fn compress_file(input_path: &Path, output_path: &Path) -> io::Result<()> {
    // Open the input file
    let mut input_file = File::open(input_path)?;

    let output_file = File::create(output_path)?;

    let mut encoder = GzEncoder::new(output_file, Compression::default());

    let mut buffer = Vec::new();
    input_file.read_to_end(&mut buffer)?;
    encoder.write_all(&buffer)?;

    encoder.finish()?;

    Ok(())
}
