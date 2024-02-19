pub const ZOOM_SIZE: i32 = 32;
pub const THUMBNAIL_SIZE: i32 = 160;

#[derive(Debug, Default, Clone)]
pub struct Media {
    pub name: String,
    pub path: String,
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
