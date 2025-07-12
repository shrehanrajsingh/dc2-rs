use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct FileEntry {
    pub name: String,
    pub size: u64,
    pub chunks: u32,
    pub chunk_size: u32,
    pub hash: String,
}

use sha2::{Digest, Sha256};
use std::path::Path;
use tokio::fs::{self, File};
use tokio::io::AsyncReadExt;

const CHUNK_SIZE: u32 = 4096;

pub async fn scan_shared_folder() -> Vec<FileEntry> {
    let mut entries = Vec::new();
    let mut dir = fs::read_dir("hostfile").await.unwrap();

    while let Some(entry) = dir.next_entry().await.unwrap() {
        let path = entry.path();
        if !path.is_file() {
            continue;
        }

        let metadata = entry.metadata().await.unwrap();
        let size = metadata.len();
        let chunks = ((size + CHUNK_SIZE as u64 - 1) / CHUNK_SIZE as u64) as u32;

        let mut file = File::open(&path).await.unwrap();
        let mut hasher = Sha256::new();
        let mut buf = vec![0u8; CHUNK_SIZE as usize];

        loop {
            let n = file.read(&mut buf).await.unwrap();
            if n == 0 {
                break;
            }

            hasher.update(&buf[..n]);
        }

        let hash = hex::encode(hasher.finalize());
        let name = path.file_name().unwrap().to_string_lossy().to_string();

        entries.push(FileEntry {
            name,
            size,
            chunks,
            chunk_size: CHUNK_SIZE,
            hash,
        });
    }

    entries
}
