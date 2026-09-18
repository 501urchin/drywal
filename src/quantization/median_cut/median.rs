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
    pub fn get_colors(&self, mut image_pixels: Vec<RGB>, _n: i64) -> Result<Vec<RGB>, MedianCutErrors> {
        Ok(Vec::new())
    }
}
