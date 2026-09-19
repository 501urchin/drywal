use image::{GenericImageView};

use crate::{quantization::median_cut::median, types::rgb::RGB};
mod quantization;
mod types;

fn get_brightnes(c: &RGB) -> f64 {
    0.299 * c.r as f64 + 0.587 * c.g as f64 + 0.114 * c.b as f64
}

fn main() {
    let img = image::open("image.png").unwrap();
    let mut image_rgb: Vec<RGB> = Vec::new();

    img.pixels()
        .for_each(|(_, _, pixel)| image_rgb.push(RGB::new(pixel.0[0], pixel.0[1], pixel.0[2])));

    if image_rgb.is_empty() {
        panic!("image is empty")
    };

    let mut colors = median::MedianCutAlgorithm::new()
        .get_colors(image_rgb, 8)
        .unwrap();

    colors.sort_by(|a, b| get_brightnes(a).total_cmp(&get_brightnes(&b)));
    colors
        .iter()
        .for_each(|c| print!("\x1b[48;2;{};{};{}m  \x1b[0m", c.r, c.g, c.b));
    println!()
}
