mod constants;
mod converter;
mod opt;

use converter::Converter;
use opt::Action;
use opt::Opt;
use structopt::StructOpt;
fn main() {
    let opt = Opt::from_args();

    let converter: Converter = Converter::new(opt.input);

    let output_path: Result<String, image::ImageError> = match opt.action {
        Action::Brighten { value } => converter.brighten_image(value),
        Action::Grayscale => converter.convert_to_grayscale(),
    };

    println!(
        "Converted successfully \nLocation: {}",
        output_path.unwrap()
    );
}
