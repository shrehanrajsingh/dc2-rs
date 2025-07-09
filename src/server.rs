use std::collections::HashSet;
use std::{fs, vec};

use sha2::{Digest, Sha256};
use tokio::fs::{File, OpenOptions};
use tokio::io::{AsyncReadExt, AsyncSeekExt, AsyncWriteExt};
use tokio::net::TcpListener;

pub async fn run_server() {
    let listener = TcpListener::bind("0.0.0.0:8000").await.unwrap();

    println!("server listening on port 8000");

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
            const END_CHUNK: u32 = u32::MAX;

            /* metadata */
            // 1. Receive filename length
            let filename_len = socket.read_u32().await.unwrap();
            // 2. Receive filename as byte array
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
}
