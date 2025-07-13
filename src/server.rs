use std::collections::{HashMap, HashSet};
use std::io::ErrorKind;
use std::net::SocketAddr;
use std::process::exit;
use std::{fs, vec};
use tokio::net::TcpStream;

use sha2::{Digest, Sha256, Sha256VarCore};
use tokio::fs::{File, OpenOptions};
use tokio::io::{AsyncReadExt, AsyncSeekExt, AsyncWriteExt};
use tokio::net::TcpListener;
use tokio::stream;

use crate::client;
use crate::files::FileEntry;
use crate::protocol::{PeerMessage, RequestType};

async fn handle_peer(mut socket: TcpStream, mut addr: SocketAddr) {
    let len = match socket.read_u32().await {
        Ok(l) => l,
        Err(e) => {
            if e.kind() == ErrorKind::UnexpectedEof {
                println!("Connection Closed Due to Unexpected EOF");
            }
            return;
        }
    };

    dbg!(&len);

    let mut buf = vec![0u8; len as usize];
    socket.read_exact(&mut buf).await.unwrap();

    // println!("buf, {}", String::from_utf8_lossy(&buf));

    // let msg: PeerMessage = serde_json::from_slice(&buf).unwrap();

    // match msg {
    //     PeerMessage::FileList(files) => {
    //         println!("Peer has the following files: ");

    //         for f in files {
    //             println!("- {} ({} bytes, {} chunks)", f.name, f.size, f.chunks);
    //         }
    //     }
    // }

    let msg = String::from_utf8_lossy(&buf);
    let mut lines = msg.lines();
    let request_type = lines.next().unwrap();
    let payload = lines.collect::<Vec<_>>().join("\n");

    dbg!(&msg);

    match RequestType::from_str(request_type) {
        Some(RequestType::FileList) => {
            let map: HashMap<String, Vec<FileEntry>> = serde_json::from_str(&payload).unwrap();
            let files = map.get("FileList").unwrap();
            println!("Peer file list:");
            for f in files {
                println!("- {} ({} bytes)", f.name, f.size);
            }
        }

        Some(RequestType::SendFile) => {
            tokio::spawn(async move {
                const END_CHUNK: u32 = u32::MAX;

                /* recieve manifest */
                let chunk_count = socket.read_u32().await.unwrap();
                let mut manifest = HashMap::new();

                for _ in 0..chunk_count {
                    let index = socket.read_u32().await.unwrap();
                    let mut hash = [0u8; 32];
                    socket.read_exact(&mut hash).await.unwrap();
                    manifest.insert(index, hash);
                }

                /* metadata */
                // 1. Receive filename length
                let filename_len = socket.read_u32().await.unwrap();
                // 2. Receive filename as byte array
                dbg!(&filename_len);
                let mut name_buf = vec![0u8; filename_len as usize];
                socket.read_exact(&mut name_buf).await.unwrap();
                let filename = String::from_utf8_lossy(&name_buf);

                let filename = match fs::create_dir_all("dump") {
                    Ok(_) => {
                        format!("dump/{}", filename)
                    }
                    Err(e) => {
                        eprintln!("error creating /dump: {}", e);
                        filename.to_string()
                    }
                };

                dbg!(&filename);

                // 3. Receive file size
                let filesize = socket.read_u64().await.unwrap();
                // 4. Receive chunk size in bytes
                let chunk_size = socket.read_u32().await.unwrap();

                println!(
                    "receiving {} ({} bytes, {}-byte chunks)",
                    filename, filesize, chunk_size
                );

                let mut file = OpenOptions::new()
                    .create(true)
                    .write(true)
                    .read(true)
                    .open(filename.to_string())
                    .await
                    .unwrap();

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
                socket.write_u32(missing_chunks.len() as u32).await.unwrap();
                for &idx in &missing_chunks {
                    socket.write_u32(idx).await.unwrap();
                }

                file.set_len(filesize).await.unwrap();
                let mut received_chunks = HashSet::new();

                loop {
                    // 5.1 Receive chunk index
                    let index = socket.read_u32().await.unwrap();
                    if index == END_CHUNK {
                        break;
                    }

                    // 5.2 data length in bytes
                    let len = socket.read_u32().await.unwrap();

                    if len == 0 {
                        eprintln!("received chunk of length 0 - skipping.");
                        continue;
                    }

                    let mut buf = vec![0u8; len as usize];
                    // 5.3 Chunk data
                    socket.read_exact(&mut buf).await.unwrap();

                    let mut hash_buf = [0u8; 32];
                    // 5.4 SHA-256 hash of chunk data
                    socket.read_exact(&mut hash_buf).await.unwrap();

                    let mut hasher = Sha256::new();
                    hasher.update(&buf);
                    let hash = hasher.finalize();

                    if hash.as_slice() != &hash_buf {
                        println!("hash mismatch for chunk {}", index);
                        continue;
                    }

                    let offset = index as u64 * chunk_size as u64;
                    file.seek(std::io::SeekFrom::Start(offset)).await.unwrap();
                    file.write_all(&buf).await.unwrap();

                    received_chunks.insert(index);
                }
            });
        }

        Some(RequestType::RequestFile) => {
            let map: HashMap<String, String> = serde_json::from_str(&payload).unwrap();
            let filename = map.get("filename").unwrap().clone();

            println!("Requested file: {}", filename);

            let filepath = format!("hostfile/{}", filename);
            if !std::path::Path::new(&filepath).exists() {
                println!("File not found: {}", filepath);
                return;
            }

            client::send_file(&mut socket, &filepath).await;
        }

        Some(RequestType::Chunk) => {}

        _ => {
            println!("Unknown request type: {}", request_type);
        }
    }
}

pub async fn run_server(port: u16) {
    let listener;
    listener = TcpListener::bind(("0.0.0.0", port)).await.unwrap();

    println!("server listening on port {}", port);

    loop {
        let (mut socket, addr) = match listener.accept().await {
            Ok((s, a)) => (s, a),
            Err(e) => {
                eprintln!("Error: {}", e);
                return;
            }
        };

        println!("new connection from {}", addr);

        tokio::spawn(async move {
            handle_peer(socket, addr).await;
        });
    }
}
