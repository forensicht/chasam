use super::Service;

use std::path::PathBuf;

use crate::csam::db;

impl Service {
    pub async fn load_database(&self, db_path: PathBuf) -> anyhow::Result<()> {
        let repo = self.repo.clone();
        db::load_csam_database(db_path, repo).await
    }
}
