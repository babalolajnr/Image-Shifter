use std::env;

use image::imageops;
use image::io::Reader;

// use image::GenericImageView;
fn main() {
    // let image = Reader::open("assets/wallpaperflare.com_wallpaper.jpg").unwrap().decode().unwrap();
    // let grayscale = imageops::grayscale(&image);
    // grayscale.save("assets/grayscale.jpg").unwrap();

    let args: Vec<String> = env::args().collect();
    // println!("args {:?}", args[1]);
    // cargo run convert input to output grayscale
    let input_image: &String = &args[2];
    let output_image: &String= &args[4];
    let operation: &String = &args[5];

    convert_to_grayscale(input_image, output_image);
}

fn convert_to_grayscale(input: &String, output: &String){
    let input = Reader::open(input).unwrap().decode().unwrap();
    imageops::grayscale(&input).save(&output).unwrap();
} 

// fn opening_error(message: &str){
//     println!("{}", message)
// }
