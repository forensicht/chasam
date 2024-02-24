use crate::utils;

use std::path::PathBuf;
use chrono::Local;
use walkdir::DirEntry;
use anyhow::{Result, Context};

pub const THUMBNAIL_SIZE: u32 = 320;

pub fn get_thumb_path() -> Result<PathBuf> {
    // let path = std::env::current_dir()
    //     .context("could not get current dir")?
    //     .join("thumbnails");

    let path = PathBuf::new().join("C:\\thumbnails");
    Ok(path)
}

#[derive(Debug, Clone)]
pub struct Media {
    pub name: String,
    pub path: String,
    pub thumb_path: String,
    pub media_type: String,
    pub size: usize,
    pub last_modified: i64,
    pub hash: String,
    pub phash: u64,
    pub match_type: String,
    pub hamming: u32,
}

impl Media {
    pub fn new(entry: DirEntry) -> Result<Self> {
        let metadata = entry.metadata().context("could not get file metadata")?;
        let name = entry.file_name().to_str().unwrap().to_owned();
        let media_path = entry.path().to_str().unwrap().to_owned();

        // get the media type
        let media_type = match entry.path().extension() {
            Some(e) if utils::media::is_image(&e.to_string_lossy().to_lowercase()) => {
                String::from("image")
            }
            _ => String::from("video"),
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
        let thumb_path = media_path.clone();

        // if media_type == "image" {
        //     let media_path_hash = utils::media::get_path_hash(&media_path).unwrap();
        //     let mut thumb_path_buf = get_thumb_path()?.join(&media_path_hash);
        //     thumb_path_buf.set_extension("jpeg");

        //     if thumb_path_buf.exists() {
        //         thumb_path = thumb_path_buf.display().to_string();
        //     } else {
        //         match utils::media::make_thumbnail(entry.path(), &thumb_path_buf, THUMBNAIL_SIZE) {
        //             Ok(result) => if result {
        //                 thumb_path = thumb_path_buf.display().to_string();
        //             }
        //             Err(err) => {
        //                 eprintln!("[ERROR] {} : {}", media_path.as_str(), err);
        //             }
        //         }
        //     }
        // }

        // perceptual hash of the file
        let phash = if media_size > 0 {
            utils::media::get_file_perceptual_hash(&thumb_path)
                .context("could not generate perceptual hash")?
        } else {
            0
        };

        let media = Media {
            name: name,
            path: media_path,
            thumb_path: thumb_path,
            media_type: media_type,
            size: media_size,
            last_modified: media_last_modified,
            hash: md5_hash,
            phash: phash,
            match_type: String::new(),
            hamming: 0,
        };

        Ok(media)
    } 
}
