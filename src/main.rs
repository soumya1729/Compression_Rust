use std::fs::File;
use std::io::{self, Read, Write};
use std::path::Path;
use zip::write::FileOptions;
use zip::CompressionMethod;

/// Compresses a source file into a ZIP archive with advanced options.
///
/// # Arguments
///
/// * `source_file` - The path to the file to compress.
/// * `zip_file` - The path where the ZIP archive will be created.
/// * `compression_method` - The compression method to use (e.g., Deflated, Stored, Bzip2, Zstd).
fn compress_to_zip(
    source_file: &str,
    zip_file: &str,
    compression_method: CompressionMethod,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("Starting compression...");

    // Open the source file for reading.
    let mut source = File::open(source_file)?;

    // Read the file contents into a buffer.
    let mut buffer = Vec::new();
    source.read_to_end(&mut buffer)?;

    // Create the ZIP file.
    let zip_path = Path::new(zip_file);
    let zip_file_handle = File::create(&zip_path)?;

    // Initialize the ZIP writer.
    let mut zip = zip::ZipWriter::new(zip_file_handle);

    // File options for the ZIP entry.
    let options = FileOptions::default()
        .compression_method(compression_method) // Use the specified compression method.
        .unix_permissions(0o644); // Set file permissions.

    // Write the file into the ZIP archive.
    zip.start_file(source_file, options)?;
    zip.write_all(&buffer)?;

    // Finish the ZIP archive.
    zip.finish()?;

    println!("Compression completed successfully!");
    println!("File '{}' has been compressed to '{}'",
             source_file,
             zip_file
    );

    Ok(())
}

fn main() {
    // Take file paths and compression method from the user.
    let mut source_file = String::new();
    let mut zip_file = String::new();
    let mut compression_choice = String::new();

    println!("Welcome to the Advanced File Compressor!");

    println!("Enter the path of the source file to compress:");
    io::stdin().read_line(&mut source_file).expect("Failed to read source file path");
    let source_file = source_file.trim();

    println!("Enter the destination path for the ZIP file:");
    io::stdin().read_line(&mut zip_file).expect("Failed to read destination file path");
    let zip_file = zip_file.trim();

    println!("Choose a compression method:");
    println!("1) Deflated  2) Stored  3) Bzip2  4) Zstd");
    io::stdin().read_line(&mut compression_choice).expect("Failed to read compression choice");
    let compression_choice = compression_choice.trim();

    let compression_method = match compression_choice {
        "1" => CompressionMethod::Deflated,
        "2" => CompressionMethod::Stored,
        "3" => CompressionMethod::Bzip2,
        "4" => CompressionMethod::Zstd,
        _ => {
            eprintln!("Invalid choice, defaulting to Deflated.");
            CompressionMethod::Deflated
        }
    };

    match compress_to_zip(source_file, zip_file, compression_method) {
        Ok(_) => println!("Compression successful."),
        Err(e) => eprintln!("An error occurred: {}", e),
    }
}
