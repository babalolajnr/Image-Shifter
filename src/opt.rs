use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "Image Shifter", about = "Manipulate images from the terminal")]
pub struct Opt {
    /// Action
    #[structopt(subcommand)]
    pub action: Action,

    #[structopt(help = "Path to the image to be manipulated")]
    pub input: String,
}

#[derive(Debug, StructOpt)]
pub enum Action {
    Grayscale,
    Brighten {
        #[structopt(
            help = "Amount to brighten each pixel by. Negative values decrease the brightness and positive values increase it.",
            short,
            long
        )]
        value: i32,
    },
    Huerotate {
        #[structopt(
            help = "Amount of degrees to rotate each pixel by. 0 and 360 do nothing",
            short,
            long
        )]
        value: i32,
    },
    Contrast {
        #[structopt(
            help = "Amount to adjust the contrast by. Negative values decrease the contrast and positive values increase the contrast.",
            short,
            long
        )]
        value: f32,
    }
}
