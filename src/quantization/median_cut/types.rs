#[derive(Debug)]
pub enum MedianCutErrors {
    FailedToInitialize,
    EmptyImagePixels,
}

#[derive(Clone, Copy)]
pub enum ColorChannel {
    R,
    G,
    B,
}
