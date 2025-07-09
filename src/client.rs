use std::path::Path;
use std::u32;

use indicatif::ProgressBar;
use tokio::fs::metadata;
use tokio::fs::File;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;

use sha2::{Digest, Sha256};

pub async fn run_client(addr: &str, filepath: &str) {
    let stream_result = TcpStream::connect(addr).await;

    let mut stream = match stream_result {
        Ok(s) => s,
        Err(e) => {
            eprintln!("error connecting to client: {}", e);
            return;
        }
    };

    let mut file = File::open(filepath).await.unwrap();
    let mut hasher = Sha256::new();
    let mut buf = [0u8; 4096];

    let filename = Path::new(filepath).file_name().unwrap().to_string_lossy();

    // Filename length
    stream.write_u32(filename.len() as u32).await.unwrap();

    // Filename as byte array
    stream.write_all(filename.as_bytes()).await.unwrap();

    let size = metadata(filepath).await.unwrap().len();
    let pb = ProgressBar::new(size);

    // Total file size in bytes
    stream.write_u64(size).await.unwrap();

    // Chunk size
    stream.write_u32(4096 as u32).await.unwrap(); /* 4KB */

    let mut chunk_index = 0 as u32;
    loop {
        let n = file.read(&mut buf).await.unwrap();
        if n == 0 {
            break;
        }

        // Chunk index
        stream.write_u32(chunk_index).await.unwrap();

        hasher.update(&buf[..n]);

        // Data length in bytes
        stream.write_u32(n as u32).await.unwrap(); /* n should be 4096 (bufsize) */

        // Chunk data
        stream.write_all(&buf[..n]).await.unwrap();
        pb.inc(n as u64);

        let mut chunk_hash = Sha256::new();
        chunk_hash.update(&buf[..n]);

        let hash = chunk_hash.finalize();
        // SHA-256 hash of chunk data
        stream.write_all(&hash).await.unwrap();

        chunk_index += 1;
    }

    pb.finish();

    // Termination marker
    stream.write_u32(u32::MAX).await.unwrap();

    println!("sent file {} [n_chunks: {}]", filename, chunk_index + 1);
}
