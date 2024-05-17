use anyhow::Context;
use std::fs::{self, OpenOptions};
use std::io;
use std::io::prelude::*;
use std::path::{Path, PathBuf};
use std::sync::{
    atomic::{AtomicBool, Ordering},
    mpsc::{self, Receiver},
    Arc,
};
use std::thread;
use threadpool::ThreadPool;
use walkdir::WalkDir;

use crate::utils;

use super::repository::Repository;

const FILE_HASH: &str = "hash.txt";
const FILE_KEYWORD: &str = "keyword.txt";
const FILE_PHASH: &str = "phash.txt";

pub fn create_keyword_database(db_path: PathBuf, content: &str) -> anyhow::Result<()> {
    if !db_path.exists() {
        fs::create_dir_all(&db_path)
            .with_context(|| format!("Could not create `{}` path", db_path.display()))?;
    }

    let file_path = db_path.join(FILE_KEYWORD);
    let file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(&file_path)
        .with_context(|| format!("Could not open file: {}", file_path.display()))?;

    let mut writer = io::BufWriter::new(file);
    writer.write_all(content.as_bytes())?;
    writer.flush()?;

    Ok(())
}

pub fn create_hash_database<P>(
    db_path: PathBuf,
    root: P,
    cancel_flag: Arc<AtomicBool>,
) -> anyhow::Result<usize>
where
    P: AsRef<Path>,
{
    let (hash_sender, hash_receiver) = mpsc::channel::<String>();

    write_in_database(db_path, FILE_HASH, hash_receiver)
        .with_context(|| "Could not create hash database.")?;

    cancel_flag.store(false, Ordering::SeqCst);
    let mut count_files: usize = 0;

    let cpus = num_cpus::get();
    let thread_pool = ThreadPool::new(cpus);

    for entry in WalkDir::new(root)
        .follow_links(false)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| !e.file_type().is_dir() && self::is_image(e.path()))
    {
        if cancel_flag.load(Ordering::SeqCst) {
            break;
        }

        count_files += 1;
        let cancel_flag = cancel_flag.clone();
        let c_hash_sender = hash_sender.clone();

        thread_pool.execute(move || {
            if cancel_flag.load(Ordering::SeqCst) {
                return;
            }

            match utils::media::get_file_hash_md5(entry.path()) {
                Ok(hash) => {
                    c_hash_sender.send(hash).expect("could not send hash");
                }
                Err(err) => tracing::error!(
                    "Could not generate hash. {}\nError: {}",
                    entry.path().display(),
                    err
                ),
            }
        });
    }

    // wait for thread pool to process all jobs
    thread_pool.join();
    drop(hash_sender);

    Ok(count_files)
}

