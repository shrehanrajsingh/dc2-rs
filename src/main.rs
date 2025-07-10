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
