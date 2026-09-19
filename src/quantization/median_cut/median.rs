use std::vec;

use crate::{
    quantization::median_cut::{
        helpers::{self, compute_centroid, find_median, split_into_box},
        types::{ColorAxis, MedianCutErrors},
    },
    types::rgb::RGB,
};

pub struct MedianCutAlgorithm {}

impl MedianCutAlgorithm {
    pub fn new() -> Self {
        MedianCutAlgorithm {}
    }

    pub fn get_colors(&self, image_pixels: Vec<RGB>, n: i64) -> Result<Vec<RGB>, MedianCutErrors> {
        let mut boxes: Vec<Vec<RGB>> = vec![image_pixels.clone()];

        while boxes.len() < n as usize {
            let target = boxes.remove(0);

            let axis = match helpers::get_longest_axis(&target) {
                Ok(v) => v,
                Err(_) => continue,
            };

            let mut target = target;
            match axis {
                ColorAxis::R => target.sort_by(|a, b| a.r.cmp(&b.r)),
                ColorAxis::G => target.sort_by(|a, b| a.g.cmp(&b.g)),
                ColorAxis::B => target.sort_by(|a, b| a.b.cmp(&b.b)),
            };

            let median = find_median(axis, &target);
            let (box1, box2) = split_into_box(median, axis, &target);

            boxes.push(box1);
            boxes.push(box2);
        }

        Ok(boxes.iter().map(compute_centroid).collect())
    }
}
