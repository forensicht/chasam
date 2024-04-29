use std::sync::{Arc, RwLock};

use super::repository::Repository;

mod create_hash_database;
mod create_phash_database;
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

    pub async fn count_keyword(&self) -> usize {
        self.repo.count_keyword()
    }

    pub async fn count_hash(&self) -> usize {
        self.repo.count_hash()
    }

    pub async fn count_phash(&self) -> usize {
        self.repo.count_phash()
    }

    pub fn cancel_task(&self) {
        *self.cancel_flag.write().unwrap() = true;
    }
}
