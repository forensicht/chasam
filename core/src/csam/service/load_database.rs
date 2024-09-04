use std::path::PathBuf;

use super::Service;
use crate::csam::db;

impl Service {
    pub async fn load_database(&self, db_path: PathBuf) -> anyhow::Result<()> {
        let mut tasks = vec![];
        tasks.push(tokio::task::spawn_blocking({
            let db_path = db_path.clone();
            let repo = self.repo.clone();
            move || db::load_hash_database(db_path, repo)
        }));
        tasks.push(tokio::task::spawn_blocking({
            let db_path = db_path.clone();
            let repo = self.repo.clone();
            move || db::load_keyword_database(db_path, repo)
        }));
        tasks.push(tokio::task::spawn_blocking({
            let repo = self.repo.clone();
            move || db::load_phash_database(db_path, repo)
        }));

        match futures::future::try_join_all(tasks).await {
            Ok(res) => {
                if let Some(err) = res.into_iter().find_map(|r| r.err()) {
                    anyhow::bail!("Could not load csam database. Error: {}", err);
                }
            }
            Err(err) => anyhow::bail!("Could not load csam database. Error: {}", err),
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use super::*;
    use crate::csam::repository::InMemoryRepository;

    #[tokio::test]
    async fn test_should_load_csam_database() {
        let db_path = PathBuf::from("../data/db/");
        let repo = Arc::new(InMemoryRepository::new());
        let service = Service::new(repo);
        service
            .load_database(db_path)
            .await
            .expect("Failed to load database.");

        // Assert
        assert!(true);
    }
}
