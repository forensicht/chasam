use image::DynamicImage;
use sha1::{Digest, Sha1};
use std::{
    fs, 
    io::Cursor, 
    path::Path,
};
use anyhow::Result;

use super::phash;

const MEDIA_TYPE_IMAGES: &[&str] = &["jpeg", "jpg", "png", "bmp", "tiff", "gif"];

const MEDIA_TYPE_VIDEOS: &[&str] = &[
    "mpeg", "mpg", "mp4", "avi", "ogg", "webm", "flv",
];

pub fn is_image(extension: &str) -> bool {
    MEDIA_TYPE_IMAGES.contains(&extension)
}

pub fn is_video(extension: &str) -> bool {
    MEDIA_TYPE_VIDEOS.contains(&extension)
}

pub fn is_media(extension: &str) -> bool {
    // is_image(extension) || is_video(extension)
    is_image(extension)
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
pub fn get_file_hash_sha1<P>(path: P) -> Result<String>
where
    P: AsRef<Path>,
{
    if let Some(p) = path.as_ref().to_str() {
        let data = fs::read(&p)?;
        let hash = &hex::encode(Sha1::digest(&data));
        return Ok(hash.to_owned());
    }

    Ok(String::new())
}

#[allow(unused)]
pub fn get_file_hash_md5<P>(path: P) -> Result<String>
where 
    P: AsRef<Path>,
{
    if let Some(p) = path.as_ref().to_str() {
        let data = fs::read(&p)?;
        let digest = md5::compute(data);
        return Ok(format!("{:x}", digest));
    }

    Ok(String::new())
}


#[allow(unused)]
pub fn get_file_perceptual_hash<P>(path: P) -> Result<u64>
where
    P: AsRef<Path>,
{
    let mut hash: u64 = 0;
    if let Some(p) = path.as_ref().to_str() {
        hash = phash::difference_hash(p)?;
    }
    Ok(hash)
}

#[allow(unused)]
pub fn get_image_perceptual_hash(img: DynamicImage, data: &[u8]) -> Result<u64> {
    phash::difference_hash_raw(img, data)
}

#[allow(unused)]
pub fn make_thumbnail<PA, PB>(
    media_path: PA,
    thumb_path: PB,
    thumb_size: u32,
) -> Result<bool>
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
pub fn make_thumbnail_to_vec<P>(
    media_path: P, 
    thumb_size: u32,
) -> Result<(DynamicImage, Vec<u8>)>
where
    P: AsRef<Path>, 
{   
    let mut buf = Vec::new();
    let img = image::open(media_path.as_ref())?;

    if img.width() > thumb_size || img.height() > thumb_size {
        let thumbnail = img.thumbnail(thumb_size, thumb_size);
        thumbnail.write_to(&mut Cursor::new(&mut buf), image::ImageOutputFormat::Jpeg(50))?;
        Ok((thumbnail, buf))
    } else {
        img.write_to(&mut Cursor::new(&mut buf), image::ImageOutputFormat::Jpeg(50))?;
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
    fn test_get_file_hash_sha1() {
        let path = Path::new("D:/images_test/horse.jpg");
        let hash = get_file_hash_sha1(path).unwrap_or_default();
        assert_ne!(hash, "");
    }

    #[test]
    fn test_get_file_hash_md5() {
        let path = Path::new("D:/images_test/horse.jpg");
        let hash = get_file_hash_md5(path).unwrap_or_default();
        assert_ne!(hash, "");
    }

    #[test]
    fn test_make_thumbnail() {
        let media_path = Path::new("D:/images_test/horse.jpg");
        let thumb_path = Path::new("D:/images_test/horse_thumb.jpg");
        let thumb_size = 320;
        
        match make_thumbnail(media_path, thumb_path, thumb_size) {
            Ok(_) => assert!(true),
            Err(err) => assert!(false, "{err}"),
        }
    }

    #[test]
    fn test_make_thumbnail_to_vec() {
        let media_path = Path::new("D:/images_test/horse.jpg");
        let thumb_size = 240;

        match make_thumbnail_to_vec(media_path, thumb_size) {
            Ok((_, buf)) => {
                let len = buf.len();
                println!("image bytes: {}", len);
                assert_ne!(len, 0);
            }
            Err(err) => assert!(false, "{err}"),
        }
    }
}
