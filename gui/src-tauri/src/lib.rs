use dotenv::dotenv;
use std::env;
use std::process::Command;
// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn server_is_running() -> bool {
    let exec = env::var("BACKEND_EXEC").expect("VITE_BACKEND_EXEC not set");
    dbg!(&exec);

    let output = Command::new(exec)
        .args(&["client", "127.0.0.1:8000", "ping"])
        .output();

    dbg!(&output);

    match output {
        Ok(output) => {
            let response = String::from_utf8_lossy(&output.stdout);
            return response.contains("PONG");
        }
        Err(_) => return false,
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    dotenv().ok();
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![greet, server_is_running])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
