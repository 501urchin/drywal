#[derive(Debug)]
pub enum MedianCutErrors {
    FailedToInitialize,
    EmptyImagePixels,
}

#[derive(Clone, Copy)]
pub enum ColorAxis {
    R,
    G,
    B,
}
