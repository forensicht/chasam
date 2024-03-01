use crate::utils;

use anyhow::{Context, Result};
use chrono::Local;
use walkdir::DirEntry;

pub const THUMBNAIL_SIZE: u32 = 240;

#[derive(Debug, Clone)]
pub enum MediaType {
    Image,
    Video,
}

#[derive(Debug, Clone)]
pub struct Media {
    pub name: String,
    pub path: String,
    pub media_type: MediaType,
    pub size: usize,
    pub last_modified: i64,
    pub hash: String,
    pub phash: u64,
    pub match_type: String,
    pub hamming: u32,
    pub data: Option<Vec<u8>>,
}

impl Media {
    pub fn new(entry: DirEntry) -> Result<Self> {
        let metadata = entry.metadata().context("could not get file metadata")?;
        let name = entry.file_name().to_str().unwrap().to_owned();
        let media_path = entry.path().to_str().unwrap().to_owned();

        // get the media type
        let media_type = match entry.path().extension() {
            Some(e) if utils::media::is_image(&e.to_string_lossy().to_lowercase()) => {
                MediaType::Image
            }
            _ => MediaType::Video,
        };
        // get the media size
        let media_size = (metadata.len() as f64 / 1024.0_f64).round() as usize;
        // get the last modification date
        let media_last_modified = Local::now().timestamp()
            - metadata
                .modified()
                .context("could not get attribute `modified` from metadata")?
                .elapsed()
                .context("could not get elapsed time from metadata")?
                .as_secs() as i64;

        // get the md5 hash of the file
        let md5_hash = utils::media::get_file_hash_md5(&media_path).unwrap_or_default();

        // make thumbnail
        let (dynamic_img, img_data) = match media_type {
            MediaType::Image => {
                match utils::media::make_thumbnail_to_vec(&media_path, THUMBNAIL_SIZE) {
                    Ok((img, buf)) => (Some(img), Some(buf)),
                    Err(err) => {
                        tracing::error!("{} : {}", media_path.as_str(), err);
                        (None, None)
                    }
                }
            }
            MediaType::Video => (None, None),
        };

        // perceptual hash of the file
        let phash = if media_size > 0 {
            if let Some(data) = img_data.as_ref() {
                utils::media::get_image_perceptual_hash(dynamic_img.unwrap(), data)
                    .context("could not generate perceptual hash")?
            } else {
                0
            }
        } else {
            0
        };

        let media = Media {
            name,
            path: media_path,
            media_type,
            size: media_size,
            last_modified: media_last_modified,
            hash: md5_hash,
            phash,
            match_type: String::new(),
            hamming: 0,
            data: img_data,
        };

        Ok(media)
    }
}
