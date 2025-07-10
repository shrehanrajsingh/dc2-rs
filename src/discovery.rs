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

pub async fn start_discovery(name: String, tcp_port: u16) -> Arc<Mutex<HashSet<SocketAddr>>> {
    let peers = Arc::new(Mutex::new(HashSet::new()));

    let peers_clone = peers.clone();
    let name_clone = name.clone();

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
                    if addr.ip().to_string() != "127.0.0.1" {
                        println!(
                            "Found peer {} @ {} (TCP: {})",
                            msg.name,
                            addr.ip(),
                            msg.tcp_port
                        );

                        peers_clone2
                            .lock()
                            .unwrap()
                            .insert(SocketAddr::new(addr.ip(), msg.tcp_port));
                    }
                }
            }
        }
    });

    peers_clone
}
