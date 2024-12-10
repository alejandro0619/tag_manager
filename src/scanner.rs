use std::fs;

pub fn scan_images(folder_path: &str) -> std::io::Result<()> {
    let allowed_extensions = ["jpg", "png"];
    let entries = fs::read_dir(folder_path)?;
    for entry in entries {
        let entry = entry?;
        let path = entry.path();

        if path.is_file() {
            if let Some(extension) = path.extension() {
                if let Some(ext_str) = extension.to_str() {
                    if allowed_extensions.contains(&ext_str.to_lowercase().as_str()) {
                        println!("Processing image: {:?}", path.file_name());
                    }
                }
            }
        }
    }
    Ok(())
}
