use std::collections::HashMap;
use std::path::{Path, PathBuf};

use color_eyre::eyre::Result;
use ipnet::Ipv4Net;
use parking_lot::Mutex;
use serde::{Deserialize, Serialize};
use tokio::fs;
use tokio::io::AsyncWriteExt;

#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
    pub repository: String,
    pub network: String,
    pub domain: String,
    pub peers: HashMap<String, Peer>,
    #[serde(skip_deserializing, skip_serializing)]
    pub network_addr: Ipv4Net,
    #[serde(skip_deserializing, skip_serializing)]
    pub config_dir: PathBuf,
    #[serde(skip_deserializing, skip_serializing)]
    mutex: Mutex<u32>,
}

impl Config {
    pub async fn load(path: &PathBuf) -> Result<Self> {
        let data = fs::read_to_string(path).await?;
        let mut config: Config = toml::from_str(&data)?;
        config.network_addr = config.network.parse::<Ipv4Net>()?;
        config.config_dir = fs::canonicalize(path.parent().unwrap_or(&Path::new("."))).await?;
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

    pub fn pid_file(&self, daemon: &str) -> PathBuf {
        Path::new(&self.config_dir).join(&format!("{}.pid", daemon))
    }

    pub async fn write_pid_file(&self, daemon: &str, pid: u32) -> Result<()> {
        let path = self.pid_file(daemon);
        let mut file = fs::File::create(&path).await?;
        file.write(&format!("{}", pid).as_bytes()).await?;
        info!("Written PID {} for {} on file for {}", pid, daemon, path.display());
        Ok(())
    }

    pub async fn remove_pid_file(&self, daemon: &str) -> Result<()> {
        let path = self.pid_file(daemon);
        fs::remove_file(&path).await?;
        info!("PID file {} removed from disk", path.display());
        Ok(())
    }
}

#[derive(Clone, Debug, Default, Deserialize, Serialize, PartialEq)]
pub struct Peer {
    pub username: String,
    pub peername: String,
    pub address: String,
    pub listen_port: u32,
    pub public_key: String,
    pub allowed_ips: Vec<String>,
    pub persistent_keepalive: u32,
    pub endpoint: Option<String>,
    pub table: Option<u32>,
    pub fwmark: Option<u32>,
    pub mtu: u32,
    pub pre_up: Option<Vec<String>>,
    pub post_up: Option<Vec<String>>,
    pub pre_down: Option<Vec<String>>,
    pub post_down: Option<Vec<String>>,
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
        endpoint: Option<String>,
        table: Option<u32>,
        fwmark: Option<u32>,
        mtu: u32,
        pre_up: Option<Vec<String>>,
        post_up: Option<Vec<String>>,
        pre_down: Option<Vec<String>>,
        post_down: Option<Vec<String>>,
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
            endpoint,
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
