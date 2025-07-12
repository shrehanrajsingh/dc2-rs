use crate::files::FileEntry;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub enum PeerMessage {
    FileList(Vec<FileEntry>),
}

#[derive(Debug)]
pub enum RequestType {
    FileList,
    RequestFile,
    Chunk,
    SendFile,
}

impl RequestType {
    pub fn from_str(s: &str) -> Option<Self> {
        match s.trim() {
            "FILE_LIST" => Some(Self::FileList),
            "REQUEST_FILE" => Some(Self::RequestFile),
            "CHUNK" => Some(Self::Chunk),
            "SEND_FILE" => Some(Self::SendFile),
            _ => None,
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            RequestType::FileList => "FILE_LIST",
            RequestType::Chunk => "CHUNK",
            RequestType::RequestFile => "REQUEST_FILE",
            RequestType::SendFile => "SEND_FILE",
        }
    }
}
