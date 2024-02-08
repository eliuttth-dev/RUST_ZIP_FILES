use std::fs::File;
use std::io::{self, Read, Write};
use zip::write::FileOptions;
use zip::CompressionMethod::Stored;
use zip::write::ZipWriter;

fn compress_file(input_path: &str, output_path: &str) -> io::Result<()> {
    let mut input_file = File::open(input_path)?;
    let mut zip_writer = ZipWriter::new(File::create(output_path)?);

    let options = FileOptions::default()
        .compression_method(Stored)
        .unix_permissions(0o755);

    zip_writer.start_file("compressed.txt", options)?;
    let mut buffer = Vec::new();
    input_file.read_to_end(&mut buffer)?;

    zip_writer.write_all(&buffer)?;

    zip_writer.finish()?;
    Ok(())
}

fn main() {
    let input_file_path = "text.txt";
    let output_zip_path = "output.zip";

    if let Err(e) = compress_file(input_file_path, output_zip_path) {
        eprintln!("Error compressing file: {:?}", e);
    } else {
        println!("File compressed successfully!");
    }
}

