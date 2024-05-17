use std::path::{Path, PathBuf};
use std::sync::atomic::Ordering;
use threadpool::ThreadPool;
use tokio::sync::mpsc::{self, Receiver, Sender};
use walkdir::WalkDir;

use super::Service;
use crate::csam::media::Media;
use crate::utils;

#[derive(Debug)]
pub enum StateMedia {
    Completed,
    Found(usize),
    Ok(Vec<Media>),
    Err(anyhow::Error),
}

impl Service {
    pub fn search_media(&self, dir: PathBuf, state_sender: Sender<StateMedia>) {
        self.cancel_flag.store(false, Ordering::SeqCst);
        let cancel_flag = self.cancel_flag.clone();
        let repo = self.repo.clone();
        let state_sender = state_sender.clone();

        std::thread::spawn(move || {
            let (media_sender, media_receiver) = mpsc::channel::<Media>(1000);

            // Asyncronous function responsible for notifying the search result.
            Self::notify_result(media_receiver, state_sender.clone());

            let mut found_files: usize = 0;
            let thread_pool = ThreadPool::new(num_cpus::get());

            for entry in WalkDir::new(dir)
                .follow_links(false)
                .into_iter()
                .filter_map(|e| e.ok())
                .filter(|e| !e.file_type().is_dir() && Service::is_media(e.path()))
            {
                if cancel_flag.load(Ordering::SeqCst) {
                    break;
                }

                found_files += 1;

                let c_stop_flag = cancel_flag.clone();
                let c_repo = repo.clone();
                let c_media_sender = media_sender.clone();
                let c_state_sender = state_sender.clone();

                thread_pool.execute(move || {
                    if c_stop_flag.load(Ordering::SeqCst) {
                        return;
                    }

                    match Media::new(c_repo, entry) {
                        Ok(media) => {
                            c_media_sender
                                .blocking_send(media)
                                .expect("could not send `Media`");
                        }
                        Err(err) => {
                            c_state_sender
                                .blocking_send(StateMedia::Err(err))
                                .expect("could not send `StateMedia::Err`");
                        }
                    }
                });
            }

            state_sender
                .blocking_send(StateMedia::Found(found_files))
                .expect("could not send `StateMedia::Found`");

            // wait for thread pool to process all jobs
            thread_pool.join();

            state_sender
                .blocking_send(StateMedia::Completed)
                .expect("could not send `StateMedia::Completed`");

            drop(media_sender);
            drop(state_sender);
        });
    }

    // Asyncronous function responsible for notifying the search result.
    fn notify_result(mut media_receiver: Receiver<Media>, state_sender: Sender<StateMedia>) {
        std::thread::spawn(move || {
            let mut count = 0;
            let mut vec_medias: Vec<Media> = vec![];
            vec_medias.reserve(100);

            while let Some(media) = media_receiver.blocking_recv() {
                vec_medias.push(media);

                if count < 100 {
                    count += 1;
                } else {
                    state_sender
                        .blocking_send(StateMedia::Ok(vec_medias.clone()))
                        .expect("could not send `StateMedia::Ok`");
                    vec_medias.clear();
                    count = 0;
                }
            }

            if count > 0 {
                state_sender
                    .blocking_send(StateMedia::Ok(vec_medias.clone()))
                    .expect("could not send `StateMedia::Ok`");
            }

            drop(state_sender);
        });
    }

    #[inline]
    fn is_media(entry: &Path) -> bool {
        match entry.extension() {
            Some(e) if utils::media::is_media(&e.to_string_lossy().to_lowercase()) => true,
            _ => false,
        }
    }
}
