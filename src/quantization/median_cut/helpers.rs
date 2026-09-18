
use crate::{
    quantization::median_cut::types::{ColorAxis, MedianCutErrors},
    types::rgb::RGB,
};

pub fn get_longest_axis(image_pixels: &Vec<RGB>) -> Result<ColorAxis, MedianCutErrors> {
    if image_pixels.is_empty() {
        return Err(MedianCutErrors::EmptyImagePixels);
    }

    let first = &image_pixels[0];
    let mut min = (first.r, first.g, first.b);
    let mut max = (first.r, first.g, first.b);
    let mut sum: (i64, i64, i64) = (0, 0, 0);

    for p in image_pixels {
        min.0 = min.0.min(p.r);
        min.1 = min.1.min(p.g);
        min.2 = min.2.min(p.b);

        max.0 = max.0.max(p.r);
        max.1 = max.1.max(p.g);
        max.2 = max.2.max(p.b);

        sum.0 += p.r as i64;
        sum.1 += p.g as i64;
        sum.2 += p.b as i64;
    }

    let ranges = [
        (ColorAxis::R, max.0 - min.0),
        (ColorAxis::G, max.1 - min.1),
        (ColorAxis::B, max.2 - min.2),
    ];

    let (color, _) = ranges.iter().max_by(|a, b| a.1.cmp(&b.1)).unwrap();
    if ranges[0].1 != ranges[1].1 || ranges[1].1 != ranges[2].1 {
        return Ok(*color);
    }

    let n = image_pixels.len() as f64;
    let means = (sum.0 as f64 / n, sum.1 as f64 / n, sum.2 as f64 / n);

    let mut variances = (0.0f64, 0.0f64, 0.0f64);
    for p in image_pixels {
        variances.0 += (p.r as f64 - means.0).powi(2);
        variances.1 += (p.g as f64 - means.1).powi(2);
        variances.2 += (p.b as f64 - means.2).powi(2);
    }
    variances.0 /= n;
    variances.1 /= n;
    variances.2 /= n;

    let candidates = [
        (ColorAxis::R, variances.0),
        (ColorAxis::G, variances.1),
        (ColorAxis::B, variances.2),
    ];

    let (best_color, _) = candidates
        .iter()
        .max_by(|a, b| a.1.partial_cmp(&b.1).unwrap())
        .unwrap();

    Ok(*best_color)
}

pub fn find_median(color_axis: ColorAxis, image_pixels: &Vec<RGB>) -> f64 {
    if image_pixels.is_empty() {
        return 0.0;
    };

    let num_pixels = image_pixels.len() as i64;
    if num_pixels % 2 == 0 {
        let n2_index = num_pixels / 2;
        let n1_index = n2_index - 1;

        let n1 = match color_axis {
            ColorAxis::R => image_pixels[n1_index as usize].r,
            ColorAxis::G => image_pixels[n1_index as usize].g,
            ColorAxis::B => image_pixels[n1_index as usize].b,
        };

        let n2 = match color_axis {
            ColorAxis::R => image_pixels[n2_index as usize].r,
            ColorAxis::G => image_pixels[n2_index as usize].g,
            ColorAxis::B => image_pixels[n2_index as usize].b,
        };

        let median: f64 = (n1 as f64 + n2 as f64) / 2.0; // TODO!: panic here - 

        median
    } else {
        let mid_index = num_pixels / 2;
        match color_axis {
            ColorAxis::R => image_pixels[mid_index as usize].r as f64,
            ColorAxis::G => image_pixels[mid_index as usize].g as f64,
            ColorAxis::B => image_pixels[mid_index as usize].b as f64,
        }
    }
}

pub fn split_into_box(median: f64, axis: ColorAxis, box_pixels: &Vec<RGB>) -> (Vec<RGB>, Vec<RGB>) {
    let mut box1: Vec<RGB> = Vec::new();
    let mut box2: Vec<RGB> = Vec::new();

    for p in box_pixels {
        let x = match axis {
            ColorAxis::R => p.r as f64,
            ColorAxis::G => p.g as f64,
            ColorAxis::B => p.b as f64,
        };

        if x <= median {
            box1.push(p.clone());
        };

        if x > median {
            box2.push(p.clone());
        };
    }

    (box1, box2)
}

pub fn compute_centroid(box_pixels: &Vec<RGB>) -> RGB {
    if box_pixels.is_empty() {
        return RGB::new(0, 0, 0);
    };

    let mut r: u64 = 0;
    let mut g: u64 = 0;
    let mut b: u64 = 0;

    for p in box_pixels {
        r += p.r as u64;
        g += p.g as u64;
        b += p.b as u64;
    }

    let n = box_pixels.len() as f64;

    RGB::new(
        (r as f64 / n).ceil() as u8,
        (g as f64 / n).ceil() as u8,
        (b as f64 / n).ceil() as u8,
    )
}
