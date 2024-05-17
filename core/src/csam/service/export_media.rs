use std::path::{Component, PathBuf};
use std::sync::atomic::Ordering;

use super::Service;

impl Service {
    pub async fn export_media(&self, save_path: &PathBuf, medias: &[String]) -> anyhow::Result<()> {
        self.cancel_flag.store(false, Ordering::SeqCst);

        let export_path: Vec<(PathBuf, PathBuf)> = medias
            .into_iter()
            .map(|media| {
                let from_path = PathBuf::from(media);
                let mut to_path = save_path.to_owned();
                for component in from_path.components() {
                    match component {
                        Component::Normal(component) => to_path.push(component),
                        _ => (),
                    }
                }
                (from_path, to_path)
            })
            .collect();

        for (from_path, to_path) in export_path.iter() {
            if self.cancel_flag.load(Ordering::SeqCst) {
                break;
            }
            self.copy_media(from_path, to_path).await?;
        }

        Ok(())
    }

    async fn copy_media(&self, from_path: &PathBuf, to_path: &PathBuf) -> anyhow::Result<()> {
        let to_dir = to_path.parent().unwrap();
        if !to_dir.exists() {
            tokio::fs::create_dir_all(to_dir).await?;
        }
        tokio::fs::copy(from_path, to_path).await?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use super::*;
    use crate::csam::repository::InMemoryRepository;

    #[tokio::test]
    async fn test_should_export_media() {
        let repo = Arc::new(InMemoryRepository::new());
        let service = Service::new(repo);

        let save_path = PathBuf::from("D:/export/");
        let medias = vec![
            "D:/images_test/target/original/imgCH_640_007.jpg".to_string(),
            "D:/images_test/target/blur/imgCH_640_005.jpg".to_string(),
            "D:/images_test/target/gray/imgCH_640_004.jpg".to_string(),
            "D:/images_test/target/original/imgCH_640_006.jpg".to_string(),
        ];

        match service.export_media(&save_path, &medias).await {
            Ok(_) => assert!(true),
            Err(err) => assert!(false, "{err}"),
        }
    }
}
