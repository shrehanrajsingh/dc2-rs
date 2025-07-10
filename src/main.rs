use crate::discovery::start_discovery;

/**
 * dc2-rs
 * Welcome to DC2-RS, a peer-to-peer miscellaneous protocol.
 * DC2-RS turns your device into a Node (Peer) that can initiate
 * file transfers and request the same from other Nodes (Peers)
 *
 */
mod client;
mod discovery;
mod server;

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
        [_, cmd, addr, path] if cmd == "client" => client::run_client(&addr, &path).await,
        _ => eprintln!("Usage:\ndc2-rs server <port>\ndc2-rs client <addr> <file>"),
    }
}
