use std::collections::HashMap;
use std::collections::HashSet;
use std::fs;
use std::path::Path;
use std::u32;

use indicatif::ProgressBar;
use tokio::fs::metadata;
use tokio::fs::File;
use tokio::fs::OpenOptions;
use tokio::io::AsyncSeekExt;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;

use sha2::{Digest, Sha256};

use crate::peer;
use crate::protocol::RequestType;
use crate::server;

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

pub async fn receive_file(stream: &mut TcpStream) {
    const END_CHUNK: u32 = u32::MAX;

    let len = stream.read_u32().await.unwrap();
    let mut header_buf = vec![0u8; len as usize];
    stream.read_exact(&mut header_buf).await.unwrap();

    dbg!(&String::from_utf8_lossy(&header_buf));

    let chunk_count = stream.read_u32().await.unwrap();
    let mut manifest = HashMap::new();

    dbg!(&chunk_count);

    for _ in 0..chunk_count {
        let index = stream.read_u32().await.unwrap();
        let mut hash = [0u8; 32];
        stream.read_exact(&mut hash).await.unwrap();
        manifest.insert(index, hash);
    }

    let filename_len = stream.read_u32().await.unwrap();

    dbg!(&filename_len);
    let mut name_buf = vec![0u8; filename_len as usize];
    stream.read_exact(&mut name_buf).await.unwrap();
    let filename = String::from_utf8_lossy(&name_buf);

    let filename = format!("dump/{}", filename);
    fs::create_dir_all("dump").unwrap();

    let filesize = stream.read_u64().await.unwrap();
    let chunk_size = stream.read_u32().await.unwrap();

    println!("Receiving {} ({} bytes)", filename, filesize);

    let mut file = OpenOptions::new()
        .create(true)
        .write(true)
        .read(true)
        .open(&filename)
        .await
        .unwrap();

    dbg!(&filesize);
    file.set_len(filesize).await.unwrap();

    let mut received_chunks = HashSet::new();

    let mut missing_chunks = Vec::new();
    let mut buf = vec![0u8; chunk_size as usize];

    for (&index, expected_hash) in &manifest {
        let offset = index as u64 * chunk_size as u64;
        file.seek(std::io::SeekFrom::Start(offset)).await.unwrap();

        let n = file.read(&mut buf).await.unwrap();
        if n == 0 {
            missing_chunks.push(index);
            continue;
        }

        let mut hasher = Sha256::new();
        hasher.update(&buf[..n]);
        let actual = hasher.finalize();

        if actual.as_slice() != expected_hash {
            missing_chunks.push(index);
        }
    }

    // Send missing chunks
    stream.write_u32(missing_chunks.len() as u32).await.unwrap();
    for &idx in &missing_chunks {
        stream.write_u32(idx).await.unwrap();
    }

    // let missing_count = chunk_count;
    // stream.write_u32(missing_count).await.unwrap();
    // for i in 0..chunk_count {
    //     stream.write_u32(i).await.unwrap();
    // }

    loop {
        let index = stream.read_u32().await.unwrap();
        dbg!(&index);
        if index == END_CHUNK {
            break;
        }

        let len = stream.read_u32().await.unwrap();
        if len == 0 {
            continue;
        }

        let mut chunk = vec![0u8; len as usize];
        stream.read_exact(&mut chunk).await.unwrap();

        let mut hash_buf = [0u8; 32];
        stream.read_exact(&mut hash_buf).await.unwrap();

        let mut hasher = Sha256::new();
        hasher.update(&chunk);
        let hash = hasher.finalize();

        if hash.as_slice() != &hash_buf {
            println!("hash mismatch for chunk {}", index);
            continue;
        }

        let offset = index as u64 * chunk_size as u64;
        file.seek(std::io::SeekFrom::Start(offset)).await.unwrap();
        file.write_all(&chunk).await.unwrap();

        received_chunks.insert(index);
    }

    println!("Download complete: {}", filename);
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

            receive_file(&mut stream).await;
        }

        RequestType::SendFile => {
            let filepath = payload.expect("Missing file for SEND_FILE");
            send_file(&mut stream, &filepath).await;
        }

        _ => (),
    }
}
