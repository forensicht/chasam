use crate::utils;
use super::media::Media;

use std::path::{Path, PathBuf};
use std::sync::{Arc, RwLock};
use tokio::sync::mpsc::Sender;
use threadpool::ThreadPool;
use walkdir::WalkDir;
use anyhow::Result;

#[derive(Debug)]
pub enum StateMedia {
    Completed,
    Found(usize),
    Ok(Media),
    Err(anyhow::Error),
}

pub struct SearchMedia {
    stopped: Arc<RwLock<bool>>,
}

impl SearchMedia {
    pub fn new() -> Self {
        SearchMedia { 
            stopped: Arc::new(RwLock::new(false)),
        }
    }

    pub async fn search(
        &self,
        dir: PathBuf,
        state_sender: Sender<StateMedia>,
    ) -> Result<()> {
        let stopped = self.stopped.clone();
        let state_sender = state_sender.clone();

        tokio::task::spawn_blocking(move || {
            let mut found_files: usize = 0;
            let thread_pool = ThreadPool::new(num_cpus::get());

            for entry in WalkDir::new(dir)
                .follow_links(false)
                .into_iter()
                .filter_map(|e| e.ok())
                .filter(|e| !e.file_type().is_dir() && SearchMedia::is_media(e.path()))
            {
                if *stopped.read().unwrap() {
                    break;
                }

                found_files += 1;

                let entry = entry.clone();
                let c_stopped = stopped.clone();
                let c_state_sender = state_sender.clone();

                thread_pool.execute(move || {
                    if *c_stopped.read().unwrap() {
                        return;
                    }

                    match Media::new(entry) {
                        Ok(media) => {
                            c_state_sender.blocking_send(StateMedia::Ok(media))
                                .expect("could not send `StateMedia::Ok`");
                        }
                        Err(error) => {
                            c_state_sender.blocking_send(StateMedia::Err(error))
                                .expect("could not send `StateMedia::Err`");
                        }
                    }
                });
            }

            state_sender.blocking_send(StateMedia::Found(found_files))
                .expect("could not send `StateMedia::Found`");

            // wait for thread pool to process all jobs
            thread_pool.join();

            state_sender.blocking_send(StateMedia::Completed)
                .expect("could not send `StateMedia::Completed`");

            drop(state_sender);

            Ok(())
        }).await?
    }

    pub fn stop(&self) {
        *self.stopped.write().unwrap() = true;
    }

    fn is_media(entry: &Path) -> bool {
        match entry.extension() {
            Some(e) if utils::media::is_media(&e.to_string_lossy().to_lowercase()) => true,
            _ => false,
        }
    }
}
