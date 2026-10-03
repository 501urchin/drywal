use crate::{quantization::median_cut::median::MedianCutAlgorithm, types::rgb::RGB};
use image::{GenericImageView, imageops::FilterType::Lanczos3};
use rand::seq::IteratorRandom;
use std::fs::read_dir;

pub fn random_file_in_dir(dir: &String) -> Option<String> {
    let mut rng = rand::rng();
    read_dir(dir)
        .ok()?
        .filter_map(|entry| entry.ok())
        .map(|entry| entry.path())
        .filter(|path| path.is_file())
        .choose(&mut rng)
        .map(|path| path.to_string_lossy().into_owned())
}

pub fn get_brightnes(c: &RGB) -> f64 {
    0.299 * c.r as f64 + 0.587 * c.g as f64 + 0.114 * c.b as f64
}

pub fn get_colors_from_image_path(
    path: String,
    colors: i64,
    resize: bool,
) -> Result<Vec<RGB>, String> {
    let mut res = match image::open(path) {
        Ok(v) => v,
        Err(e) => {
            return Err(format!("failed to open image: {}", e).to_string());
        }
    };

    if resize {
        res = res.resize(256, 256, Lanczos3);
    }

    let q = MedianCutAlgorithm::new();

    let mut colors = q
        .get_colors(
            res.pixels()
                .map(|(_, _, p)| RGB::new(p.0[0], p.0[1], p.0[2]))
                .collect(),
            colors,
        )
        .unwrap();

    colors.sort_by(|a, b| get_brightnes(&a).total_cmp(&get_brightnes(b)));

    Ok(colors)
}

pub fn print_colors(colors: &Vec<RGB>) {
    print!("color preview: ");
    colors
        .iter()
        .for_each(|c| print!("\x1b[48;2;{};{};{}m  \x1b[0m", c.r, c.g, c.b));
    println!("");
}
