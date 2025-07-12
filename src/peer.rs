use crate::files::scan_shared_folder;
use crate::protocol::PeerMessage;
use tokio::io::AsyncWriteExt;

pub async fn connect_to_peer(addr: &str) {
    let mut stream = tokio::net::TcpStream::connect(addr).await.unwrap();
    let mut files = scan_shared_folder().await;

    let msg = PeerMessage::FileList(files);
    let json = serde_json::to_string(&msg).unwrap();
    let json_bytes = json.as_bytes();

    let message = format!("FILE_LIST\n{}", json);

    // println!("{}", message);
    stream.write_u32(message.len() as u32).await.unwrap();
    stream.write_all(message.as_bytes()).await.unwrap();

    println!("Sent file list to peer {}", addr);
}
