use eframe::egui::{self, CentralPanel, Label};
use rusqlite::Connection;

pub struct AppState {
    conn: Connection,
    peers: Vec<(String, u16, String, String)>,
}

impl AppState {
    pub fn new() -> Self {
        let conn = Connection::open("database/peers.db").unwrap();

        Self {
            peers: load_peers(&conn),
            conn,
        }
    }
}

fn load_peers(conn: &Connection) -> Vec<(String, u16, String, String)> {
    let mut stmt = conn
        .prepare("SELECT ip, tcp_port, name, last_seen FROM peers ORDER BY last_seen DESC")
        .unwrap();

    let rows = stmt
        .query_map([], |row| {
            Ok((row.get(0)?, row.get(1)?, row.get(2)?, row.get(3)?))
        })
        .unwrap();

    rows.map(|r| r.unwrap()).collect()
}

impl eframe::App for AppState {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        CentralPanel::default().show(ctx, |ui| {
            ui.heading("Discovered Peers");

            if self.peers.is_empty() {
                ui.label("No peers found");
            } else {
                for (ip, port, name, seen) in &self.peers {
                    ui.horizontal(|ui| {
                        ui.label(format!("{}: {} [{}] - Last seen {}", ip, port, name, seen));
                    });
                }
            }

            if ui.button("Refresh").clicked() {
                self.peers = load_peers(&self.conn);
            }
        });
    }
}
