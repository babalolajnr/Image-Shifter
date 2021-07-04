use std::env;
use std::fs::create_dir_all;
use std::path::Path;
use std::str;

use directories::UserDirs;
use image::imageops;
use image::io::Reader;

// use image::GenericImageView;
fn main() {
    let args: Vec<String> = env::args().collect();

    // cargo run convert input output grayscale
    let input: &String = &args[2];
    let action: &String = &args[3];
    let config = Config::new(input.to_owned(), action.to_owned());

    let action = config.action.to_uppercase();

    let converter = Converter::new(config.input);

    match action.as_str() {
        "GRAYSCALE" => converter.convert_to_grayscale(),
        "BRIGHTEN" => converter.brighten_image(),
        _ => panic!("Action is not available"),
    }
}

struct Converter {
    decoded_input: image::DynamicImage,
    output_path: String,
}

impl Converter {
    fn new(input: String) -> Self {
        let decoded_input = Reader::open(&input).unwrap().decode().unwrap();
        let user_dirs = UserDirs::new().unwrap();
        let document_dir = user_dirs
            .document_dir()
            .unwrap()
            .as_os_str()
            .to_str()
            .unwrap();

        let output_path = &mut str::replace(document_dir, r#"\"#, "/").to_owned();

        let filename = Path::new(&input).file_name().unwrap().to_str().unwrap();

        output_path.push_str("/Image Shifter/");
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
    fn brighten_image(&self) {
        let decoded_input = &self.decoded_input;
        let output_path = &self.output_path;

        imageops::brighten(decoded_input, 1)
            .save(&output_path)
            .unwrap(); // unwrap and check for exception
        println!("Conversion successful. '{}'", output_path)
    }

    /// Convert input to grayscale
    fn convert_to_grayscale(&self) {
        let decoded_input = &self.decoded_input;
        let output_path = &self.output_path;

        imageops::grayscale(decoded_input)
            .save(&output_path)
            .unwrap(); // unwrap and check for exception
        println!("Conversion successful. '{}'", output_path)
    }
}

struct Config {
    input: String,
    action: String,
}

impl Config {
    fn new(input: String, action: String) -> Self {
        Self { input, action }
    }
}
