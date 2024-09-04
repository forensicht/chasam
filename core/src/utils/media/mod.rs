pub mod decoder;

use anyhow::Result;
use image::DynamicImage;
use sha1::{Digest, Sha1};
use std::{fs, io::Cursor, path::Path};

use super::phash;

const MEDIA_TYPE_IMAGES: &[&str] = &["jpeg", "jpg", "png", "bmp", "tiff", "gif"];
const MEDIA_TYPE_VIDEOS: &[&str] = &["mpeg", "mpg", "mp4", "avi", "ogg", "webm", "flv"];

pub fn is_image(extension: &str) -> bool {
    MEDIA_TYPE_IMAGES.contains(&extension)
}

#[allow(unused)]
pub fn is_video(extension: &str) -> bool {
    MEDIA_TYPE_VIDEOS.contains(&extension)
}

pub fn is_media(extension: &str) -> bool {
    is_image(extension) || is_video(extension)
}

#[allow(unused)]
pub fn get_path_hash<P>(path: P) -> Option<String>
where
    P: AsRef<Path>,
{
    if let Some(p) = path.as_ref().to_str() {
        let hash = &hex::encode(Sha1::digest(p.as_bytes()));
        return Some(hash.to_owned());
    }
    None
}

#[allow(unused)]
pub fn get_sha1_hash_of_file<P>(path: P) -> Result<String>
where
    P: AsRef<Path>,
{
    if let Some(p) = path.as_ref().to_str() {
        let data = fs::read(p)?;
        let hash = &hex::encode(Sha1::digest(&data));
        return Ok(hash.to_owned());
    }

    Ok(String::new())
}

#[allow(unused)]
pub fn get_md5_hash_of_file<P>(path: P) -> Result<String>
where
    P: AsRef<Path>,
{
    if let Some(p) = path.as_ref().to_str() {
        let data = fs::read(p)?;
        let digest = md5::compute(data);
        return Ok(format!("{:x}", digest));
    }

    Ok(String::new())
}

#[allow(unused)]
pub fn get_perceptual_hash_of_file<P>(path: P) -> Result<u64>
where
    P: AsRef<Path>,
{
    let mut hash: u64 = 0;
    if let Some(p) = path.as_ref().to_str() {
        let img = image::open(path)?;
        hash = phash::perception_hash(img, phash::ColorType::Threshold)?;
    }
    Ok(hash)
}

#[allow(unused)]
pub fn get_perceptual_hash_of_image(img: DynamicImage) -> Result<u64> {
    phash::perception_hash(img, phash::ColorType::Threshold)
}

#[allow(unused)]
pub fn make_thumbnail<PA, PB>(media_path: PA, thumb_path: PB, thumb_size: u32) -> Result<bool>
where
    PA: AsRef<Path>,
    PB: AsRef<Path>,
{
    let img = image::open(media_path.as_ref())?;

    if img.width() > thumb_size || img.height() > thumb_size {
        img.thumbnail(thumb_size, thumb_size)
            .save(thumb_path.as_ref())?;
        Ok(true)
    } else {
        Ok(false)
    }
}

#[allow(unused)]
pub fn make_thumbnail_to_vec<P>(media_path: P, thumb_size: u32) -> Result<(DynamicImage, Vec<u8>)>
where
    P: AsRef<Path>,
{
    let mut buf = Vec::new();
    let img = image::open(media_path.as_ref())?;

    if img.width() > thumb_size || img.height() > thumb_size {
        let thumbnail = img.thumbnail(thumb_size, thumb_size);
        thumbnail.write_to(
            &mut Cursor::new(&mut buf),
            image::ImageOutputFormat::Jpeg(50),
        )?;
        Ok((thumbnail, buf))
    } else {
        img.write_to(
            &mut Cursor::new(&mut buf),
            image::ImageOutputFormat::Jpeg(50),
        )?;
        Ok((img, buf))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_image() {
        assert_eq!(is_image("jpeg"), true);
    }

    #[test]
    fn test_is_video() {
        assert_eq!(is_video("mp4"), true);
    }

    #[test]
    fn test_is_media() {
        assert_eq!(is_media("bmp"), true);
    }

    #[test]
    fn test_get_sha1_hash_of_file() {
        let path = Path::new("../data/img/horse.jpg");
        let hash = get_sha1_hash_of_file(path).expect("Failed to get sha1 hash of file.");

        // Assert
        assert_eq!(hash, "b7b6e21916253608c9ff081db046a58100536963");
    }

    #[test]
    fn test_get_md5_hash_of_file() {
        let path = Path::new("../data/img/horse.jpg");
        let hash = get_md5_hash_of_file(path).expect("Failed to get md5 hash of file.");

        // Assert
        assert_eq!(hash, "506bf7f41ca0c6f9e7612c04e93ab235");
    }

    #[test]
    fn test_make_thumbnail_to_vec() {
        let media_path = Path::new("../data/img/horse.jpg");
        let thumb_size = 240;
        let (_, buf) = make_thumbnail_to_vec(media_path, thumb_size)
            .expect("Failed to make thumbnail for vec.");

        // Assert
        assert_eq!(buf.len(), 8060);
    }
}
