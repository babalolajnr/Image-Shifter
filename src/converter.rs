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
    pub fn brighten(&self, value: i32) -> Result<String, ImageError> {
        imageops::brighten(&self.decoded_input, value).save(&self.output_path)?;
        Ok(self.output_path.to_string())
    }

    /// Convert input to grayscale
    pub fn grayscale(&self) -> Result<String, ImageError> {
        imageops::grayscale(&self.decoded_input).save(&self.output_path)?;
        Ok(self.output_path.to_string())
    }

    /// Huerotate input
    pub fn huerotate(&self, value: i32) -> Result<String, ImageError> {
        imageops::huerotate(&self.decoded_input, value).save(&self.output_path)?;
        Ok(self.output_path.to_string())
    }

    pub fn contrast(&self, contrast: f32) -> Result<String, ImageError> {
        imageops::contrast(&self.decoded_input, contrast).save(&self.output_path)?;
        Ok(self.output_path.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_converter_returns_decoded_image() {
        let input = r"C:\Users\User\Downloads\cosmonaut_astronaut_space_suit_137404_3840x2400.jpg";
        let converter = Converter::new(input.to_string());

        let decoded_input: DynamicImage = Reader::open(&input).unwrap().decode().unwrap();

        assert_eq!(converter.decoded_input, decoded_input);
    }
}
