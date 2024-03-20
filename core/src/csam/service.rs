use anyhow::{Context, Result};
use std::path::PathBuf;
use std::sync::Arc;
use tokio::sync::mpsc::Sender;

use super::Repository;
use super::SearchMedia;
use super::StateMedia;
use crate::repository::CsamRepository;

pub struct Service {
    repository: Arc<dyn Repository>,
    search_media: SearchMedia,
}

impl Service {
    pub fn new() -> Self {
        let repository = Arc::new(CsamRepository::new());

        Service {
            repository: repository.clone(),
            search_media: SearchMedia::new(repository),
        }
    }

    pub async fn load_database(&self, db_path: PathBuf) -> Result<()> {
        let repository = self.repository.clone();

        tokio::task::spawn_blocking(move || {
            super::load_csam_database(db_path, repository)
                .with_context(|| "Could not load csam database.")?;
            Ok(())
        })
        .await?
    }

    pub fn start_search_media(&self, dir: PathBuf, state_sender: Sender<StateMedia>) {
        self.search_media.search(dir, state_sender);
    }

    pub fn stop_search_media(&self) {
        self.search_media.stop();
    }

    pub fn count_keyword(&self) -> usize {
        self.repository.count_keyword()
    }

    pub fn count_hash(&self) -> usize {
        self.repository.count_hash()
    }

    pub fn count_phash(&self) -> usize {
        self.repository.count_phash()
    }
}
