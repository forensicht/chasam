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
enum MatchType {
    MD5,
    PHash(u64, u32),
    Keyword(String),
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
    pub img_buf: Option<Bytes>,
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
        let md5_hash = utils::media::get_md5_hash_of_file(&media_path).unwrap_or_default();

        // make thumbnail
        let (dynamic_img, img_buf) = match media_type {
            MediaType::Image => {
                match utils::media::make_thumbnail_to_vec(&media_path, Self::THUMBNAIL_SIZE) {
                    Ok((img, buf)) => (Some(vec![img]), Some(Bytes::from(buf))),
                    Err(err) => {
                        tracing::error!("{} : {}", media_path.as_str(), err);
                        (None, None)
                    }
                }
            }
            MediaType::Video => match utils::media::decoder::make_thumbnail_to_vec(&media_path) {
                Ok((imgs, buf)) => (Some(imgs), Some(Bytes::from(buf))),
                Err(err) => {
                    tracing::error!("{} : {}", media_path.as_str(), err);
                    (None, None)
                }
            },
        };

        // perceptual hash of the file
        #[allow(clippy::unnecessary_unwrap)]
        let phash_vec = if media_size > 0 && dynamic_img.is_some() {
            dynamic_img
                .unwrap()
                .into_iter()
                .map(|img| {
                    utils::media::get_perceptual_hash_of_image(img)
                        .with_context(|| "could not generate perceptual hash")
                })
                .collect::<anyhow::Result<Vec<u64>>>()
        } else {
            Ok(vec![0])
        };

        // checks if the media is in the CSAM database
        let (phash, match_type, distance_hamming) = {
            let phash_vec = phash_vec?;

            if phash_vec.len() > 0 {
                let phash = phash_vec[0];
                match Media::find_csam(repo.clone(), &name, &md5_hash, &phash_vec) {
                    Some(match_type) => match match_type {
                        MatchType::MD5 => (phash, String::from("MD5"), 0),
                        MatchType::Keyword(keyword) => (phash, format!("Keyword [ {keyword} ]"), 0),
                        MatchType::PHash(phash, distance_hamming) => (
                            phash,
                            format!("PHash [ {distance_hamming} ]"),
                            distance_hamming,
                        ),
                    },
                    None => (0u64, String::new(), 0u32),
                }
            } else {
                (0u64, String::new(), 0u32)
            }
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
            img_buf,
        };

        Ok(media)
    }

    fn find_csam(
        repo: Arc<dyn Repository>,
        name: &str,
        hash: &str,
        phash_vec: &[u64],
    ) -> Option<MatchType> {
        if Media::find_csam_by_hash(repo.clone(), hash) {
            return Some(MatchType::MD5);
        }

        if let Some(keyword) = Media::find_csam_by_keyword(repo.clone(), name) {
            return Some(MatchType::Keyword(keyword));
        }

        if phash_vec.len() == 0 {
            return None;
        }

        if phash_vec.len() == 1 {
            let phash = phash_vec[0];
            if let Some(distance_hamming) = Media::find_csam_by_phash(repo.clone(), phash) {
                return Some(MatchType::PHash(phash, distance_hamming));
            }
        } else {
            let phash = phash_vec
                .into_iter()
                .map(
                    |&phash| match Media::find_csam_by_phash(repo.clone(), phash) {
                        Some(distance_hamming) => (phash, distance_hamming),
                        None => (0u64, 0u32),
                    },
                )
                .filter(|phash| phash.0 != 0 && phash.1 > 0)
                .reduce(|lhs, rhs| if lhs.1 < rhs.1 { lhs } else { rhs })
                .unwrap_or((0u64, 0));

            return Some(MatchType::PHash(phash.0, phash.1));
        }

        None
    }

    fn find_csam_by_hash(repo: Arc<dyn Repository>, hash: &str) -> bool {
        repo.contains_hash(hash)
    }

    fn find_csam_by_keyword(repo: Arc<dyn Repository>, name: &str) -> Option<String> {
        repo.contains_keyword(name)
    }

    fn find_csam_by_phash(repo: Arc<dyn Repository>, phash: u64) -> Option<u32> {
        repo.match_phash(phash, Media::MAX_DISTANCE_HAMMING)
    }
}
