use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "Image Shifter", about = "Manipulate images from the terminal")]
pub struct Opt {
    /// Action
    #[structopt(subcommand)]
    pub action: Action,

    #[structopt(help = "Pass in the path to the image to be manipulated")]
    pub input: String,
}

#[derive(Debug, StructOpt)]
pub enum Action {
    Grayscale,
}
