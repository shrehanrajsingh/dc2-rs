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
