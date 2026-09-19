#[cfg(test)]
mod tests {
    use crate::{
        quantization::median_cut::{helpers, types::ColorAxis},
        types::rgb::{RGB},
    };

    #[test]
    fn test_get_longest_axis_happy() {
        #[rustfmt::skip]
        let pixels = vec![
            RGB { r: 1,   g: 10,  b: 255  },
            RGB { r: 10,  g: 14,  b: 0   },
            RGB { r: 255, g: 5,   b: 55 },
        ];

        let res = helpers::get_longest_axis(&pixels).unwrap();

        assert_eq!(res, ColorAxis::R);
    }

    #[test]
    fn test_get_longest_axis_should_return_red_if_all_range_the_same() {
        #[rustfmt::skip]
        let pixels = vec![
            RGB { r: 1,   g: 10,  b: 255  },
            RGB { r: 0,  g: 255,  b: 0   },
            RGB { r: 255, g: 0,   b: 55 },
        ];

        let res = helpers::get_longest_axis(&pixels).unwrap();

        assert_eq!(res, ColorAxis::R);
    }

    #[test]
    fn test_get_longest_axis_empty() {
        #[rustfmt::skip]
        let pixels = vec![];

        let res = helpers::get_longest_axis(&pixels);
        assert!(res.is_err());
    }

    #[test]
    fn test_find_median_uneven_len() {
        #[rustfmt::skip]
        let pixels = vec![
            RGB { r: 1, g: 10,   b: 255  },
            RGB { r: 10, g: 14,  b: 0   },
            RGB { r: 255, g: 5,   b: 55 },
        ];

        let median = helpers::find_median(ColorAxis::R, &pixels);

        assert_eq!(median, 10 as f64);
    }

    #[test]
    fn test_find_median_even_len() {
        #[rustfmt::skip]
        let pixels = vec![
            RGB { r: 1, g: 10,   b: 255  },
            RGB { r: 10, g: 14,  b: 0   },
            RGB { r: 254, g: 5,   b: 55 },
            RGB { r: 255, g: 5,   b: 55 },
        ];

        let median = helpers::find_median(ColorAxis::R, &pixels);
        assert_eq!(median, 132 as f64);
    }

    #[test]
    fn test_compute_centroid() {
        #[rustfmt::skip]
        let pixels = vec![
            RGB { r: 1,   g: 10,  b: 255  },
            RGB { r: 10,  g: 14,  b: 0   },
        ];

        let centroid = helpers::compute_centroid(&pixels);

        assert_eq!(centroid, RGB::new(6, 12, 128))
    }

    #[test]
    fn test_split_into_box() {
        #[rustfmt::skip]
        let pixels = vec![
            RGB { r: 1, g: 10,   b: 255  },
            RGB { r: 10, g: 14,  b: 0   },
            RGB { r: 254, g: 5,   b: 55 },
            RGB { r: 255, g: 5,   b: 55 },
        ];

        let (box1, box2) = helpers::split_into_box(132 as f64, ColorAxis::R, &pixels);

        assert_eq!(
            box1,
            vec![
                RGB {
                    r: 1,
                    g: 10,
                    b: 255
                },
                RGB { r: 10, g: 14, b: 0 }
            ]
        );
        assert_eq!(
            box2,
            vec![
                RGB {
                    r: 254,
                    g: 5,
                    b: 55
                },
                RGB {
                    r: 255,
                    g: 5,
                    b: 55
                }
            ]
        );
    }
}
