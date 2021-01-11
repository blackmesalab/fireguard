use std::collections::HashMap;
use std::path::PathBuf;

use color_eyre::eyre::{bail, Result};
use parking_lot::Mutex;
use serde::{Deserialize, Serialize};
use tera::{Context, Tera};
use tokio::fs::File;
use tokio::io::AsyncWriteExt;

use crate::config::Peer as ConfigPeer;

static WIREGARD_CONFIG_TMPL: &str = r#"# {{ host.name }} wireguard configuration
# Note: this file is managed by fireguard (https://github.com/blackmesalab/fireguard)
[Interface]
Address = {{ host.address }}
PrivateKey = {{ host.private_key }}
{% if host.listen_port > 0 %}ListenPort = {{ host.listen_port }}{% endif %}
{% if host.dns %}DNS = {{ host.dns | join(sep=",") }}{% endif %}
{% if host.pre_up %}PreUp = {{ host.pre_up }}{% endif %}
{% if host.post_up %}PostUp = {{ host.post_up }}{% endif %}
{% if host.pre_down %}PreDown = {{ host.pre_down }}{% endif %}
{% if host.post_down %}PostDown = {{ host.post_down }}{% endif %}

{% for peer in host.peers %}# Peer {{ peer.name }}
[Peer]
PublicKey = {{ peer.public_key }}
{% if peer.endpoint %}Endpoint = {{ peer.endpoint }} {% endif %}
{% if peer.allowed_ips %}AllowedIps = {{ peer.allowed_ips | join(sep=",") }}{% endif %}
{% if peer.persistent_keepalive > 0 %}PersistentKeepalive = {{ peer.persistent_keepalive}}{% endif %}

{% endfor -%}"#;

#[derive(Debug, Serialize, Deserialize)]
pub struct WgConfig {
    host: Host,
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
            let mut wg_peers = peers
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
                .collect::<Vec<Peer>>();
            wg_peers.retain(|x| x.name != peername);
            let wg_host = Host::new(
                peername,
                my_peer.address.clone(),
                private_key.to_string(),
                my_peer.listen_port,
                my_peer.pre_up.clone().unwrap_or(String::new()),
                my_peer.post_up.clone().unwrap_or(String::new()),
                my_peer.pre_down.clone().unwrap_or(String::new()),
                my_peer.post_down.clone().unwrap_or(String::new()),
                my_peer.dns.clone().unwrap_or(Vec::new()),
                wg_peers,
            );
            Ok(Self { host: wg_host, mutex: Mutex::new(0) })
        } else {
            bail!("Unable to find peer {} for repository {}", peername, repository)
        }
    }

    pub async fn render(&self, config_path: &PathBuf) -> Result<()> {
        info!("Rendering Wireguard configuration on {}", config_path.display());
        let mut wg_tera = Tera::default();
        wg_tera.add_raw_template("wireguard.txt", WIREGARD_CONFIG_TMPL)?;
        let wg_config = wg_tera.render("wireguard.txt", &Context::from_serialize(self)?)?;
        let mut file = File::create(config_path).await?;
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
    pub pre_down: String,
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
        pre_down: String,
        post_down: String,
        dns: Vec<String>,
        peers: Vec<Peer>,
    ) -> Self {
        Host { name, address, private_key, listen_port, pre_up, pre_down, post_up, post_down, dns, peers }
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
