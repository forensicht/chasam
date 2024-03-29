use std::collections::HashSet;
use std::sync::RwLock;

use crate::csam;
use crate::utils;

#[derive(Debug)]
pub struct CsamRepository {
    keyword_store: RwLock<HashSet<String>>,
    hash_store: RwLock<HashSet<String>>,
    phash_store: RwLock<Vec<u64>>,
}

impl CsamRepository {
    pub fn new() -> Self {
        let keyword_store: RwLock<HashSet<String>> = RwLock::new(HashSet::new());
        let hash_store: RwLock<HashSet<String>> = RwLock::new(HashSet::new());
        let phash_store: RwLock<Vec<u64>> = RwLock::new(vec![]);
        Self {
            keyword_store,
            hash_store,
            phash_store,
        }
    }
}

impl csam::Repository for CsamRepository {
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

    fn match_phash(&self, phash: u64, max_distance: u32) -> Option<u32> {
        let store = self.phash_store.read().unwrap();
        store
            .iter()
            .map(|l_phash| utils::phash::distance(*l_phash, phash))
            .filter(|&distance| distance <= max_distance)
            .min()
    }
}

#[cfg(test)]
mod tests {
    use self::csam::Repository;

    use super::*;

    #[test]
    fn test_contains_keyword_should_return_true() {
        let repo = CsamRepository::new();
        let filename = "File name 13 year old test xpto.";
        let keyword = "13 year old";
        repo.add_keyword(keyword);
        let result = repo.contains_keyword(filename);
        assert_eq!(result, Some(keyword.to_owned()));
    }

    #[test]
    fn test_contains_hash_should_return_true() {
        let repo = CsamRepository::new();
        let hash = "cd63c80d3ad93dde00213e9b7a621513519c0d90";
        repo.add_hash(hash);
        let result = repo.contains_hash(hash);
        assert_eq!(result, true);
    }

    #[test]
    fn test_match_phash_should_return_distance_equals_1() {
        let repo = CsamRepository::new();
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
