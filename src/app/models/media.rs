use core_chasam as core;

pub const ZOOM_SIZE: i32 = 32;
pub const THUMBNAIL_SIZE: i32 = 160;

#[derive(Debug, Default, Clone)]
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
    pub is_selected: bool,
    pub thumbnail_size: i32,
}

impl From<&core::csam::Media> for Media {
    fn from(media: &core::csam::Media) -> Self {
        Self {
            name: media.name.clone(),
            path: media.path.clone(),
            thumb_path: media.thumb_path.clone(),
            media_type: media.media_type.clone(),
            size: media.size,
            last_modified: media.last_modified,
            hash: media.hash.clone(),
            phash: media.phash,
            match_type: media.match_type.clone(),
            hamming: media.hamming,
            is_selected: false,
            thumbnail_size: THUMBNAIL_SIZE,
        }
    }
}

#[derive(Debug)]
pub struct MediaFilter {
    pub search_entry: Option<String>,
    pub size_0: bool,
    pub size_30: bool,
    pub size_100: bool,
    pub size_500: bool,
    pub size_greater_500: bool,
}

impl Default for MediaFilter {
    fn default() -> Self {
        Self {
            search_entry: None,
            size_0: true,
            size_30: true,
            size_100: true,
            size_500: true,
            size_greater_500: true,
        }
    }
}
