use crate::discovery::{init_db, start_discovery};

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
mod metadb;
mod metalang;
mod peer;
mod protocol;
mod server;

use protocol::RequestType;
use std::{
    io::{self, Write},
    thread::sleep,
    time::Duration,
};

#[derive(serde::Serialize, serde::Deserialize, Debug)]
struct HelloMsg {
    name: String,
    tcp_port: u16,
}

async fn term_run() {
    let args: Vec<String> = std::env::args().collect();
    let mut curr_port = 0;
    let mut dc2mp = "unset";

    if args.len() > 1 {
        dc2mp = &args[1];
        let dc2meta_contents = std::fs::read_to_string(dc2mp).expect("Failed to read dc2meta file");
        let ctx = metalang::Context::from_lang(dc2meta_contents);

        match args.as_slice() {
            [_, _, cmd, port_str] if cmd == "server" => {
                let port = port_str.parse().expect("Invalid port number");
                curr_port = port;
                let peers = start_discovery("Anonymous".to_string(), port).await;
                server::run_server(port).await;
            }
            [_, _, cmd, port_str, head] if cmd == "server" && head == "head" => {
                let port: u16 = port_str.parse().expect("Invalid port number");
                curr_port = port;
            }
            [_, _, cmd, addr, subcmd] if cmd == "client" && subcmd == "file_list" => {
                client::run_client(addr, RequestType::FileList, None).await;
            }
            [_, _, cmd, addr, subcmd, filename] if cmd == "client" && subcmd == "request_file" => {
                client::run_client(addr, RequestType::RequestFile, Some(filename.to_string()))
                    .await;
            }
            [_, _, cmd, addr, subcmd, filepath] if cmd == "client" && subcmd == "send_file" => {
                client::run_client(addr, RequestType::SendFile, Some(filepath.to_string())).await;
            }
            [_, _, cmd, subcmd] if cmd == "client" && subcmd == "list_peers" => {
                let conn = init_db(match ctx.get_var("Database") {
                    metalang::Value::Str(s) => s,
                    _ => unreachable!(),
                });
                discovery::print_all_peers(&conn);
            }
            [_, _, cmd, addr, subcmd] if cmd == "client" && subcmd == "ping" => {
                client::ping_peer(&addr).await;
            }
            _ => {
                eprintln!("Usage:");
                eprintln!("  dc2-rs /path/to/.dc2.meta server port");
                eprintln!("  dc2-rs /path/to/.dc2.meta client <addr> file_list");
                eprintln!("  dc2-rs /path/to/.dc2.meta client <addr> request_file <filename>");
                eprintln!("  dc2-rs /path/to/.dc2.meta client list_peers /path/to/database");
                eprintln!("  dc2-rs /path/to/.dc2.meta client <addr> send_file <filepath>");
                eprintln!("  dc2-rs /path/to/.dc2.meta client <addr> ping");
            }
        }
    } else {
        eprintln!(
            "Missing arguments. Use the format: dc2-rs /path/to/.dc2.meta [command] [args...]"
        );
    }
}

#[tokio::main]
async fn main() {
    term_run().await;
}
