use std::collections::HashMap;
use image::GenericImageView;

struct Color {
    r: u8,
    g: u8,
    b: u8,
    count: i64,
}

fn quantize(value: u8, bucket_size: u8) -> u8 {
    (value / bucket_size) * bucket_size
}

fn main() {
    let img = image::open("image.png").unwrap();
    let bucket_size: u8 = 50; 

    let mut map: HashMap<(u8, u8, u8), i64> = HashMap::new();

    for (_, _, pixel) in img.pixels() {
        if pixel.0.len() > 3 && pixel.0[3] == 0 {
            continue;
        }

        let key = (
            quantize(pixel.0[0], bucket_size),
            quantize(pixel.0[1], bucket_size),
            quantize(pixel.0[2], bucket_size),
        );
        *map.entry(key).or_insert(0) += 1;
    }

    let mut colors: Vec<Color> = map
        .into_iter()
        .map(|((r, g, b), count)| Color { r, g, b, count })
        .collect();

    colors.sort_by(|a, b| b.count.cmp(&a.count)); 

    for c in colors.iter().take(10) {
        println!("{:02X}{:02X}{:02X}", c.r, c.g, c.b);
    }

    for c in colors.iter().take(10) {
        print!("\x1b[48;2;{};{};{}m  \x1b[0m", c.r, c.g, c.b);
    }
    println!()
}