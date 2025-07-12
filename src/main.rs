use crate::discovery::start_discovery;

/* dc2-rs
 * Welcome to DC2-RS, a peer-to-peer protocol for file transfers.
 * DC2-RS enables your device to operate as a node (peer) capable of both initiating
 * and receiving file transfers from other nodes.
 * Each node requires at least two available ports:
 * - One for the server endpoint, which handles incoming file transfer requests (seeding).
 * - One for the client endpoint, which initiates file download requests to other nodes' server endpoints.
*/
mod client;
mod discovery;
mod files;
mod peer;
mod protocol;
mod server;

use protocol::RequestType;

#[derive(serde::Serialize, serde::Deserialize, Debug)]
struct HelloMsg {
    name: String,
    tcp_port: u16,
}

#[tokio::main]
async fn main() {
    let args: Vec<String> = std::env::args().collect();

    match args.as_slice() {
        [_, cmd, port_str] if cmd == "server" => {
            let port = port_str.parse().expect("Invalid port number");
            let peers = start_discovery("Node1".to_string(), port).await;
            server::run_server(port).await;
        }
        [_, cmd, addr, subcmd] if cmd == "client" && subcmd == "file_list" => {
            client::run_client(addr, RequestType::FileList, None).await;
        }
        [_, cmd, addr, subcmd, filename] if cmd == "client" && subcmd == "request_file" => {
            client::run_client(addr, RequestType::RequestFile, Some(filename.to_string())).await;
        }
        [_, cmd, addr, subcmd, filepath] if cmd == "client" && subcmd == "send_file" => {
            client::run_client(addr, RequestType::SendFile, Some(filepath.to_string())).await;
        }
        _ => {
            eprintln!("Usage:");
            eprintln!("  dc2-rs server port");
            eprintln!("  dc2-rs client <addr> file_list");
            eprintln!("  dc2-rs client <addr> request_file <filename>");
            eprintln!("  dc2-rs client <addr> send_file <filepath>");
        }
    }
}
