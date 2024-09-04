use std::collections::HashSet;
use std::sync::RwLock;

use super::Repository;
use crate::utils;

#[derive(Debug, Default)]
pub struct InMemoryRepository {
    keyword_store: RwLock<HashSet<String>>,
    hash_store: RwLock<HashSet<String>>,
    phash_store: RwLock<Vec<u64>>,
}

impl InMemoryRepository {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Repository for InMemoryRepository {
    fn add_keyword(&self, keyword: &str) {
        let mut store = self.keyword_store.write().unwrap();
        store.insert(keyword.to_lowercase());
    }

    fn add_hash(&self, hash: &str) {
        let mut store = self.hash_store.write().unwrap();
        store.insert(hash.to_lowercase());
    }

    fn add_phash(&self, phash: u64) {
        let mut store = self.phash_store.write().unwrap();
        store.push(phash);
    }

    fn remove_all_keywords(&self) {
        self.keyword_store.write().unwrap().clear();
    }

    fn remove_all_hash(&self) {
        self.hash_store.write().unwrap().clear();
    }

    fn remove_all_phash(&self) {
        self.phash_store.write().unwrap().clear();
    }

    fn contains_keyword(&self, filename: &str) -> Option<String> {
        for keyword in self.keyword_store.read().unwrap().iter() {
            if filename.contains(keyword) {
                return Some(keyword.clone());
            }
        }
        None
    }

    fn contains_hash(&self, hash: &str) -> bool {
        self.hash_store.read().unwrap().contains(hash)
    }

    fn load_keywords(&self) -> Vec<String> {
        self.keyword_store
            .read()
            .unwrap()
            .iter()
            .map(|kw| kw.to_string())
            .collect::<Vec<String>>()
    }

    fn match_phash(&self, phash: u64, max_distance: u32) -> Option<u32> {
        let store = self.phash_store.read().unwrap();
        store
            .iter()
            .map(|l_phash| utils::phash::distance(*l_phash, phash))
            .filter(|&distance| distance <= max_distance)
            .min()
    }

    fn count_keyword(&self) -> usize {
        self.keyword_store.read().unwrap().len()
    }

    fn count_hash(&self) -> usize {
        self.hash_store.read().unwrap().len()
    }

    fn count_phash(&self) -> usize {
        self.phash_store.read().unwrap().len()
    }

    fn clear(&self) {
        self.keyword_store.write().unwrap().clear();
        self.hash_store.write().unwrap().clear();
        self.phash_store.write().unwrap().clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_contains_keyword_should_return_true() {
        let repo = InMemoryRepository::new();
        let filename = "File name 13 year old test xpto.";
        let keyword = "13 year old";
        repo.add_keyword(keyword);
        let result = repo.contains_keyword(filename);
        assert_eq!(result, Some(keyword.to_owned()));
    }

    #[test]
    fn test_contains_hash_should_return_true() {
        let repo = InMemoryRepository::new();
        let hash = "50cd5ed4af91a2723d14f8b9f4254b7d";
        repo.add_hash(hash);
        let result = repo.contains_hash(hash);
        assert_eq!(result, true);
    }

    #[test]
    fn test_match_phash_should_return_distance_equals_1() {
        let repo = InMemoryRepository::new();
        let phash_1: u64 = 15634510955120228568;
        repo.add_phash(phash_1);
        let phash_2: u64 = 15634510955120226520;
        let result = repo.match_phash(phash_2, 10);
        assert_ne!(result, None);

        if let Some(distance) = result {
            assert_eq!(distance, 1);
        }
    }
}
