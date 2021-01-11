pub mod key;

use std::collections::HashMap;
use std::path::Path;

use color_eyre::eyre::{bail, Result};
use parking_lot::Mutex;
use serde::{Deserialize, Serialize};
use tera::{Context, Tera};
use tokio::fs::File;
use tokio::io::AsyncWriteExt;

use crate::config::Peer as ConfigPeer;

#[derive(Debug, Serialize, Deserialize)]
pub struct WgConfig {
    host: Host,
    peers: Vec<Peer>,
    #[serde(skip_deserializing, skip_serializing)]
    mutex: Mutex<u8>,
}

impl WgConfig {
    pub fn new(
        peers: HashMap<String, ConfigPeer>,
        repository: &str,
        username: &str,
        peername: &str,
        private_key: &str,
    ) -> Result<Self> {
        let peername = format!("{}-{}", username, peername);
        let my_peer = peers.get(&peername);
        if let Some(my_peer) = my_peer {
            let wg_peers = peers
                .values()
                .map(|x| {
                    Peer::new(
                        format!("{}-{}", x.username, x.peername),
                        x.public_key.clone(),
                        x.allowed_ips.clone(),
                        x.persistent_keepalive,
                        x.endpoint.clone().unwrap_or(String::new()),
                    )
                })
                .collect();
            let wg_host = Host::new(
                peername,
                my_peer.address.clone(),
                private_key.to_string(),
                my_peer.listen_port,
                my_peer.pre_up.clone().unwrap_or(String::new()),
                my_peer.post_up.clone().unwrap_or(String::new()),
                my_peer.post_down.clone().unwrap_or(String::new()),
                my_peer.dns.clone().unwrap_or(Vec::new()),
                Vec::new(),
            );
            Ok(Self { host: wg_host, peers: wg_peers, mutex: Mutex::new(0) })
        } else {
            bail!("Unable to find peer {} for repository {}", peername, repository)
        }
    }

    pub async fn render(&self, config_path: &str, username: &str, peername: &str) -> Result<()> {
        let wg_tmpl = "templates/*.txt".to_string();
        let config = Path::new(config_path).join(&format!("{}-{}", username, peername));
        info!("Rendering Wireguard configuration on {}", config.display());
        let wg_tera = Tera::new(&wg_tmpl)?;
        let wg_config = wg_tera.render("templates/wireguard.txt", &Context::from_serialize(self)?)?;
        let mut file = File::open(&config).await?;
        file.write_all(&wg_config.as_bytes()).await?;
        Ok(())
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Host {
    pub name: String,
    pub address: String,
    pub private_key: String,
    pub listen_port: u32,
    pub pre_up: String,
    pub post_up: String,
    pub post_down: String,
    pub dns: Vec<String>,
    pub peers: Vec<Peer>,
}

impl Host {
    pub fn new(
        name: String,
        address: String,
        private_key: String,
        listen_port: u32,
        pre_up: String,
        post_up: String,
        post_down: String,
        dns: Vec<String>,
        peers: Vec<Peer>,
    ) -> Self {
        Host { name, address, private_key, listen_port, pre_up, post_up, post_down, dns, peers }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Peer {
    pub name: String,
    pub public_key: String,
    pub allowed_ips: Vec<String>,
    pub persistent_keepalive: u32,
    pub endpoint: String,
}

impl Peer {
    pub fn new(
        name: String,
        public_key: String,
        allowed_ips: Vec<String>,
        persistent_keepalive: u32,
        endpoint: String,
    ) -> Self {
        Self { name, public_key, allowed_ips, persistent_keepalive, endpoint }
    }
}
