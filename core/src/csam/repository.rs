pub const MAX_DISTANCE_HAMMING: u32 = 20;

pub trait Repository: Send + Sync {
    fn add_keyword(&self, keyword: &str);
    fn add_hash(&self, hash: &str);
    fn add_phash(&self, phash: u64);
    fn contains_keyword(&self, filename: &str) -> Option<String>;
    fn contains_hash(&self, hash: &str) -> bool;
    fn match_phash(&self, phash: u64, max_distance: u32) -> Option<u32>;
}
