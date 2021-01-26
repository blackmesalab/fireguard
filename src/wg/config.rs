use std::collections::HashMap;
use std::path::Path;

use color_eyre::eyre::{bail, Result};
use parking_lot::Mutex;
use serde::{Deserialize, Serialize};
use tera::{Context, Tera};
use tokio::fs::File;
use tokio::io::AsyncWriteExt;

use crate::config::Peer as ConfigPeer;

static WIREGARD_CONFIG_TMPL: &str = r#"# {{ host.repository }} - {{ host.name }} wireguard configuration
# Note: this file is managed by fireguard (https://github.com/blackmesalab/fireguard)
[Interface]
Address = {{ host.address }}
PrivateKey = {{ host.private_key }}
{% if host.listen_port > 0 %}ListenPort = {{ host.listen_port }}{% endif %}
{% if host.dns %}DNS = {{ host.dns | join(sep=",") }}{% endif %}
{% if host.fwmark > 0 %}FwMark = {{ host.fwmark }}{% endif %}
{% if host.table > 0 %}Table = {{ host.table }}{% endif %}
{% if host.pre_up %}PreUp = {{ host.pre_up }}{% endif %}
{% if host.post_up %}PostUp = {{ host.post_up }}{% endif %}
{% if host.pre_down %}PreDown = {{ host.pre_down }}{% endif %}
{% if host.post_down %}PostDown = {{ host.post_down }}{% endif %}

{% for peer in host.peers %}# Peer {{ peer.name }}
[Peer]
{% if peer.endpoint %}Endpoint = {{ peer.endpoint }}:{{ peer.listen_port }}{% endif %}
PublicKey = {{ peer.public_key }}
AllowedIps = {{ peer.allowed_ips | join(sep=",") }}
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
                        x.listen_port,
                        x.allowed_ips.clone(),
                        x.persistent_keepalive,
                        x.endpoint.clone(),
                    )
                })
                .collect::<Vec<Peer>>();
            wg_peers.retain(|x| x.name != peername);
            let wg_host = Host::new(
                repository.to_string(),
                peername,
                my_peer.address.clone(),
                private_key.to_string(),
                my_peer.listen_port,
                my_peer.pre_up.clone().unwrap_or_default(),
                my_peer.post_up.clone().unwrap_or_default(),
                my_peer.pre_down.clone().unwrap_or_default(),
                my_peer.post_down.clone().unwrap_or_default(),
                my_peer.dns.clone().unwrap_or_default(),
                my_peer.table.unwrap_or(0),
                my_peer.fwmark.unwrap_or(0),
                wg_peers,
            );
            Ok(Self { host: wg_host, mutex: Mutex::new(0) })
        } else {
            bail!("Unable to find peer {} for repository {}", peername, repository)
        }
    }

    pub async fn render(&self, config_path: &Path) -> Result<()> {
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
    pub repository: String,
    pub name: String,
    pub address: String,
    pub private_key: String,
    pub listen_port: u32,
    pub pre_up: String,
    pub post_up: String,
    pub pre_down: String,
    pub post_down: String,
    pub dns: Vec<String>,
    pub table: u32,
    pub fwmark: u32,
    pub peers: Vec<Peer>,
}

impl Host {
    #![allow(clippy::too_many_arguments)]
    pub fn new(
        repository: String,
        name: String,
        address: String,
        private_key: String,
        listen_port: u32,
        pre_up: Vec<String>,
        post_up: Vec<String>,
        pre_down: Vec<String>,
        post_down: Vec<String>,
        dns: Vec<String>,
        table: u32,
        fwmark: u32,
        peers: Vec<Peer>,
    ) -> Self {
        Self {
            repository,
            name,
            address,
            private_key,
            listen_port,
            pre_up: Self::build_hook_cmd(pre_up),
            pre_down: Self::build_hook_cmd(pre_down),
            post_up: Self::build_hook_cmd(post_up),
            post_down: Self::build_hook_cmd(post_down),
            dns,
            table,
            fwmark,
            peers,
        }
    }

    fn build_hook_cmd(hooks: Vec<String>) -> String {
        if hooks.is_empty() {
            String::new()
        } else {
            format!(r#"bash -c "{}""#, hooks.join("; "))
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Peer {
    pub name: String,
    pub public_key: String,
    pub listen_port: u32,
    pub allowed_ips: Vec<String>,
    pub persistent_keepalive: u32,
    pub endpoint: Option<String>,
}

impl Peer {
    pub fn new(
        name: String,
        public_key: String,
        listen_port: u32,
        allowed_ips: Vec<String>,
        persistent_keepalive: u32,
        endpoint: Option<String>,
    ) -> Self {
        Self { name, public_key, listen_port, allowed_ips, persistent_keepalive, endpoint }
    }
}
