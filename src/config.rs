use std::collections::HashMap;
use std::fs::{self, read_to_string};

use eyre::Result;
use parking_lot::Mutex;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct Config {
    peers: HashMap<String, Peer>,
    #[serde(skip_deserializing,skip_serializing)]
    mutex: Mutex<u32>,
}

impl Config {
    pub fn load(path: &str) -> Result<Self> {
        let data = read_to_string(path)?;
        Ok(toml::from_str(&data)?)
    }

    pub fn save(&self, path: &str) -> Result<()> {
        let _ = self.mutex.lock();
        let data = toml::to_string(self)?;
        fs::write(path, data)?;
        Ok(())
    }

    pub fn peers(&self) -> &HashMap<String, Peer> {
        &self.peers
    }
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct Peer {
    pub name: String,
    pub address: String,
    pub listen_port: i32,
    pub public_key: String,
    pub allowed_ips: Vec<String>,
    pub persistent_keepalive: Option<i32>,
    pub endpoint: Option<String>,
}
