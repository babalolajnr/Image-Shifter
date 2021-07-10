mod converter;
mod constants;
mod opt;

use converter::Converter;
use opt::Opt;
use structopt::StructOpt;
fn main() {

    let opt = Opt::from_args();
    let converter: Converter = Converter::new(opt.input);

   let output_path: Result<String, image::ImageError> = match opt.action {
        _grayscale => converter.convert_to_grayscale()
    };

    println!("Converted successfully \nLocation: {}", output_path.unwrap());

}
