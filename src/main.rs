use clap::Parser;
use image::GenericImageView;
use rand::seq::IteratorRandom;
use std::fs::{metadata, read_dir};

use crate::{quantization::median_cut::median::MedianCutAlgorithm, types::rgb::RGB};
mod quantization;
mod types;

#[derive(Parser)]
#[command(
    name = "drywal",
    version,
    about = "extracts the most common colors in a image and applies it system wide"
)]
struct DrywalArgs {
    #[arg(short = 'i', long)]
    input: String,

    #[arg(short = 'c', long, default_value_t = 16)]
    colors: i64,
}

fn random_file_in_dir(dir: &String) -> Option<String> {
    let mut rng = rand::rng();
    read_dir(dir)
        .ok()?
        .filter_map(|entry| entry.ok())
        .map(|entry| entry.path())
        .filter(|path| path.is_file())
        .choose(&mut rng)
        .map(|path| path.to_string_lossy().into_owned())
}

fn get_brightnes(c: &RGB) -> f64 {
    0.299 * c.r as f64 + 0.587 * c.g as f64 + 0.114 * c.b as f64
}

fn main() {
    let cli = DrywalArgs::parse();

    let md = match metadata(&cli.input) {
        Ok(v) => v,
        Err(e) => {
            println!("failed to parse input: {}", e);
            return;
        }
    };

    let input_file: String = if md.is_dir() {
        random_file_in_dir(&cli.input).expect("no files found in directory")
    } else {
        cli.input
    };

    println!("\n{}", input_file);
    let res = match image::open(input_file.clone()) {
        Ok(v) => v,
        Err(e) => {
            println!("failed to open image: {}", e);
            return;
        }
    };

    let q = MedianCutAlgorithm::new();

    let mut colors = q
        .get_colors(
            res.pixels()
                .map(|(_, _, p)| RGB::new(p.0[0], p.0[1], p.0[2]))
                .collect(),
            cli.colors,
        )
        .unwrap();

    colors.sort_by(|a, b| get_brightnes(&a).total_cmp(&get_brightnes(b)));

    colors
        .iter()
        .for_each(|c| print!("\x1b[48;2;{};{};{}m  \x1b[0m", c.r, c.g, c.b));
    println!("");
}
