use super::transform_image as transform;
use anyhow::Result;
use image::{imageops::FilterType, DynamicImage};

/// difference_hash function returns a hash computation of difference hash.
/// Implementation follows
/// https://www.hackerfactor.com/blog/index.php?/archives/529-Kind-of-Like-That.html
#[allow(unused)]
pub fn difference_hash(img: DynamicImage) -> Result<u64> {
    let (w, h) = (9, 8);
    let img_resized = img.resize_exact(w, h, FilterType::Lanczos3);
    let img_buf = img_resized.to_luma8();
    let pixel_matrix = transform::image_to_pixel_matrix(&img_buf);
    let mut idx: u64 = 0;
    let mut hash: u64 = 0;

    let (w, h) = (w as usize, h as usize);

    for y in 0..h {
        for x in 0..w - 1 {
            if pixel_matrix[y][x] > pixel_matrix[y][x + 1] {
                hash |= 1 << (64 - idx - 1);
            }
            idx += 1;
        }
    }

    Ok(hash)
}

/// average_hash function returns a hash computation of average hash vertically.
/// Implementation follows
/// https://www.hackerfactor.com/blog/index.php?/archives/432-Looks-Like-It.html
#[allow(unused)]
pub fn average_hash(img: DynamicImage) -> Result<u64> {
    let (w, h) = (8, 8);
    let img_resized = img.resize_exact(w, h, FilterType::Lanczos3);
    let img_buf = img_resized.to_luma8();
    let pixel_matrix = transform::image_to_pixel_matrix(&img_buf);
    let mut float_pixels: Vec<f64> = Vec::with_capacity(64);
    let mut sum = 0.0;

    let (w, h) = (w as usize, h as usize);

    for y in 0..h {
        for x in 0..w {
            sum += pixel_matrix[y][x];
            float_pixels.push(pixel_matrix[y][x]);
        }
    }

    let avg = sum / 64 as f64;
    let mut idx: u64 = 0;
    let mut hash: u64 = 0;

    for p in float_pixels {
        if p > avg {
            hash |= 1 << (64 - idx - 1);
        }
        idx += 1;
    }

    Ok(hash)
}

#[allow(unused)]
#[derive(Debug)]
pub enum ColorType {
    Gray,
    Threshold,
}

/// perception_hash function returns a hash computation of perception hash.
/// Implementation follows
/// https://www.hackerfactor.com/blog/index.php?/archives/432-Looks-Like-It.html
#[allow(unused)]
pub fn perception_hash(img: DynamicImage, color_type: ColorType) -> Result<u64> {
    let (w, h) = (32, 32);
    let img_resized = img.resize_exact(w, h, FilterType::Lanczos3);
    let img_buf = img_resized.to_luma8();
    let pixels = match color_type {
        ColorType::Gray => transform::image_to_pixel_matrix(&img_buf),
        ColorType::Threshold => transform::image_to_threshold_matrix(&img_buf, 114),
    };
    let dct = transform::dct2d(&pixels, w as usize, h as usize);

    // Calculate the average of the dct.
    let (w, h) = (8, 8);
    let mut flat_dct: Vec<f64> = Vec::with_capacity(64);
    let mut sum = 0.0;

    for y in 0..h {
        for x in 0..w {
            sum += dct[y][x];
            flat_dct.push(dct[y][x]);
        }
    }

    // excluding the first term since the DC coefficient can be significantly different from the
    // other values and will throw off the average.
    sum -= dct[0][0];
    let avg = sum / 63 as f64;

    // extract the hash.
    let mut hash: u64 = 0;

    for (idx, p) in flat_dct.iter().enumerate() {
        if *p > avg {
            hash |= 1 << (64 - idx - 1);
        }
    }

    Ok(hash)
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::media;
    use std::path::Path;

    #[test]
    fn test_difference_hash() {
        let media_path = Path::new("D:/images_test/horse.jpg");
        let thumb_size = 240;

        match media::make_thumbnail_to_vec(media_path, thumb_size) {
            Ok((img, _)) => match difference_hash(img) {
                Ok(hash) => {
                    println!("D-HASH: {}", format!("{hash:X}"));
                    assert_ne!(hash, 0);
                }
                Err(err) => assert!(false, "{err}"),
            },
            Err(err) => assert!(false, "{err}"),
        }
    }

    #[test]
    fn test_average_hash() {
        let media_path = Path::new("D:/images_test/horse.jpg");
        let thumb_size = 240;

        match media::make_thumbnail_to_vec(media_path, thumb_size) {
            Ok((img, _)) => match average_hash(img) {
                Ok(hash) => {
                    println!("A-HASH: {}", format!("{hash:X}"));
                    assert_ne!(hash, 0);
                }
                Err(err) => assert!(false, "{err}"),
            },
            Err(err) => assert!(false, "{err}"),
        }
    }

    #[test]
    fn test_perception_hash() {
        let media_path = Path::new("D:/images_test/horse.jpg");
        let thumb_size = 240;

        match media::make_thumbnail_to_vec(media_path, thumb_size) {
            Ok((img, _)) => match perception_hash(img, ColorType::Gray) {
                Ok(hash) => {
                    println!("P-HASH: {}", format!("{hash:X}"));
                    assert_ne!(hash, 0);
                }
                Err(err) => assert!(false, "{err}"),
            },
            Err(err) => assert!(false, "{err}"),
        }
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
