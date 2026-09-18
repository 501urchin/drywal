use crate::{
    quantization::median_cut::types::{ColorAxis, MedianCutErrors},
    types::rgb::RGB,
};

pub struct MedianCutAlgorithm {}

impl MedianCutAlgorithm {
    pub fn new() -> Self {
        MedianCutAlgorithm {}
    }

    #[rustfmt::skip]
    // TODO: calc variance for each range and calc
    fn get_longest_axis(&self, image_pixels: &Vec<RGB>) -> Result<ColorAxis, MedianCutErrors> {
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
            (ColorAxis::R, max.0 - min.0),
            (ColorAxis::G, max.1 - min.1),
            (ColorAxis::B, max.2 - min.2),
        ];

        let (color, _) = ranges.iter().max_by(|a, b| a.1.cmp(&b.1)).unwrap();
        if ranges[0].1 != ranges[1].1 || ranges[1].1 != ranges[2].1 {
            return Ok(*color);
        }

        let n = image_pixels.len() as f64;
        let means = (
            sum.0 as f64 / n,
            sum.1 as f64 / n,
            sum.2 as f64 / n,
        );

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

    fn find_median(&self, color_axis: ColorAxis, image_pixels: &Vec<RGB>) -> f64 {
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

            let median: f64 = (n1 + n2) as f64 / 2.0; // TODO!: panic here - 

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

    fn split_into_box(
        &self,
        median: f64,
        axis: ColorAxis,
        box_pixels: &Vec<RGB>,
    ) -> (Vec<RGB>, Vec<RGB>) {
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

    fn compute_centroid(&self, box_pixels: &Vec<RGB>) -> RGB {
        let mut centroid: RGB = RGB::new(0, 0, 0);
        for p in box_pixels {
            centroid.r += p.r;
            centroid.g += p.g;
            centroid.b += p.b;
        }

        let n = box_pixels.len() as u8;
        centroid.r /= n;
        centroid.g /= n;
        centroid.b /= n;

        centroid
    }

    #[rustfmt::skip]
    pub fn get_colors(&self, mut image_pixels: Vec<RGB>, _n: i64) -> Result<Vec<RGB>, MedianCutErrors> {
        let axis = self.get_longest_axis(&image_pixels).unwrap();
        match axis {
            ColorAxis::R => image_pixels.sort_by(|a, b| a.r.cmp(&b.r)),
            ColorAxis::G => image_pixels.sort_by(|a, b| a.r.cmp(&b.g)),
            ColorAxis::B => image_pixels.sort_by(|a, b| a.r.cmp(&b.b)),
        };
        let median = self.find_median(axis, &image_pixels);
        let (mut b1,mut b2) = self.split_into_box(median, axis, &image_pixels);

        let b1_axis = self.get_longest_axis(&b1).unwrap();
        match b1_axis {
            ColorAxis::R => b1.sort_by(|a, b| a.r.cmp(&b.r)),
            ColorAxis::G => b1.sort_by(|a, b| a.r.cmp(&b.g)),
            ColorAxis::B => b1.sort_by(|a, b| a.r.cmp(&b.b)),
        };
        
        let b1_median = self.find_median(b1_axis, &b1);
        let (box1, box2) = self.split_into_box(b1_median, axis, &b1);

        let b2_axis = self.get_longest_axis(&b2).unwrap();
        match b2_axis {
            ColorAxis::R => b2.sort_by(|a, b| a.r.cmp(&b.r)),
            ColorAxis::G => b2.sort_by(|a, b| a.r.cmp(&b.g)),
            ColorAxis::B => b2.sort_by(|a, b| a.r.cmp(&b.b)),
        };
        
        let b2_median = self.find_median(b2_axis, &b2);
        let (box3, box4) = self.split_into_box(b2_median, axis, &b2);



        let centroid1 = self.compute_centroid(&box1);
        let centroid2 = self.compute_centroid(&box2);
        let centroid3 = self.compute_centroid(&box3);
        let centroid4 = self.compute_centroid(&box4);

        println!("centroid 1: {:?}", centroid1);
        println!("centroid 2: {:?}", centroid2);
        println!("centroid 3: {:?}", centroid3);
        println!("centroid 4: {:?}", centroid4);
        Ok(Vec::new())
    }
}