pub fn create_phash_database<P>(
    db_path: PathBuf,
    root: P,
    cancel_flag: Arc<AtomicBool>,
) -> anyhow::Result<usize>
where
    P: AsRef<Path>,
{
    let (phash_sender, phash_receiver) = mpsc::channel::<String>();

    write_in_database(db_path, FILE_PHASH, phash_receiver)
        .with_context(|| "Could not create perceptual hash database.")?;

    cancel_flag.store(false, Ordering::SeqCst);
    let mut count_files: usize = 0;

    let cpus = num_cpus::get();
    let thread_pool = ThreadPool::new(cpus);

    for entry in WalkDir::new(root)
        .follow_links(false)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| !e.file_type().is_dir() && self::is_image(e.path()))
    {
        if cancel_flag.load(Ordering::SeqCst) {
            break;
        }

        count_files += 1;
        let cancel_flag = cancel_flag.clone();
        let c_phash_sender = phash_sender.clone();

        thread_pool.execute(move || {
            if cancel_flag.load(Ordering::SeqCst) {
                return;
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

    // wait for thread pool to process all jobs
    thread_pool.join();
    drop(phash_sender);

    Ok(count_files)
}

// Worker writing the content to the file.
fn write_in_database(
    db_path: PathBuf,
    db_name: &str,
    receiver: Receiver<String>,
) -> anyhow::Result<()> {
    if !db_path.exists() {
        fs::create_dir_all(&db_path)
            .with_context(|| format!("Could not create `{}` path", db_path.display()))?;
    }

    let file_path = db_path.join(db_name);
    let file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(&file_path)
        .with_context(|| format!("Could not open file: {}", file_path.display()))?;

    thread::spawn(move || {
        let mut writer = io::BufWriter::new(file);
        let mut content = String::new();

        for value in receiver.iter() {
            content.clear();
            content.push_str(&value);
            content.push('\n');

            match writer.write_all(content.as_bytes()) {
                Err(err) => {
                    tracing::error!("Could not write in file.\nError: {:?}", err)
                }
                _ => (),
            }
        }

        match writer.flush() {
            Err(err) => {
                tracing::error!("Could not write in file.\nError: {:?}", err)
            }
            _ => (),
        }
    });

    Ok(())
}

fn is_image(entry: &Path) -> bool {
    match entry.extension() {
        Some(e) if utils::media::is_image(&e.to_string_lossy().to_lowercase()) => true,
        _ => false,
    }
}

pub fn load_keyword_database(db_path: PathBuf, repo: Arc<dyn Repository>) -> anyhow::Result<()> {
    let path = db_path.join(FILE_KEYWORD);

    match OpenOptions::new()
        .read(true)
        .append(true)
        .create(true)
        .open(&path)
    {
        Ok(file) => {
            repo.remove_all_keywords();

            let mut lines = utils::file_reader::Lines::new(file);
            while let Some(Ok(line)) = lines.next() {
                if !line.trim().is_empty() {
                    repo.add_keyword(line);
                }
            }
        }
        Err(err) => anyhow::bail!("Could not open {} : {}", path.display(), err),
    }

    Ok(())
}

pub fn load_hash_database(db_path: PathBuf, repo: Arc<dyn Repository>) -> anyhow::Result<()> {
    let path = db_path.join(FILE_HASH);

    match OpenOptions::new()
        .read(true)
        .append(true)
        .create(true)
        .open(&path)
    {
        Ok(file) => {
            repo.remove_all_hash();

            let mut lines = utils::file_reader::Lines::new(file);
            while let Some(Ok(line)) = lines.next() {
                repo.add_hash(line);
            }
        }
        Err(err) => anyhow::bail!("Could not open {} : {}", path.display(), err),
    }

    Ok(())
}

pub fn load_phash_database(db_path: PathBuf, repo: Arc<dyn Repository>) -> anyhow::Result<()> {
    let path = db_path.join(FILE_PHASH);

    match OpenOptions::new()
        .read(true)
        .append(true)
        .create(true)
        .open(&path)
    {
        Ok(file) => {
            repo.remove_all_phash();

            let mut lines = utils::file_reader::Lines::new(file);
            while let Some(Ok(line)) = lines.next() {
                if let Ok(hash) = line.parse::<u64>() {
                    repo.add_phash(hash);
                }
            }
        }
        Err(err) => anyhow::bail!("Could not open {} : {}", path.display(), err),
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::csam::repository::InMemoryRepository;

    #[test]
    fn test_should_create_keyword_database() {
        let db_path = PathBuf::from("D:/csam_test/");
        let content = "keyword_1\nkeyword_2\nkeyword_3";

        match create_keyword_database(db_path, content) {
            Ok(_) => assert!(true),
            Err(err) => assert!(false, "{err}"),
        }
    }

    #[test]
    fn test_should_create_hash_database() {
        let db_path = PathBuf::from("D:/csam_test/");
        let root = "D:/images_test/target/original";
        let cancel_flag = Arc::new(AtomicBool::new(false));

        match create_hash_database(db_path, root, cancel_flag.clone()) {
            Ok(size) => println!("Total hash created: {}", size),
            Err(err) => assert!(false, "{err}"),
        }
    }

    #[test]
    fn test_should_create_phash_database() {
        let db_path = PathBuf::from("D:/csam_test/");
        let root = "D:/images_test/target/original";
        let cancel_flag = Arc::new(AtomicBool::new(false));

        match create_phash_database(db_path, root, cancel_flag.clone()) {
            Ok(size) => println!("Total files found: {}", size),
            Err(err) => assert!(false, "{err}"),
        }
    }

    #[test]
    fn test_should_load_keyword_database() {
        let db_path = PathBuf::from("D:/csam_test/");
        let repo = Arc::new(InMemoryRepository::new());

        match load_keyword_database(db_path, repo.clone()) {
            Ok(_) => {
                assert!(repo.count_keyword() > 0);
            }
            Err(err) => assert!(false, "{err}"),
        }
    }

    #[test]
    fn test_should_load_hash_database() {
        let db_path = PathBuf::from("D:/csam_test/");
        let repo = Arc::new(InMemoryRepository::new());

        match load_hash_database(db_path, repo.clone()) {
            Ok(_) => {
                assert!(repo.count_hash() > 0);
            }
            Err(err) => assert!(false, "{err}"),
        }
    }

    #[test]
    fn test_should_load_phash_database() {
        let db_path = PathBuf::from("D:/csam_test/");
        let repo = Arc::new(InMemoryRepository::new());

        match load_phash_database(db_path, repo.clone()) {
            Ok(_) => {
                assert!(repo.count_phash() > 0);
            }
            Err(err) => assert!(false, "{err}"),
        }
    }
}
