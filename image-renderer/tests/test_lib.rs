use image_renderer::{parse_complex, parse_pair, pixel_to_point};
use num::Complex;

#[test]
fn test_parse_pair() {
    assert_eq!(parse_pair::<i32>("", ','), None);
    assert_eq!(parse_pair::<i32>("12,22", ','), Some((12, 22)));
    assert_eq!(
        parse_complex("1.23,-0.877"),
        Some(Complex {
            re: 1.23,
            im: -0.877
        })
    );
}

#[test]
fn test_pixel_to_point() {
    assert_eq!(
        pixel_to_point(
            (100, 200),
            (25, 175),
            Complex { re: -1.0, im: 1.0 },
            Complex { re: 1.0, im: -1.0 }
        ),
        Complex {
            re: -0.5,
            im: -0.75
        }
    );
}
