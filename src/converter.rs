use crate::constants::OUTPUT_DIR;
use image::{DynamicImage, ImageError};
use std::fs::create_dir_all;
use std::path::Path;
use std::str;

use directories::UserDirs;
use image::imageops;
use image::io::Reader;

pub struct Converter {
    decoded_input: DynamicImage,
    output_path: String,
}

impl Converter {
    pub fn new(input: String) -> Self {
        // Check if path exists
        if !Path::new(&input).exists() {
            panic!("The input location does not exist, {}", input)
        }

        let decoded_input: DynamicImage = Reader::open(&input).unwrap().decode().unwrap();
        //Get path to documnent directory depending on the OS
        let user_dirs: UserDirs = UserDirs::new().unwrap();
        let document_dir: &str = user_dirs
            .document_dir()
            .unwrap()
            .as_os_str()
            .to_str()
            .unwrap();

        #[cfg(target_os = "windows")]
        let output_path = &mut str::replace(document_dir, r#"\"#, "/").to_owned();

        #[cfg(not(target_os = "windows"))]
        let output_path = document_dir.to_owned();

        let filename: &str = Path::new(&input).file_name().unwrap().to_str().unwrap();

        output_path.push_str(OUTPUT_DIR);
        if !Path::new(&output_path).exists() {
            create_dir_all(&output_path).unwrap()
        }

        output_path.push_str(filename);
        let output_path = output_path.as_str();

        Self {
            decoded_input,
            output_path: output_path.to_string(),
        }
    }

    /// Brighten input
    pub fn brighten_image(&self) -> Result<String, ImageError> {
        let decoded_input: &DynamicImage = &self.decoded_input;
        let output_path: &String = &self.output_path;

        imageops::brighten(decoded_input, 1).save(&output_path)?;
        Ok(output_path.to_string())
    }

    /// Convert input to grayscale
    pub fn convert_to_grayscale(&self) -> Result<String, ImageError> {
        let decoded_input: &DynamicImage = &self.decoded_input;
        let output_path: &String = &self.output_path;

        imageops::grayscale(decoded_input).save(&output_path)?;
        Ok(output_path.to_string())
    }
}
