#[derive(Debug)]
pub enum MedianCutErrors {
    // FailedToInitialize,
    EmptyImagePixels,
}

#[derive(Clone, Copy, Debug,PartialEq)]
pub enum ColorAxis {
    R,
    G,
    B,
}
