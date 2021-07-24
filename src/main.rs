mod constants;
mod converter;
mod opt;

use converter::Converter;
use opt::Action;
use opt::Opt;
use structopt::StructOpt;
fn main() {
    let opt = Opt::from_args();

    let converter: Converter = Converter::new(opt.input, opt.output);

    let output_path: Result<String, image::ImageError> = match opt.action {
        Action::Brighten { value } => converter.brighten(value),
        Action::Grayscale => converter.grayscale(),
        Action::Huerotate { value } => converter.huerotate(value),
        Action::Contrast { value } => converter.contrast(value),
        Action::Crop {
            x,
            y,
            width,
            height,
        } => converter.crop(x, y, width, height),
    };

    println!(
        "Converted successfully \nLocation: {}",
        output_path.unwrap()
    );
}
