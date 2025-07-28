use dotenv::dotenv;
use serde::Serialize;
use std::env;
use std::net::IpAddr;
use std::process::Command;
use std::time::SystemTime;

#[derive(Debug)]
pub struct Peer {
    pub ip: IpAddr,
    pub port: u16,
    pub name: String,
    pub last_seen: SystemTime,
}

impl Peer {
    pub fn new(ip: IpAddr, port: u16, name: String) -> Self {
        Peer {
            ip,
            port,
            name,
            last_seen: SystemTime::now(),
        }
    }

    pub fn update_last_seen(&mut self) {
        self.last_seen = SystemTime::now();
    }
}

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn server_is_running() -> bool {
    let exec = env::var("BACKEND_EXEC").expect("VITE_BACKEND_EXEC not set");
    // dbg!(&exec);

    let output = Command::new(exec)
        .args(&["client", "127.0.0.1:8000", "ping"])
        .output();

    // dbg!(&output);

    match output {
        Ok(output) => {
            let response = String::from_utf8_lossy(&output.stdout);
            return response.contains("PONG");
        }
        Err(_) => return false,
    }
}

#[tauri::command]
fn get_peers() -> String {
    #[derive(Serialize, Debug)]
    struct SerializablePeer {
        ip: String,
        port: u16,
        name: String,
        last_seen: String,
    }

    let exec = match env::var("BACKEND_EXEC") {
        Ok(exec) => exec,
        Err(_) => return String::from("[]"),
    };
    dbg!(&exec);

    let db_path = match env::var("DATABASE_URL") {
        Ok(path) => path,
        Err(_) => return String::from("./"),
    };
    dbg!(&db_path);

    let output = Command::new(exec)
        .args(&["client", "list_peers", &db_path])
        .output();

    dbg!(&output);

    match output {
        Ok(output) => {
            let response = String::from_utf8_lossy(&output.stdout);

            let mut serializable_peers: Vec<SerializablePeer> = Vec::new();

            // Split the response by lines and process each peer line
            for line in response.lines() {
                if line.starts_with(">") {
                    // Remove the leading "> " and parse the peer information
                    let peer_info = line.trim_start_matches("> ");

                    // The format is expected to be: "IP:PORT (NAME), last seen TIMESTAMP"
                    if let Some((addr_name, last_seen_str)) = peer_info.split_once(", last seen ") {
                        if let Some((addr, name)) = addr_name.split_once(" (") {
                            let name = name.trim_end_matches(')').to_string();

                            if let Some((ip_str, port_str)) = addr.split_once(':') {
                                if let (Ok(ip), Ok(port)) =
                                    (ip_str.parse::<IpAddr>(), port_str.parse::<u16>())
                                {
                                    serializable_peers.push(SerializablePeer {
                                        ip: ip.to_string(),
                                        port,
                                        name,
                                        last_seen: String::from(last_seen_str),
                                    });
                                }
                            }
                        }
                    }
                }
            }

            dbg!(&serializable_peers);

            // Serialize to JSON
            match serde_json::to_string(&serializable_peers) {
                Ok(json) => json,
                Err(_) => String::from("[]"),
            }
        }
        Err(_) => String::from("[]"),
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    dotenv().ok();
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            greet,
            server_is_running,
            get_peers
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
