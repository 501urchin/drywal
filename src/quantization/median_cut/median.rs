use crate::{
    quantization::median_cut::types::{ColorChannel, MedianCutErrors},
    types::rgb::RGB,
};

pub struct MedianCutAlgorithm {}

impl MedianCutAlgorithm {
    pub fn new() -> Self {
        MedianCutAlgorithm {}
    }

    fn getLongestAxisAndVariance(&self, image_pixels: &Vec<RGB>) -> Result<(ColorChannel, f64), MedianCutErrors> {
        if image_pixels.is_empty() {
            return Err( MedianCutErrors::EmptyImagePixels);
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
            (ColorChannel::R, max.0 - min.0),
            (ColorChannel::G, max.1 - min.1),
            (ColorChannel::B, max.2 - min.2),
        ];

        let (color, _) = ranges.iter().max_by(|a, b| a.1.cmp(&b.1)).unwrap();

        let mut mean = match color {
            ColorChannel::R => sum.0,
            ColorChannel::G => sum.1,
            ColorChannel::B => sum.2,
        };

        mean /= image_pixels.len() as i64;

        let mut variance: f64 = 0.0;

        for p in image_pixels {
            let mut v: i64 = match color {
                ColorChannel::R => p.r as i64,
                ColorChannel::G => p.g as i64,
                ColorChannel::B => p.b as i64,
            };

            v -= mean;

            variance += v.pow(2) as f64;
        }

        variance /= image_pixels.len() as f64;

        Ok((*color, variance))
    }

    pub fn get_colors(
        &self,
        image_pixels: Vec<RGB>,
        n: i64,
    ) -> Result<Vec<RGB>, MedianCutErrors> {
        let (_, mean) = self.getLongestAxisAndVariance(&image_pixels).unwrap();
        println!("mean: {}", mean);
        Ok(Vec::new())
    }
}
