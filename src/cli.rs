use clap::Parser;

#[derive(Parser)]
#[command(
    name = "drywal",
    version,
    about = "extracts the most common colors in a image and applies it system wide"
)]
pub struct DrywalArgs {
    #[arg(short = 'i', long)]
    pub input: String,

    #[arg(short = 'c', long, default_value_t = 16)]
    pub colors: i64,

    #[arg(short = 'r', long, default_value_t = false)]
    pub resize: bool,

    #[arg(short = 'p', long, default_value_t = false)]
    pub preview: bool,

    #[arg(long, default_value_t = String::from("~/config/drywal/templates"))]
    pub tpath: String,

    #[arg(short = 'o', long, default_value_t = String::from("./"))]
    pub outpath: String,
}

pub fn parse_args() -> DrywalArgs {
    DrywalArgs::parse()
}
