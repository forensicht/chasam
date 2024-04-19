use super::repository::Repository;
use crate::utils;

use anyhow::{Context, Result};
use bytes::Bytes;
use chrono::Local;
use std::sync::Arc;
use walkdir::DirEntry;

#[derive(Debug, Clone, Copy, PartialEq)]
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
    pub data: Option<Bytes>,
}

impl Media {
    pub const THUMBNAIL_SIZE: u32 = 240;
    pub const MAX_DISTANCE_HAMMING: u32 = 20;

    pub fn new(repo: Arc<dyn Repository>, entry: DirEntry) -> Result<Self> {
        let metadata = entry
            .metadata()
            .with_context(|| "could not get file metadata")?;
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
                .with_context(|| "could not get attribute `modified` from metadata")?
                .elapsed()
                .with_context(|| "could not get elapsed time from metadata")?
                .as_secs() as i64;

        // get the md5 hash of the file
        let md5_hash = utils::media::get_file_hash_md5(&media_path).unwrap_or_default();

        // make thumbnail
        let (dynamic_img, img_data) = match media_type {
            MediaType::Image => {
                match utils::media::make_thumbnail_to_vec(&media_path, Self::THUMBNAIL_SIZE) {
                    Ok((img, buf)) => (Some(img), Some(Bytes::from(buf))),
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
            utils::media::get_image_perceptual_hash(dynamic_img.unwrap())
                .with_context(|| "could not generate perceptual hash")?
        } else {
            0
        };

        // checks if the media is in the CSAM database
        let (match_type, distance_hamming) =
            match Media::find_csam(repo.clone(), &name, &md5_hash, phash, media_type) {
                Some((match_type, distance_hamming)) => (match_type, distance_hamming),
                None => (String::new(), 0u32),
            };

        let media = Media {
            name,
            path: media_path,
            media_type,
            size: media_size,
            last_modified: media_last_modified,
            hash: md5_hash,
            phash,
            match_type,
            hamming: distance_hamming,
            data: img_data,
        };

        Ok(media)
    }

    fn find_csam(
        repo: Arc<dyn Repository>,
        name: &str,
        hash: &str,
        phash: u64,
        media_type: MediaType,
    ) -> Option<(String, u32)> {
        if let Some(hash) = Media::find_csam_by_hash(repo.clone(), hash) {
            return Some((hash, 0));
        }
        if let Some(keyword) = Media::find_csam_by_keyword(repo.clone(), name) {
            return Some((keyword, 0));
        }
        if media_type == MediaType::Video {
            return None;
        }
        if phash == 0 {
            return None;
        }
        if let Some(distance) = Media::find_csam_by_phash(repo.clone(), phash) {
            return Some(("chHash".to_string(), distance));
        }
        None
    }

    fn find_csam_by_hash(repo: Arc<dyn Repository>, hash: &str) -> Option<String> {
        if repo.contains_hash(hash) {
            return Some("MD5".to_string());
        }
        None
    }

    fn find_csam_by_keyword(repo: Arc<dyn Repository>, name: &str) -> Option<String> {
        if let Some(keyword) = repo.contains_keyword(name) {
            return Some(format!("keyword [{}]", keyword));
        }
        None
    }

    fn find_csam_by_phash(repo: Arc<dyn Repository>, phash: u64) -> Option<u32> {
        repo.match_phash(phash, Media::MAX_DISTANCE_HAMMING)
    }
}
