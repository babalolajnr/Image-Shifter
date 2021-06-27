use std::path::Path;
use std::env;
use std::fs;

use image::imageops;
use image::io::Reader;

// use image::GenericImageView;
fn main() {

    let args: Vec<String> = env::args().collect();

    // println!("args {:?}", args[1]);
    // cargo run convert input to output grayscale
    let input: &String = &args[2];
    let action: &String = &args[4];
    let config = Config::new(input.to_owned(), action.to_owned());

    let action = config.action.to_uppercase();

   if action == "GRAYSCALE".to_owned() {
    convert_to_grayscale(&config.input);
   }
    
}
/// Convert input to grayscale
fn convert_to_grayscale(input: &String){
    let decoded_input = Reader::open(input).unwrap().decode().unwrap();
    let output_path = "C:/Users/User/Documents/Image Shifter";
    fs::create_dir_all(&output_path).unwrap();
    
    let filename = Path::new(&input).file_name().unwrap().to_str().unwrap();
    let mut output = String::from(output_path);
    output.push_str("/");
    output.push_str(filename);

    imageops::grayscale(&decoded_input).save(output).unwrap();
} 

struct Config {
    input: String,
    action: String
}

impl Config {
    fn new(input: String, action: String) -> Self { Self { input, action } }
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


