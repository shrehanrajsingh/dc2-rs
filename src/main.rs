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
mod gui;
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
    let conn = discovery::init_db();

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
        [_, cmd, subcmd] if cmd == "client" && subcmd == "list_peers" => {
            discovery::print_all_peers(&conn);
        }
        [_, cmd] if cmd == "session" => {
            println!("Available commands:");
            println!("  server <port>");
            println!("  client <addr> file_list");
            println!("  client <addr> request_file <filename>");
            println!("  client <addr> send_file <filepath>");
            println!("  client list_peers");
            println!("  exit");
            loop {
                // print!("> ");
                // io::stdout().flush().unwrap();
                let mut input = String::new();
                io::stdin().read_line(&mut input).unwrap();
                let input = input.trim();

                if input == "exit" {
                    break;
                }

                let parts: Vec<&str> = input.split_whitespace().collect();
                match parts.as_slice() {
                    ["server", port_str] => {
                        let port = port_str.parse().expect("Invalid port number");
                        tokio::spawn(async move {
                            let peers = start_discovery("Node1".to_string(), port).await;
                            server::run_server(port).await;
                        });
                        sleep(Duration::from_millis(50));
                    }
                    ["client", addr, "file_list"] => {
                        let addr_str = addr.to_string();
                        println!("Requesting file list from {}", addr);
                        tokio::spawn(async move {
                            client::run_client(&addr_str, RequestType::FileList, None).await;
                        });
                    }
                    ["client", addr, "request_file", filename] => {
                        let addr_str = addr.to_string();
                        let filename_str = filename.to_string();
                        println!("Requesting file {} from {}", filename, addr);
                        tokio::spawn(async move {
                            client::run_client(
                                &addr_str,
                                RequestType::RequestFile,
                                Some(filename_str),
                            )
                            .await;
                        });
                    }
                    ["client", addr, "send_file", filepath] => {
                        let addr_str = addr.to_string();
                        let filepath_str = filepath.to_string();
                        println!("Sending file {} to {}", filepath, addr);
                        tokio::spawn(async move {
                            client::run_client(
                                &addr_str,
                                RequestType::SendFile,
                                Some(filepath_str),
                            )
                            .await;
                        });
                    }
                    ["client", "list_peers"] => {
                        discovery::print_all_peers(&conn);
                    }
                    ["help"] => {
                        println!("Available commands:");
                        println!("  server <port>");
                        println!("  client <addr> file_list");
                        println!("  client <addr> request_file <filename>");
                        println!("  client <addr> send_file <filepath>");
                        println!("  exit");
                    }
                    _ => {
                        println!("Unknown command. Type 'help' for usage information.");
                    }
                }
            }
        }
        _ => {
            eprintln!("Usage:");
            eprintln!("  dc2-rs server port");
            eprintln!("  dc2-rs client <addr> file_list");
            eprintln!("  dc2-rs client <addr> request_file <filename>");
            eprintln!("  dc2-rs client list_peers");
            eprintln!("  dc2-rs client <addr> send_file <filepath>");
        }
    }
}

fn gui_run() -> eframe::Result<()> {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "DC2-RS",
        options,
        Box::new(|_cc| Box::new(gui::AppState::new())),
    )
}

#[tokio::main]
async fn main() {
    gui_run().unwrap();
    // term_run().await;
}
