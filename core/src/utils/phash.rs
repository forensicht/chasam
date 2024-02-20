use image::{imageops::FilterType, Luma};
use std::path::Path;

type PixelMatrix = Vec<Vec<Luma<u8>>>;

#[allow(unused)]
pub fn difference_hash<P>(path: P) -> Result<u64, image::ImageError>
where
    P: AsRef<Path>,
{
    if let Some(p) = path.as_ref().to_str() {
        let img = image::open(p)?;

        let (w, h) = (9, 8);
        let img_resized = img.resize_exact(w, h, FilterType::Lanczos3);
        let img_buf = img_resized.to_luma8();
        let pixel_matrix = image_to_pixel_matrix(&img_buf);
        let mut idx: u64 = 0;
        let mut hash: u64 = 0;

        let (w, h) = (w as usize, h as usize);

        for y in 0..h {
            for x in 0..w - 1 {
                if pixel_matrix[y][x].0 > pixel_matrix[y][x + 1].0 {
                    hash |= 1 << (64 - idx - 1) as u32;
                }
                idx += 1;
            }
        }

        return Ok(hash);
    }

    Ok(0)
}

#[allow(unused)]
pub fn average_hash<P>(path: P) -> Result<u64, image::ImageError>
where
    P: AsRef<Path>,
{
    if let Some(p) = path.as_ref().to_str() {
        let img = image::open(p)?;

        let (w, h) = (8, 8);
        let img_resized = img.resize_exact(w, h, FilterType::Lanczos3);
        let img_buf = img_resized.to_luma8();
        let pixel_matrix = image_to_pixel_matrix(&img_buf);
        let mut float_pixels: Vec<u8> = Vec::with_capacity(64);
        let mut sum = 0u32;

        let (w, h) = (w as usize, h as usize);

        for y in 0..h {
            for x in 0..w {
                sum += pixel_matrix[y][x].0[0] as u32;
                float_pixels.push(pixel_matrix[y][x].0[0]);
            }
        }

        let avg = (sum / 64) as u8;
        let mut idx: u64 = 0;
        let mut hash: u64 = 0;

        for p in float_pixels {
            if p > avg {
                hash |= 1 << (64 - idx - 1) as u32;
            }
            idx += 1;
        }

        return Ok(hash);
    }

    Ok(0)
}

#[allow(unused)]
pub fn distance(l_hash: u64, r_hash: u64) -> u32 {
    let hamming = l_hash ^ r_hash;
    hamming.count_ones()
}

#[allow(unused)]
pub fn hex_to_u64(hex: &str) -> Result<u64, std::num::ParseIntError> {
    u64::from_str_radix(hex, 16)
}

fn new_canvas(x: u32, y: u32) -> PixelMatrix {
    let canvas = vec![vec![image::Luma([255 as u8]); y as usize]; x as usize];
    canvas
}

fn image_to_pixel_matrix(img: &image::GrayImage) -> PixelMatrix {
    let (w, h) = img.dimensions();
    let mut output_matrix = new_canvas(h, w);

    for y in 0..h {
        for x in 0..w {
            output_matrix[y as usize][x as usize] = *img.get_pixel(x, y);
        }
    }

    output_matrix
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_difference_hash() {
        let l_path = std::env::current_dir()
            .unwrap()
            .join("tests")
            // .join("original.png");
            .join("cavalo.jpg");

        let r_path = std::env::current_dir()
            .unwrap()
            .join("tests")
            // .join("original-blur.png");
            .join("cavalo-2.jpg");

        let l_hash = match difference_hash(&l_path) {
            Ok(hash) => {
                println!("{} {}", format!("{hash:X}"), hash);
                hash
            }
            Err(err) => {
                eprintln!("{}", format!("{err}"));
                0
            }
        };

        let r_hash = match difference_hash(&r_path) {
            Ok(hash) => {
                println!("{} {}", format!("{hash:X}"), hash);
                hash
            }
            Err(err) => {
                eprintln!("{}", format!("{err}"));
                0
            }
        };

        println!("Distance: {}", distance(l_hash, r_hash));
    }

    #[test]
    fn test_average_hash() {
        let l_path = std::env::current_dir()
            .unwrap()
            .join("tests")
            // .join("original.png");
            .join("cavalo.jpg");

        let r_path = std::env::current_dir()
            .unwrap()
            .join("tests")
            // .join("original-blur.png");
            .join("cavalo-2.jpg");

        let l_hash = match average_hash(&l_path) {
            Ok(hash) => {
                println!("{}", format!("{hash:X}"));
                hash
            }
            Err(err) => {
                eprintln!("{}", format!("{err}"));
                0
            }
        };

        let r_hash = match average_hash(&r_path) {
            Ok(hash) => {
                println!("{}", format!("{hash:X}"));
                hash
            }
            Err(err) => {
                eprintln!("{}", format!("{err}"));
                0
            }
        };

        println!("Distance: {}", distance(l_hash, r_hash));
    }

    #[test]
    fn test_hex_to_u64() {
        let hex = String::from("F4F9DB52780CDE9A");
        let result = match hex_to_u64(&hex) {
            Ok(v) => v,
            _ => 0,
        };
        assert_eq!(result, 17652381361703280282);
    }
}
