use clap::{Parser, Subcommand};
use std::path::Path;
use tags::TagManager;

mod scanner;
mod tags;

/// Tag Manager CLI Application
#[derive(Parser)]
#[command(name = "Tag Manager")]
#[command(author = "Your Name <alejandrolpz0619@gmail.com>")]
#[command(version = "1.0")]
#[command(about = "Manage tags for image files", long_about = None)]
struct CLI {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Scan a folder for images
    ScanFolder {
        /// Folder path to scan for images
        folder_path: String,
    },
    /// Add metadata to a PNG file
    AddMetadata {
        /// Path to the PNG file
        file_path: String,
        /// Metadata key
        key: String,
        /// Metadata value
        value: String,
    },
    /// View metadata of a PNG file
    ViewMetadata {
        /// Path to the PNG file
        file_path: String,
    },
    /// Verify metadata of a PNG file
    VerifyMetadata {
        /// Path to the PNG file
        file_path: String,
        /// Metadata key to verify
        key: String,
    },
    /// Scan a folder for tags
    ScanFolderTags {
        /// Folder path to scan for images with tags
        folder_path: String,
    },
}

fn main() {
    let cli = CLI::parse();

    match &cli.command {
        Commands::ScanFolder { folder_path } => scan_folder(folder_path),
        Commands::AddMetadata {
            file_path,
            key,
            value,
        } => add_metadata(file_path, key, value),
        Commands::ViewMetadata { file_path } => view_metadata(file_path),
        Commands::VerifyMetadata { file_path, key } => verify_metadata(file_path, key),
        Commands::ScanFolderTags { folder_path } => scan_folder_with_tags(folder_path),
    }
}

fn scan_folder(folder_path: &str) {
    let path = Path::new(folder_path);

    if !path.exists() || !path.is_dir() {
        eprintln!("The provided path does not exist or is not a directory: {:?}", folder_path);
        return;
    }

    if let Err(e) = scanner::scan_images(folder_path) {
        eprintln!("Error scanning folder: {}", e);
    }
}

fn add_metadata(file_path: &str, key: &str, value: &str) {
    let path = Path::new(file_path);

    if !path.exists() {
        eprintln!("The provided file does not exist: {:?}", file_path);
        return;
    }

    if let Err(e) = TagManager::add_png_metadata(path, key, value) {
        eprintln!("Error adding metadata: {}", e);
    }
}

fn view_metadata(file_path: &str) {
    let path = Path::new(file_path);

    if !path.exists() {
        eprintln!("The provided file does not exist: {:?}", file_path);
        return;
    }

    match TagManager::read_png_metadata(path) {
        Ok(metadata) => {
            if metadata.is_empty() {
                println!("No metadata found in the PNG file.");
            } else {
                println!("Metadata found in the PNG file:");
                for (key, value) in metadata {
                    println!("Key: {}, Value: {}", key, value);
                }
            }
        }
        Err(e) => eprintln!("Error reading metadata: {}", e),
    }
}

fn verify_metadata(file_path: &str, key: &str) {
    let path = Path::new(file_path);

    if !path.exists() {
        eprintln!("The provided file does not exist: {:?}", file_path);
        return;
    }

    match TagManager::read_png_metadata(path) {
        Ok(metadata) => {
            if let Some(value) = metadata.iter().find(|(k, _)| k == key).map(|(_, v)| v) {
                println!("Metadata found: Key: {}, Value: {}", key, value);
            } else {
                println!("Metadata key '{}' not found in the PNG file.", key);
            }
        }
        Err(e) => eprintln!("Error reading metadata: {}", e),
    }
}

fn scan_folder_with_tags(folder_path: &str) {
    TagManager::scan_images_with_tags(folder_path);
}
