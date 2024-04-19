use bytes::Bytes;
use chrono::prelude::*;

use crate::fl;
use core_chasam;

pub const ZOOM_SIZE: i32 = 20;
pub const ZOOM_LIMIT: i32 = 240;
pub const THUMBNAIL_SIZE: i32 = 160;

#[derive(Debug, Default, Clone, PartialEq)]
pub enum MediaType {
    #[default]
    Image,
    Video,
}

impl MediaType {
    pub fn name(&self) -> String {
        let image: &String = fl!("image");
        let video: &String = fl!("video");
        match self {
            Self::Image => image.clone(),
            Self::Video => video.clone(),
        }
    }
}

#[derive(Debug, Default, Clone)]
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
    pub is_selected: bool,
    pub thumbnail_size: i32,
}

impl From<&core_chasam::csam::Media> for Media {
    fn from(media: &core_chasam::csam::Media) -> Self {
        Self {
            name: media.name.clone(),
            path: media.path.clone(),
            media_type: match media.media_type {
                core_chasam::csam::MediaType::Image => MediaType::Image,
                core_chasam::csam::MediaType::Video => MediaType::Video,
            },
            size: media.size,
            last_modified: media.last_modified,
            hash: media.hash.clone(),
            phash: media.phash,
            match_type: media.match_type.clone(),
            hamming: media.hamming,
            data: media.data.clone(),
            is_selected: false,
            thumbnail_size: THUMBNAIL_SIZE,
        }
    }
}

impl Media {
    pub fn is_csam(&self) -> bool {
        !self.match_type.is_empty()
    }

    // pub fn is_image(&self) -> bool {
    //     match self.media_type {
    //         MediaType::Image => true,
    //         _ => false,
    //     }
    // }
}

#[derive(Debug, Default)]
pub struct MediaDetail {
    pub name: String,
    pub path: String,
    pub media_type: String,
    pub size: String,
    pub last_modified: String,
    pub hash: String,
    pub phash: String,
    pub match_type: String,
    pub hamming: String,
}

impl From<&Media> for MediaDetail {
    fn from(media: &Media) -> Self {
        let date_time = Local.timestamp_opt(media.last_modified, 0);

        Self {
            name: media.name.clone(),
            path: media.path.clone(),
            media_type: media.media_type.clone().name(),
            size: if media.size > 1024 {
                format!("{:.2} MB", (media.size / 1024) as f64)
            } else {
                format!("{} KB", media.size)
            },
            last_modified: if let Some(date_time) = date_time.single() {
                date_time.format("%d/%m/%Y %H:%M:%S").to_string()
            } else {
                String::new()
            },
            hash: media.hash.clone(),
            phash: format!("{:X}", media.phash),
            match_type: media.match_type.clone(),
            hamming: media.hamming.to_string(),
        }
    }
}

#[derive(Debug)]
pub struct MediaFilter {
    pub search_entry: Option<String>,
    pub is_image: bool,
    pub is_video: bool,
    pub is_csam: bool,
    pub is_size_0: bool,
    pub is_size_30: bool,
    pub is_size_100: bool,
    pub is_size_500: bool,
    pub is_size_greater_500: bool,
    pub hamming_distance: u32,
}

impl Default for MediaFilter {
    fn default() -> Self {
        Self {
            search_entry: None,
            is_image: true,
            is_video: true,
            is_csam: false,
            is_size_0: true,
            is_size_30: true,
            is_size_100: true,
            is_size_500: true,
            is_size_greater_500: true,
            hamming_distance: core_chasam::csam::Media::MAX_DISTANCE_HAMMING,
        }
    }
}
