use super::Service;

use anyhow::{Context, Result};
use std::path::PathBuf;

use crate::csam::db;

impl Service {
    pub async fn load_database(&self, db_path: PathBuf) -> Result<()> {
        let repo = self.repo.clone();

        tokio::task::spawn_blocking(move || {
            db::load_csam_database(db_path, repo)
                .with_context(|| "Could not load csam database.")?;
            Ok(())
        })
        .await?
    }
}
