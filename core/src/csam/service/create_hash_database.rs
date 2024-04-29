use std::path::PathBuf;

use super::Service;
use crate::csam::db;

impl Service {
    pub async fn create_hash_database(
        &self,
        db_path: PathBuf,
        root: PathBuf,
    ) -> anyhow::Result<usize> {
        *self.cancel_flag.write().unwrap() = false;
        let cancel_flag = self.cancel_flag.clone();
        let repo = self.repo.clone();

        tokio::task::spawn_blocking(move || {
            let count = db::create_hash_database(db_path.clone(), root, cancel_flag)?;
            db::load_hash_database(db_path, repo)?;
            Ok(count)
        })
        .await?
    }
}
