/**
 * dc2-rs
 * Welcome to DC2-RS, a peer-to-peer miscellaneous protocol.
 * DC2-RS turns your device into a Node (Peer) that can initiate
 * file transfers and request the same from other Nodes (Peers)
 *
 */
mod client;
mod server;

#[tokio::main]
async fn main() {
    let args: Vec<String> = std::env::args().collect();

    match args.as_slice() {
        [_, cmd] if cmd == "server" => server::run_server().await,
        [_, cmd, addr, path] if cmd == "client" => client::run_client(&addr, &path).await,
        _ => eprintln!("Usage:\ndc2-rs server\ndc2-rs client <addr> <file>"),
    }
}
