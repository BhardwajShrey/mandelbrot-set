use num::Complex;
use std::str::FromStr;

fn main() {
    println!("Hello, world!");
}

/// try to determine if 'c' is in the mandelbrot set using at most 'limit' iterations to decide
///
/// return Some(i) where 'i' is the number of iterations it took for 'c' to leave a circle of radius
/// 2 centered at the origin. if we reach the iteration limit without proving that 'c' is not a
/// member, return None
fn escape_time(c: Complex<f64>, limit: usize) -> Option<usize> {
    let mut z = Complex {
        re: 0.0,
        im: 0.0
    };

    for i in 0..limit {
        if z.norm_sqr() > 4.0 {
            return Some(i);
        }
        z = z * z + c;
    }
    None
}

/// parse string 's' as coordinate pair, with format <left><delimiter><right>
///
/// delimiter is to be specified using the 'separator' argument and must be an ASCII character
/// return Some((x, y)) if 's' has proper form, otherwise None.
fn parse_pair<T: FromStr>(s: &str, separator: char) -> Option<(T, T)> {
    match s.find(separator) {
        None => None,
        Some(index) => {
            match (T::from_str(&s[..index]), T::from_str(&s[(index + 1)..])) {
                (Ok(l), Ok(r)) => Some((l, r)),
                _ => None
            }
        }
    }
}

#[test]
fn test_parse_pair() {
    assert_eq!(parse_pair::<i32>("", ','), None);
    assert_eq!(parse_pair::<i32>("10,", ','), None);
    assert_eq!(parse_pair::<i32>(",10", ','), None);
    assert_eq!(parse_pair::<i32>("10,20", ','), Some((10, 20)));
    assert_eq!(parse_pair::<i32>("10,20xy", ','), None);
    assert_eq!(parse_pair::<f64>("0.5x", 'x'), None);
    assert_eq!(parse_pair::<f64>("0.5x1.5", 'x'), Some((0.5, 1.5)));
}

/// parse a pair of floating point numbers into a complex number
fn parse_complex(s: &str) -> Option<Complex<f64>> {
    match parse_pair(s, ',') {
        Some((re, im)) => Some(Complex { re, im }),
        None => None
    }
}

#[test]
fn test_parse_complex() {
    assert_eq!(parse_complex("1.25,-0.0625"), Some(Complex { re: 1.25, im: -0.0625 }));
    assert_eq!(parse_complex(",-0.0625"), None);
}

/// return the corresponding point on the complex plane, given the (row, column) coordinates of a
/// pixel in the output image
/// 'bounds' = width and height of the image in pixels
fn pixel_to_point(
    bounds: (usize, usize),
    pixel: (usize, usize),
    upper_left: Complex<f64>,
    lower_right: Complex<f64>
) -> Complex<f64> {
    let (width, height) = (lower_right.re - upper_left.re, upper_left.im - lower_right.im);

    Complex {
        re: upper_left.re + pixel.0 as f64 * width / bounds.0 as f64,
        im: upper_left.im - pixel.1 as f64 * height / bounds.1 as f64
    }
}

#[test]
fn test_pixel_to_point() {
    assert_eq!(
        Complex { re: -0.5, im: -0.75 },
        pixel_to_point((100, 200), (25, 175), Complex { re: -1.0, im: 1.0 }, Complex { re: 1.0, im: -1.0 })
    );
}
