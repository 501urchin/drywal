#[derive(Debug)]
pub enum MedianCutErrors {
    FailedToInitialize,
    EmptyImagePixels,
}

#[derive(Clone, Copy, Debug)]
pub enum ColorAxis {
    R,
    G,
    B,
}
