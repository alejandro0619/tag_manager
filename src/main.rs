mod scanner;
mod tags;
use std::io;
use std::path::Path;
use tags::TagManager;


fn main() {
    println!("Tag Manager - Choose an option:");
    println!("1. Scan folder for images");
    println!("2. Add metadata to PNG file");
    println!("3. View PNG metadata");
    println!("4. Verify PNG metadata");
    
    let mut choice = String::new();
    io::stdin()
        .read_line(&mut choice)
        .expect("Failed to read input");

    match choice.trim() {
        "1" => scan_folder(),
        "2" => add_metadata(),
        "3" => read_metadata(),
        "4" => verify_metadata(),
        _ => println!("Invalid option"),
    }
}

// The other functions remain the same as in the original code


fn scan_folder() {
    println!("Enter the absolute folder path to scan for images:");

    let mut folder_path = String::new();
    io::stdin()
        .read_line(&mut folder_path)
        .expect("Failed to read input");

    let folder_path = folder_path.trim();
    let folder_path = Path::new(folder_path);

    if !folder_path.exists() || !folder_path.is_dir() {
        eprintln!("The provided path does not exist or is not a directory: {:?}", folder_path);
        return;
    }

    let absolute_path = folder_path.canonicalize().expect("Failed to get absolute path");

    if let Err(e) = scanner::scan_images(absolute_path.to_str().unwrap()) {
        eprintln!("Error scanning folder: {}", e);
    }
}

fn add_metadata() {
    println!("Enter the PNG file path (absolute or relative):");
    let mut file_path = String::new();
    io::stdin()
        .read_line(&mut file_path)
        .expect("Failed to read input");

    let file_path = Path::new(file_path.trim());
    
    // Convert to absolute path and verify it exists
    if !file_path.exists() {
        eprintln!("The provided file does not exist: {:?}", file_path);
        return;
    }

    let absolute_path = file_path.canonicalize()
        .expect("Failed to get absolute path");

    println!("Enter the metadata key:");
    let mut key = String::new();
    io::stdin()
        .read_line(&mut key)
        .expect("Failed to read input");

    println!("Enter the metadata value:");
    let mut value = String::new();
    io::stdin()
        .read_line(&mut value)
        .expect("Failed to read input");
    
    if let Err(e) = TagManager::add_png_metadata(&absolute_path, key.trim(), value.trim()) {
        eprintln!("Error adding metadata: {}", e);
    }
}

fn read_metadata() {
    println!("Enter the PNG file path (absolute or relative):");
    let mut file_path = String::new();
    io::stdin()
        .read_line(&mut file_path)
        .expect("Failed to read input");

    let file_path = Path::new(file_path.trim());
    
    // Convert to absolute path and verify it exists
    if !file_path.exists() {
        eprintln!("The provided file does not exist: {:?}", file_path);
        return;
    }

    let absolute_path = file_path
        .canonicalize()
        .expect("Failed to get absolute path");

    match TagManager::read_png_metadata(&absolute_path) {
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


fn verify_metadata() {
    println!("Enter the PNG file path (absolute or relative):");
    let mut file_path = String::new();
    io::stdin()
        .read_line(&mut file_path)
        .expect("Failed to read input");

    println!("Enter the metadata key to verify:");
    let mut key = String::new();
    io::stdin()
        .read_line(&mut key)
        .expect("Failed to read input");

    let file_path = Path::new(file_path.trim());
    
    // Convert to absolute path and verify it exists
    if !file_path.exists() {
        eprintln!("The provided file does not exist: {:?}", file_path);
        return;
    }

    let absolute_path = file_path
        .canonicalize()
        .expect("Failed to get absolute path");

    match TagManager::read_png_metadata(&absolute_path) {
        Ok(metadata) => {
            let key = key.trim();
            if let Some(value) = metadata.iter().find(|(k, _)| k == key).map(|(_, v)| v) {
                println!("Metadata found: Key: {}, Value: {}", key, value);
            } else {
                println!("Metadata key '{}' not found in the PNG file.", key);
            }
        }
        Err(e) => eprintln!("Error reading metadata: {}", e),
    }
}

