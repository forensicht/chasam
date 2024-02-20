use crate::utils;
use super::media::Media;

use std::path::{Path, PathBuf};
use std::sync::{Arc, RwLock};
use std::thread;
use tokio::sync::mpsc;
use threadpool::ThreadPool;
use walkdir::WalkDir;
use anyhow::{Result, Context};

#[derive(Debug)]
pub enum StateMedia {
    Ok(Media),
    Err(anyhow::Error),
    Running(bool),
    Found(usize),
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

    pub fn search(
        &self,
        dir: PathBuf,
        state_sender: mpsc::Sender<StateMedia>,
    ) -> Result<()> {
        state_sender.blocking_send(StateMedia::Running(true))
            .context("could not send `StateMedia::Running`")?;

        let c_stopped = self.stopped.clone();
        let c_state_sender = state_sender.clone();

        thread::spawn(move || {
            let mut found_files: usize = 0;
            let thread_pool = ThreadPool::new(num_cpus::get());

            for entry in WalkDir::new(dir)
                .follow_links(false)
                .into_iter()
                .filter_map(|e| e.ok())
                .filter(|e| !e.file_type().is_dir() && SearchMedia::is_media(e.path()))
            {
                if *c_stopped.read().unwrap() {
                    break;
                }

                found_files += 1;

                let entry = entry.clone();
                let c_stopped = c_stopped.clone();
                let c_state_sender = c_state_sender.clone();

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

            c_state_sender.blocking_send(StateMedia::Found(found_files))
                .expect("could not send `StateMedia::Found`");

            // wait for thread pool to process all jobs
            thread_pool.join();

            c_state_sender.blocking_send(StateMedia::Running(false))
                .expect("cound not send `StateMedia::Running`");
        });
        
        drop(state_sender);

        Ok(())
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
