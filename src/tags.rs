use png::Decoder;
use std::fs::{File, self};
use std::io::{BufReader, BufWriter};
use std::path::Path;

#[derive(Debug)]
enum ImageFormat {
    PNG,
    JPG,
}

pub struct TagManager {}

impl TagManager {
    fn get_image_format(path: &Path) -> Option<ImageFormat> {
        match path.extension()?.to_str()?.to_lowercase().as_str() {
            "png" => Some(ImageFormat::PNG),
            "jpg" | "jpeg" => Some(ImageFormat::JPG),
            _ => None,
        }
    }

    pub fn add_png_metadata(
        file_path: &Path,
        key: &str,
        value: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let file = File::open(file_path)?;
        let decoder = png::Decoder::new(BufReader::new(file));
        let mut reader = decoder.read_info()?;
        let mut buf = vec![0; reader.output_buffer_size()];
        let _info = reader.next_frame(&mut buf)?;

        let file_out = File::create(file_path)?;
        let writer = BufWriter::new(file_out);
        let mut encoder = png::Encoder::new(writer, reader.info().width, reader.info().height);
        encoder.set_color(reader.info().color_type);
        encoder.set_depth(reader.info().bit_depth);
        encoder.add_text_chunk(key.to_string(), value.to_string())?;

        let mut writer = encoder.write_header()?;
        writer.write_image_data(&buf)?;
        println!("Metadata added to PNG: '{}: {}'", key, value);

        Ok(())
    }
    pub fn read_png_metadata(file_path: &Path) -> Result<Vec<(String, String)>, Box<dyn std::error::Error>> {
        // Open the PNG file
        let file = File::open(file_path)?;
        let decoder = Decoder::new(file);
        let reader = decoder.read_info()?;
    
        let mut metadata = Vec::new();
    
        // Process uncompressed `tEXt` chunks
        for text_chunk in &reader.info().uncompressed_latin1_text {
            metadata.push((
                text_chunk.keyword.clone(),
                text_chunk.text.clone(),
            ));
        }
        Ok(metadata)
    }

    pub fn scan_images_with_tags(folder_path: &str) {
        let allowed_extensions = ["jpg", "png"];
        let entries = match fs::read_dir(folder_path) {
            Ok(entries) => entries,
            Err(e) => {
                eprintln!("Failed to read folder: {}", e);
                return;
            }
        };

        for entry in entries {
            let entry = match entry {
                Ok(entry) => entry,
                Err(e) => {
                    eprintln!("Failed to process entry: {}", e);
                    continue;
                }
            };

            let path = entry.path();
            if path.is_file() {
                if let Some(extension) = path.extension() {
                    if let Some(ext_str) = extension.to_str() {
                        if allowed_extensions.contains(&ext_str.to_lowercase().as_str()) {
                            println!("Processing image: {:?}", path.file_name());

                            match TagManager::get_image_format(&path) {
                                Some(ImageFormat::PNG) => {
                                    match TagManager::read_png_metadata(&path) {
                                        Ok(metadata) => {
                                            if metadata.is_empty() {
                                                println!("  No metadata found.");
                                            } else {
                                                println!("  Metadata:");
                                                for (key, value) in metadata {
                                                    println!("    {}: {}", key, value);
                                                }
                                            }
                                        }
                                        Err(e) => {
                                            println!("  Failed to read PNG metadata: {}", e);
                                        }
                                    }
                                }
                                Some(ImageFormat::JPG) => {
                                    println!("  Metadata reading not implemented for JPG.");
                                }
                                None => {
                                    println!("  Unsupported file format.");
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
