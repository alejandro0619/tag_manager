use clap::{Parser, Subcommand};
use std::path::Path;
use tags::TagManager;

mod scanner;
mod tags;
mod config;

use config::Config;

/// Tag Manager CLI Application
#[derive(Parser)]
#[command(name = "Tag Manager")]
#[command(author = "<alejandrolpz0619@gmail.com>")]
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
        folder_path: Option<String>,
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
        folder_path: Option<String>,
    },
    /// Configure Default folder for the images, otherwise type them in each command
    SetDefaultFolder {
        /// Set the default folder, if not provided, you will need to type the folder in each command
        folder_path: String,
    },
    ViewDefaultFolder,
}

fn main() {
    let mut config = Config::load();
    let cli = CLI::parse();

    match &cli.command {
        Commands::ScanFolder { folder_path } => {
            let folder = folder_path.clone().or_else(|| config.get_default_folder());
            if let Some(folder) = folder {
                scan_folder(&folder);
            } else {
                eprintln!("No folder provided and no default folder set.");
            }
        }
        Commands::AddMetadata {
            file_path,
            key,
            value,
        } => add_metadata(file_path, key, value),
        Commands::ViewMetadata { file_path } => view_metadata(file_path),
        Commands::VerifyMetadata { file_path, key } => verify_metadata(file_path, key),
        Commands::ScanFolderTags { folder_path } => {
            let folder = folder_path.clone().or_else(|| config.get_default_folder());
            if let Some(folder) = folder {
                scan_folder_with_tags(&folder);
            } else {
                eprintln!("No folder provided and no default folder set.");
            }
        }
        Commands::SetDefaultFolder { folder_path } => {
            config.set_default_folder(folder_path.clone());
        },
        Commands::ViewDefaultFolder => {
            if let Some(folder) = config.get_default_folder() {
                println!("Default folder: {}", folder);
            } else {
                println!("No default folder set.");
            }
        }
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

fn add_metadata(file_name_or_path: &str, key: &str, value: &str) {
    let config = config::Config::load();
    let mut file_path = None;
    println!("Adding metadata to file: {}", file_name_or_path);
    // Step 1: Check in default folder
    if let Some(default_folder) = config.get_default_folder() {
        let potential_path = Path::new(&default_folder).join(file_name_or_path);
        if potential_path.exists() && potential_path.is_file() {
            file_path = Some(potential_path);
        }
    }

    // Step 2: Check if it's a valid file path
    if file_path.is_none() {
        let provided_path = Path::new(file_name_or_path);
        if provided_path.exists() && provided_path.is_file() {
            file_path = Some(provided_path.to_path_buf());
        }
    }

    // Step 3: Handle cases where file is not found
    if let Some(file_path) = file_path {
        if let Err(e) = TagManager::add_png_metadata(&file_path, key, value) {
            eprintln!("Error adding metadata: {}", e);
        } else {
            println!(
                "Metadata added successfully to file: {}",
                file_path.display()
            );
        }
    } else {
        eprintln!(
            "File '{}' couldn't be found in the default folder or as a file path.",
            file_name_or_path
        );
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
