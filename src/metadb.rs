use serde::{Deserialize, Serialize};
use serde_json;
use std::fs::{File, OpenOptions};
use std::io::{self, Read, Write};
use std::path::Path;

#[derive(Serialize, Deserialize, Debug)]
pub struct Meta {
    name: String,
    ip: String,
    port: u16,
}

impl Meta {
    pub fn new(name: String, ip: String, port: u16) -> Meta {
        Meta { name, ip, port }
    }
}

pub struct MetaDb {
    db_path: String,
}

impl MetaDb {
    pub fn new(db_path: String) -> Self {
        MetaDb { db_path }
    }

    pub fn init(&self) -> io::Result<()> {
        if !Path::new(&self.db_path).exists() {
            let initial_data: Vec<Meta> = Vec::new();
            let json = serde_json::to_string(&initial_data)
                .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;

            let mut file = File::create(&self.db_path)?;
            file.write_all(json.as_bytes())?;
        }
        Ok(())
    }

    pub fn add(&self, meta: Meta) -> io::Result<()> {
        let mut entries = self.get_all()?;
        entries.push(meta);
        self.write_all(&entries)
    }

    pub fn get_all(&self) -> io::Result<Vec<Meta>> {
        let file_result = File::open(&self.db_path);

        match file_result {
            Ok(mut file) => {
                let mut contents = String::new();
                file.read_to_string(&mut contents)?;

                if contents.is_empty() {
                    return Ok(Vec::new());
                }

                serde_json::from_str(&contents)
                    .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))
            }
            Err(e) if e.kind() == io::ErrorKind::NotFound => {
                self.init()?;
                Ok(Vec::new())
            }
            Err(e) => Err(e),
        }
    }

    pub fn find_by_name(&self, name: &str) -> io::Result<Option<Meta>> {
        let entries = self.get_all()?;
        Ok(entries.into_iter().find(|entry| entry.name == name))
    }

    pub fn update(&self, name: &str, new_meta: Meta) -> io::Result<bool> {
        let mut entries = self.get_all()?;
        let mut updated = false;

        for entry in &mut entries {
            if entry.name == name {
                *entry = new_meta;
                updated = true;
                break;
            }
        }

        if updated {
            self.write_all(&entries)?;
        }

        Ok(updated)
    }

    pub fn delete(&self, name: &str) -> io::Result<bool> {
        let mut entries = self.get_all()?;
        let original_len = entries.len();

        entries.retain(|entry| entry.name != name);

        if entries.len() < original_len {
            self.write_all(&entries)?;
            Ok(true)
        } else {
            Ok(false)
        }
    }

    fn write_all(&self, entries: &[Meta]) -> io::Result<()> {
        let json = serde_json::to_string(entries)
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;

        let mut file = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(&self.db_path)?;

        file.write_all(json.as_bytes())?;

        Ok(())
    }

    pub fn meta_to_json(meta: &Meta) -> Result<String, serde_json::Error> {
        serde_json::to_string(meta)
    }

    pub fn json_to_meta(json: &str) -> Result<Meta, serde_json::Error> {
        serde_json::from_str(json)
    }
}
