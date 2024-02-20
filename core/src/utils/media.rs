use sha1::{Digest, Sha1};
use std::{fs, io, path::Path};

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
pub fn get_file_hash_sha1<P>(path: P) -> Result<String, io::Error>
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
pub fn get_file_hash_md5<P>(path: P) -> Result<String, io::Error>
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
pub fn get_file_perceptual_hash<P>(path: P) -> Result<u64, image::ImageError>
where
    P: AsRef<Path>,
{
    let mut hash: u64 = 0;
    if let Some(p) = path.as_ref().to_str() {
        hash = phash::difference_hash(p)?;
    }
    Ok(hash)
}
