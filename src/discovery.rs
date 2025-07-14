use chrono::Utc;
use rusqlite::{params, Connection};
use socket2::{Domain, Protocol, Socket, Type};
use std::collections::HashSet;
use std::io::StdoutLock;
use std::net::UdpSocket as StdUdpSocket;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::sync::{Arc, Mutex};
use tokio::net::UdpSocket;
use tokio::time::{sleep, Duration};

const DISCOVERY_PORT: u16 = 45678;
const BROADCAST_ADDR: &str = "255.255.255.255:45678";

fn get_local_ip() -> IpAddr {
    let sock = std::net::UdpSocket::bind("0.0.0.0:0").unwrap();
    sock.connect("8.8.8.8:80").unwrap(); /* doesn't actually send */
    sock.local_addr().unwrap().ip()
}

pub fn init_db() -> Connection {
    std::fs::create_dir_all("database").unwrap();
    let conn = Connection::open("database/peers.db").unwrap();

    conn.execute(
        "CREATE TABLE IF NOT EXISTS peers (
        id INTEGER PRIMARY KEY AUTOINCREMENT,
        ip TEXT NOT NULL,
        tcp_port INTEGER NOT NULL,
        name TEXT,
        last_seen TEXT
        )",
        [],
    )
    .unwrap();

    conn.execute(
        "CREATE UNIQUE INDEX IF NOT EXISTS peer_index ON peers (ip, tcp_port);",
        [],
    )
    .unwrap();

    conn
}

pub fn save_peer(conn: &Connection, addr: SocketAddr, name: &str, tcp_port: u16) {
    let ip = addr.ip().to_string();
    let now = Utc::now().to_rfc3339();

    conn.execute(
        "INSERT INTO peers (ip, tcp_port, name, last_seen) 
        VALUES (?1, ?2, ?3, ?4) ON CONFLICT(ip, tcp_port) DO UPDATE
        SET last_seen=?4",
        params![ip, tcp_port, name, now],
    )
    .unwrap();
}

pub async fn start_discovery(name: String, tcp_port: u16) -> Arc<Mutex<HashSet<SocketAddr>>> {
    let peers = Arc::new(Mutex::new(HashSet::new()));

    let peers_clone = peers.clone();
    let name_clone = name.clone();

    let local_ip = get_local_ip();

    let conn = Arc::new(Mutex::new(init_db()));
    let conn_clone = conn.clone();

    tokio::spawn(async move {
        let sock = UdpSocket::bind("0.0.0.0:0").await.unwrap();
        sock.set_broadcast(true).unwrap();

        let msg = serde_json::to_string(&super::HelloMsg {
            name: name_clone,
            tcp_port,
        })
        .unwrap();

        loop {
            let _ = sock.send_to(msg.as_bytes(), BROADCAST_ADDR).await;
            sleep(Duration::from_secs(5)).await;
        }
    });

    let peers_clone2 = peers.clone();
    tokio::spawn(async move {
        let addr = SocketAddr::from((Ipv4Addr::UNSPECIFIED, DISCOVERY_PORT));

        let socket = Socket::new(Domain::IPV4, Type::DGRAM, Some(Protocol::UDP)).unwrap();

        #[cfg(unix)]
        socket.set_reuse_port(true).unwrap();

        socket.bind(&addr.into()).unwrap();

        let std_socket = std::net::UdpSocket::from(socket);
        std_socket.set_nonblocking(true).unwrap();

        let udp_socket = UdpSocket::from_std(std_socket).unwrap();

        let mut buf = [0u8; 1024];

        loop {
            if let Ok((n, addr)) = udp_socket.recv_from(&mut buf).await {
                if let Ok(msg) = serde_json::from_slice::<super::HelloMsg>(&buf[..n]) {
                    if addr.ip() == local_ip && msg.tcp_port == tcp_port {
                        continue; /* self */
                    }

                    if addr.ip().to_string() != "127.0.0.1" {
                        // println!(
                        //     "Found peer {} @ {} (TCP: {})",
                        //     msg.name,
                        //     addr.ip(),
                        //     msg.tcp_port
                        // );

                        peers_clone2
                            .lock()
                            .unwrap()
                            .insert(SocketAddr::new(addr.ip(), msg.tcp_port));

                        let conn = conn_clone.lock().unwrap();
                        save_peer(&conn, addr, &msg.name, msg.tcp_port);
                    }
                }
            }
        }
    });

    peers_clone
}

pub fn print_all_peers(conn: &Connection) {
    let mut stmt = conn
        .prepare("SELECT ip, tcp_port, name, last_seen FROM peers")
        .unwrap();
    let rows = stmt
        .query_map([], |row| {
            Ok((
                row.get::<_, String>(0)?,
                row.get::<_, u16>(1)?,
                row.get::<_, String>(2)?,
                row.get::<_, String>(3)?,
            ))
        })
        .unwrap();

    println!("Known Peers:");
    for row in rows {
        let (ip, port, name, last_seen) = row.unwrap();
        println!("> {}:{} ({}), last seen {}", ip, port, name, last_seen);
    }
}
