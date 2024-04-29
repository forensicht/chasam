pub use in_memory_repository::InMemoryRepository;

mod in_memory_repository;

pub trait Repository: Send + Sync {
    fn add_keyword(&self, keyword: &str);
    fn add_hash(&self, hash: &str);
    fn add_phash(&self, phash: u64);
    fn remove_all_keywords(&self);
    fn remove_all_hash(&self);
    fn remove_all_phash(&self);
    fn contains_keyword(&self, filename: &str) -> Option<String>;
    fn contains_hash(&self, hash: &str) -> bool;
    fn match_phash(&self, phash: u64, max_distance: u32) -> Option<u32>;
    fn count_keyword(&self) -> usize;
    fn count_hash(&self) -> usize;
    fn count_phash(&self) -> usize;
    fn clear(&self);
}
