use std::collections::HashMap;
use std::fs::{self, read_to_string};

use eyre::Result;
use parking_lot::Mutex;
use serde::{Deserialize, Serialize};

pub mod key;

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct WgConfig {
    peers: HashMap<String, Host>,
    #[serde(skip_deserializing,skip_serializing)]
    mutex: Mutex<u32>,
}

impl WgConfig {
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

    pub fn peers(&self) -> &HashMap<String, Host> {
        &self.peers
    }
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct Host {
    pub name: String,
    pub address: String,
    pub private_key: Option<String>,
    pub listen_port: i32,
    pub pre_up: Option<String>,
    pub post_up: Option<String>,
    pub post_down: Option<String>,
    #[serde(rename = "DNS")]
    pub dns: Option<Vec<String>>,
    pub peers: Vec<Peer>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct Peer {
    pub name: String,
    pub public_key: String,
    #[serde(rename = "AllowedIPs")]
    pub allowed_ips: Vec<String>,
    pub persistent_keepalive: Option<i32>,
    pub endpoint: Option<String>,
}
