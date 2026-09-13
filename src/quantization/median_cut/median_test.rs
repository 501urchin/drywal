#[cfg(test)]
mod tests {

    use crate::{quantization::median_cut::median::MedianCutAlgorithm, types::rgb::RGB};

    static SRC: &[RGB] = &[
        RGB { r: 255, g: 0, b: 0 },
        RGB {
            r: 250,
            g: 10,
            b: 5,
        },
        RGB {
            r: 240,
            g: 5,
            b: 10,
        },
        RGB { r: 0, g: 255, b: 0 },
        RGB { r: 5, g: 250, b: 5 },
        RGB {
            r: 10,
            g: 240,
            b: 0,
        },
        RGB { r: 0, g: 0, b: 255 },
        RGB { r: 5, g: 5, b: 250 },
    ];

    #[test]
    fn test_returns_correct_amount_of_palettes() {
        let mc = MedianCutAlgorithm::new();

        let p_len = 4;
        let res = mc.GetColors(SRC.to_vec(), p_len).unwrap();

        assert_eq!(res.len(), p_len as usize);
    }
    
    #[test]
    fn test_returns_correct_palettes() {
        let mc = MedianCutAlgorithm::new();

        let expected: [RGB; 4] = [
            RGB { r: 3, g: 3, b: 253 },
            RGB { r: 3, g: 253, b: 3 },
            RGB {
                r: 125,
                g: 123,
                b: 5,
            },
            RGB { r: 253, g: 5, b: 3 },
        ];

        let p_len = 4;
        let res = mc.GetColors(SRC.to_vec(), p_len).unwrap();

        assert_eq!(res, expected);
    }
}
