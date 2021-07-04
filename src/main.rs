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

    // cargo run convert input to output grayscale
    let input: &String = &args[2];
    let action: &String = &args[4];
    let config = Config::new(input.to_owned(), action.to_owned());

    let action = config.action.to_uppercase();

    // if action == "GRAYSCALE".to_owned() {
    //     convert_to_grayscale(&config.input);
    // }
    match action.as_str() {
        "GRAYSCALE" => convert_to_grayscale(&config.input),
        "BRIGHTEN" => brighten_image(&config.input),
        _ => panic!("Action is not available"),
    }
}
/// Convert input to grayscale
fn convert_to_grayscale(input: &String) {
    let decoded_input = Reader::open(input).unwrap().decode().unwrap();
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

    let output_path = String::from(output_path);
    imageops::grayscale(&decoded_input)
        .save(&output_path)
        .unwrap(); // unwrap and check for exception
    println!("Conversion successful. '{}'", output_path)
}

fn brighten_image(input: &String) {
    let decoded_input = Reader::open(input).unwrap().decode().unwrap();
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

    let output_path = String::from(output_path);

    imageops::brighten(&decoded_input, 1)
        .save(&output_path)
        .unwrap(); // unwrap and check for exception
    println!("Conversion successful. '{}'", output_path)
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

// enum Actions {
//     Grayscale
// }

// impl Actions {
//     /// Returns `true` if the actions is [`Grayscale`].
//     fn is_grayscale(&self) -> bool {
//         matches!(self, Self::Grayscale)
//     }
// }
