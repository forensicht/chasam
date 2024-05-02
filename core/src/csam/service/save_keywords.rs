use std::path::PathBuf;

use super::Service;
use crate::csam::db;

impl Service {
    pub async fn save_keywords(&self, db_path: PathBuf, content: &str) -> anyhow::Result<()> {
        let repo = self.repo.clone();
        let content = content.to_string();

        tokio::task::spawn_blocking(move || {
            db::create_keyword_database(db_path.clone(), content.trim())?;
            db::load_keyword_database(db_path, repo)?;
            Ok(())
        })
        .await?
    }
}
