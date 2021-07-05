mod config;
mod converter;
mod constants;

use config::Config;
use converter::Converter;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    // cargo run convert input output grayscale
    let input: &String = &args[2];
    let action: &String = &args[3];
    let config: Config = Config::new(input.to_owned(), action.to_owned());

    let action: String = config.action.to_uppercase();

    let converter: Converter = Converter::new(config.input);

    let output_path: Result<String, image::ImageError> = match action.as_str() {
        "GRAYSCALE" => converter.convert_to_grayscale(),
        "BRIGHTEN" => converter.brighten_image(),
        _ => panic!("Action is not available"),
    };

    println!("Converted successfully \nLocation: {}", output_path.unwrap());
}
