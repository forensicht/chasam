use std::path::PathBuf;
use std::sync::atomic::Ordering;

use super::Service;
use crate::csam::db;

impl Service {
    pub async fn create_phash_database(
        &self,
        db_path: PathBuf,
        root: PathBuf,
    ) -> anyhow::Result<usize> {
        self.cancel_flag.store(false, Ordering::SeqCst);
        let cancel_flag = self.cancel_flag.clone();
        let repo = self.repo.clone();

        tokio::task::spawn_blocking(move || {
            let count_before = repo.count_phash();
            let _ = db::create_phash_database(db_path.clone(), root, cancel_flag)?;
            db::load_phash_database(db_path, repo.clone())?;
            let count_after = repo.count_phash();
            Ok(count_after - count_before)
        })
        .await?
    }
}
