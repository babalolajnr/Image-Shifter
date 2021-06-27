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
    // let output: &String= &args[4];
    // let operation: &String = &args[5];

    convert_to_grayscale(input);
}

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




