use std::collections::HashMap;
use std::path::PathBuf;

use color_eyre::eyre::Result;
use ipnet::Ipv4Net;
use parking_lot::Mutex;
use serde::{Deserialize, Serialize};
use tokio::fs::{self, read_to_string};

#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
    pub repository: String,
    pub network: String,
    pub domain: String,
    pub peers: HashMap<String, Peer>,
    #[serde(skip_deserializing, skip_serializing)]
    pub network_addr: Ipv4Net,
    #[serde(skip_deserializing, skip_serializing)]
    mutex: Mutex<u32>,
}

impl Config {
    pub async fn load(path: &PathBuf) -> Result<Self> {
        let data = read_to_string(path).await?;
        let mut config: Config = toml::from_str(&data)?;
        config.network_addr = config.network.parse::<Ipv4Net>()?;
        Ok(config)
    }

    pub async fn save(&self, path: &PathBuf) -> Result<()> {
        let _ = self.mutex.lock();
        let data = toml::to_string(self)?;
        fs::write(path, data).await?;
        Ok(())
    }

    pub fn get_peer(&self, peer: &str) -> Option<&Peer> {
        self.peers.get(peer)
    }

    pub fn add_peer(&mut self, name: &str, peer: Peer) {
        let _ = self.mutex.lock();
        self.peers.insert(name.to_string(), peer);
    }

    pub fn remove_peer(&mut self, name: &str) -> Option<Peer> {
        let _ = self.mutex.lock();
        self.peers.remove(name)
    }

    pub fn get_peers_ips(&self) -> Vec<String> {
        self.peers.values().into_iter().map(|v| v.address.clone()).collect::<Vec<String>>()
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Peer {
    pub username: String,
    pub peername: String,
    pub address: String,
    pub listen_port: u32,
    pub public_key: String,
    pub allowed_ips: Vec<String>,
    pub persistent_keepalive: u32,
    pub endpoint: String,
    pub table: Option<u32>,
    pub fwmark: Option<u32>,
    pub mtu: u32,
    pub pre_up: Option<String>,
    pub post_up: Option<String>,
    pub pre_down: Option<String>,
    pub post_down: Option<String>,
    pub dns: Option<Vec<String>>,
}

impl Peer {
    pub fn new(
        username: &str,
        peername: &str,
        address: &str,
        listen_port: u32,
        public_key: &str,
        allowed_ips: &Vec<String>,
        persistent_keepalive: u32,
        endpoint: &str,
        table: Option<u32>,
        fwmark: Option<u32>,
        mtu: u32,
        pre_up: Option<String>,
        post_up: Option<String>,
        pre_down: Option<String>,
        post_down: Option<String>,
        dns: Option<Vec<String>>,
    ) -> Self {
        Peer {
            username: username.to_string(),
            peername: peername.to_string(),
            address: address.to_string(),
            listen_port,
            public_key: public_key.to_string(),
            allowed_ips: allowed_ips.clone(),
            persistent_keepalive,
            endpoint: endpoint.to_string(),
            table,
            fwmark,
            mtu,
            pre_up,
            pre_down,
            post_up,
            post_down,
            dns,
        }
    }
}
