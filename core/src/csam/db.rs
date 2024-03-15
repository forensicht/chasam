use anyhow::{bail, Context, Result};
use std::fs::File;
use std::fs::{self, OpenOptions};
use std::io;
use std::io::prelude::*;
use std::path::{Path, PathBuf};
use std::sync::{
    mpsc,
    mpsc::{Receiver, Sender},
    Arc, RwLock,
};
use std::thread::{self, JoinHandle};
use threadpool::ThreadPool;
use walkdir::WalkDir;

use crate::utils;

use super::Repository;

const FILE_HASH: &str = "hash.txt";
const FILE_KEYWORD: &str = "keyword.txt";
const FILE_PHASH: &str = "phash.txt";

pub fn create_phash_database<P>(
    db_path: PathBuf,
    root: P,
    progress_sender: Sender<usize>,
    stopped: Arc<RwLock<bool>>,
) -> Result<usize>
where
    P: AsRef<Path>,
{
    let (phash_sender, phash_receiver) = mpsc::channel::<String>();

    write_phash_database(db_path, phash_receiver, progress_sender)
        .with_context(|| "Could not create perceptual hash database.")?;

    {
        *stopped.write().unwrap() = false;
    }

    let mut count_files: usize = 0;

    let cpus = num_cpus::get();
    let thread_pool = ThreadPool::new(cpus);

    for entry in WalkDir::new(root)
        .follow_links(false)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| !e.file_type().is_dir() && self::is_image(e.path()))
    {
        {
            if *stopped.read().unwrap() {
                break;
            }
        }

        count_files += 1;
        let stopped = stopped.clone();
        let c_phash_sender = phash_sender.clone();

        thread_pool.execute(move || {
            {
                if *stopped.read().unwrap() {
                    return;
                }
            }

            match utils::media::get_file_perceptual_hash(entry.path()) {
                Ok(hash) => {
                    c_phash_sender
                        .send(hash.to_string())
                        .expect("could not send phash");
                }
                Err(err) => tracing::error!(
                    "Could not generate perceptual hash. {}\nError: {}",
                    entry.path().display(),
                    err
                ),
            }
        });
    }

    drop(phash_sender);

    Ok(count_files)
}

// Worker writing the perceptual hashes to the file.
fn write_phash_database(
    db_path: PathBuf,
    phash_receiver: Receiver<String>,
    progress_sender: Sender<usize>,
) -> Result<()> {
    if !db_path.exists() {
        fs::create_dir_all(&db_path)
            .with_context(|| format!("Could not create `{}` path", db_path.display()))?;
    }

    let phash_path = db_path.join(FILE_PHASH);
    let file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(&phash_path)
        .with_context(|| format!("Could not open file phash: {}", phash_path.display()))?;

    thread::spawn(move || {
        let mut writer = io::BufWriter::new(file);
        let mut hash = String::new();
        let mut count_files: usize = 0;

        for phash in phash_receiver.iter() {
            hash.clear();
            hash.push('\n');
            hash.push_str(&phash);

            match writer.write_all(hash.as_bytes()) {
                Ok(_) => {
                    count_files += 1;
                    progress_sender
                        .send(count_files)
                        .expect("could not send progress");
                }
                Err(err) => {
                    tracing::error!("Could not write perceptual hash in file.\nError: {:?}", err)
                }
            }
        }

        match writer.flush() {
            Err(err) => {
                tracing::error!("Could not write perceptual hash in file.\nError: {:?}", err)
            }
            _ => {}
        }

        drop(progress_sender);
    });

    Ok(())
}

fn is_image(entry: &Path) -> bool {
    match entry.extension() {
        Some(e) if utils::media::is_image(&e.to_string_lossy().to_lowercase()) => true,
        _ => false,
    }
}

pub fn load_csam_database(database_path: PathBuf, repository: Arc<dyn Repository>) -> Result<()> {
    repository.clear();

    let path = database_path.join(FILE_HASH);
    let work_hash = load_hash_database(path, repository.clone());

    let path = database_path.join(FILE_KEYWORD);
    let work_keyword = load_keyword_database(path, repository.clone());

    let path = database_path.join(FILE_PHASH);
    let work_phash = load_phash_database(path, repository.clone());

    match work_hash.join() {
        Err(_) => bail!("Could not load CSAM hash database."),
        _ => {}
    }

    match work_keyword.join() {
        Err(_) => bail!("Could not load CSAM keyword database."),
        _ => {}
    }

    match work_phash.join() {
        Err(_) => bail!("Could not load CSAM phash database."),
        _ => {}
    }

    Ok(())
}

fn load_hash_database(path: PathBuf, repository: Arc<dyn Repository>) -> JoinHandle<()> {
    thread::spawn(move || match File::open(&path) {
        Ok(file) => {
            let mut lines = utils::file_reader::Lines::new(file);
            while let Some(Ok(line)) = lines.next() {
                repository.add_hash(line);
            }
        }
        Err(err) => tracing::error!("Could not open {} : {}", path.display(), err),
    })
}

fn load_keyword_database(path: PathBuf, repository: Arc<dyn Repository>) -> JoinHandle<()> {
    thread::spawn(move || match File::open(&path) {
        Ok(file) => {
            let mut lines = utils::file_reader::Lines::new(file);
            while let Some(Ok(line)) = lines.next() {
                repository.add_keyword(line);
            }
        }
        Err(err) => tracing::error!("Could not open {} : {}", path.display(), err),
    })
}

fn load_phash_database(path: PathBuf, repository: Arc<dyn Repository>) -> JoinHandle<()> {
    thread::spawn(move || match File::open(&path) {
        Ok(file) => {
            let mut lines = utils::file_reader::Lines::new(file);
            while let Some(Ok(line)) = lines.next() {
                if let Ok(hash) = line.parse::<u64>() {
                    repository.add_phash(hash);
                }
            }
        }
        Err(err) => tracing::error!("Could not open {} : {}", path.display(), err),
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::repository::CsamRepository;

    #[test]
    fn test_create_phash_database() {
        let db_path = PathBuf::from("D:/csam/");
        let root = "D:/images_test/target/original";
        let (send, recv) = mpsc::channel::<usize>();
        let stopped = Arc::new(RwLock::new(false));
        let mut total: usize = 0;

        match create_phash_database(db_path, root, send, stopped.clone()) {
            Ok(size) => {
                total = size;
                println!("Total files found: {}", size);
            }
            Err(err) => assert!(false, "Error: {err}"),
        }

        for p in recv.iter() {
            println!("\t [Total] {}/{}", p, total);
        }
    }

    #[test]
    fn test_load_csam_database() {
        let db_path = PathBuf::from("D:/csam/");
        let repo = Arc::new(CsamRepository::new());

        match load_csam_database(db_path, repo.clone()) {
            Ok(_) => {
                let filename = "File name 13 year old test xpto.";
                let keyword = String::from("13 year old");
                let result = repo.contains_keyword(filename);
                assert_eq!(result, Some(keyword));

                let result = repo.contains_hash("cd63c80d3ad93dde00213e9b7a621513519c0d90");
                assert_eq!(result, true);

                let result = repo.match_phash(15634510955120226520, 10);
                assert_ne!(result, None);
                if let Some(distance) = result {
                    assert_eq!(distance, 1);
                }
            }
            Err(err) => eprintln!("Error: {}", err),
        }
    }
}
