use std::path::Path;
use std::u32;

use indicatif::ProgressBar;
use tokio::fs::metadata;
use tokio::fs::File;
use tokio::io::AsyncSeekExt;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;

use sha2::{Digest, Sha256};

use crate::peer;
use crate::protocol::RequestType;

pub async fn send_file(stream: &mut tokio::net::TcpStream, filepath: &str) {
    let mut manifest = Vec::new();
    let mut file = File::open(filepath).await.unwrap();
    let mut hasher = Sha256::new();
    let mut buf = [0u8; 4096];
    let mut index = 0;

    loop {
        let n = file.read(&mut buf).await.unwrap();
        if n == 0 {
            break;
        }

        let chunk_data = &buf[..n];
        let mut hasher = Sha256::new();
        hasher.update(chunk_data);
        let hash = hasher.finalize();

        manifest.push((index, hash));
        index += 1;
    }

    // Send request type
    stream.write_u32(10 as u32).await.unwrap();
    stream.write_all(b"SEND_FILE\n").await.unwrap();

    // Send Manifest
    stream.write_u32(manifest.len() as u32).await.unwrap();
    for (i, hash) in &manifest {
        stream.write_u32(*i).await.unwrap();
        stream.write_all(&hash[..]).await.unwrap();
    }

    file.seek(std::io::SeekFrom::Start(0)).await.unwrap();

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

    // Receive missing chunk list
    let missing_count = stream.read_u32().await.unwrap();
    let mut missing_indices = Vec::new();

    for _ in 0..missing_count {
        let idx = stream.read_u32().await.unwrap();
        missing_indices.push(idx);
    }

    // let mut chunk_index = 0 as u32;
    // loop {
    //     let n = file.read(&mut buf).await.unwrap();
    //     if n == 0 {
    //         break;
    //     }

    //     // Chunk index
    //     stream.write_u32(chunk_index).await.unwrap();

    //     hasher.update(&buf[..n]);

    //     // Data length in bytes
    //     stream.write_u32(n as u32).await.unwrap(); /* n should be 4096 (bufsize) */
    //     // Chunk data
    //     stream.write_all(&buf[..n]).await.unwrap();
    //     pb.inc(n as u64);

    //     let mut chunk_hash = Sha256::new();
    //     chunk_hash.update(&buf[..n]);

    //     let hash = chunk_hash.finalize();
    //     // SHA-256 hash of chunk data
    //     stream.write_all(&hash).await.unwrap();

    //     chunk_index += 1;
    // }

    for &index in &missing_indices {
        let offset = index as u64 * 4096 as u64;
        file.seek(std::io::SeekFrom::Start(offset)).await.unwrap();

        let n = file.read(&mut buf).await.unwrap();
        let chunk_data = &buf[..n];

        let mut hasher = Sha256::new();
        hasher.update(chunk_data);
        let hash = hasher.finalize();

        // Send index, length, data, hash
        stream.write_u32(index).await.unwrap();
        stream.write_u32(n as u32).await.unwrap();
        stream.write_all(chunk_data).await.unwrap();
        stream.write_all(&hash).await.unwrap();
    }

    pb.finish();

    // Termination marker
    stream.write_u32(u32::MAX).await.unwrap();

    println!(
        "sent file {} [n_chunks: {}]",
        filename,
        missing_indices.len() as u32
    );
}

pub async fn run_client(addr: &str, request: RequestType, payload: Option<String>) {
    let mut stream = tokio::net::TcpStream::connect(addr).await.unwrap();

    match request {
        RequestType::FileList => peer::connect_to_peer(addr).await,

        RequestType::RequestFile => {
            let filename = payload.expect("Missing filename for REQUEST_FILE");
            let message = format!("REQUEST_FILE\n{{\"filename\":\"{}\"}}", filename);

            stream.write_u32(message.len() as u32).await.unwrap();
            stream.write_all(message.as_bytes()).await.unwrap();

            println!("Requested file: {}", filename);
        }

        RequestType::SendFile => {
            let filepath = payload.expect("Missing file for SEND_FILE");
            send_file(&mut stream, &filepath).await;
        }

        _ => (),
    }
}
