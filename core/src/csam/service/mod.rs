use std::sync::{Arc, RwLock};

use super::repository::Repository;

mod load_database;
mod search_media;
pub use search_media::*;

pub struct Service {
    repo: Arc<dyn Repository>,
    cancel_flag: Arc<RwLock<bool>>,
}

impl Service {
    pub fn new(repo: Arc<dyn Repository>) -> Self {
        Service {
            repo,
            cancel_flag: Arc::new(RwLock::new(false)),
        }
    }

    pub fn count_keyword(&self) -> usize {
        self.repo.count_keyword()
    }

    pub fn count_hash(&self) -> usize {
        self.repo.count_hash()
    }

    pub fn count_phash(&self) -> usize {
        self.repo.count_phash()
    }
}
