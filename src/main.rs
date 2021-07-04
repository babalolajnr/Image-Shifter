mod converter;
mod config;

use crate::converter::Converter;
use crate::config::Config;
use std::env;

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

