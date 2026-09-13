use crate::{quantization::median_cut::types, types::rgb::RGB};

pub struct MedianCutAlgorithm {}

impl MedianCutAlgorithm {
    pub fn new() -> Self {
        MedianCutAlgorithm {}
    }

    pub fn GetColors(&self, imagePixels: Vec<RGB>, n: i64) -> Result<Vec<RGB>, types::MedianCutErrors> {
        Ok(Vec::new())
    }
}
