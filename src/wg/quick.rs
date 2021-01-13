use std::collections::HashMap;

use color_eyre::eyre::{bail, Result};

use crate::shell::Shell;

#[derive(Default, Debug, Clone)]
pub struct WgStatus {
    pub repository: String,
    pub public_key: String,
    pub private_key: String,
    pub listen_port: u32,
    pub fwmark: u32,
    pub table: Option<u32>,
    pub peers: Vec<WgPeer>,
}

impl WgStatus {
    pub fn new(status: &str, peers: Vec<WgPeer>) -> Result<Self> {
        let status = status.split(" ").collect::<Vec<&str>>();
        if ! status.is_empty() {
            Ok(Self { 
                repository: status[0].to_string(), 
                public_key: status[1].to_string(), 
                private_key: status[2].to_string(), 
                listen_port: status[3].parse::<u32>()?, 
                fwmark: status[4].parse::<u32>()?, 
                table: None, 
                peers
            })
        } else {
            Ok(WgStatus::default())
        }
    }
}

#[derive(Default, Debug, Clone)]
pub struct WgPeer {
    endpoint: Option<String>,
    public_key: String,
    latest_handshake: Option<u64>,
    tranfer_rx: Option<u128>,
    tranfer_tx: Option<u128>,
    persistent_keepalive: Option<u32>,
    allowed_ips: Vec<String>,
}

impl WgPeer {}

pub struct WgQuick {
    repository: String,
}

impl WgQuick {
    pub fn new(repository: &str) -> Result<Self> {
        if !Shell::runnable("wg-quick") {
            bail!("Missing command dependency")
        }
        Ok(WgQuick { repository: repository.to_string() })
    }

    pub async fn up(&self) -> Result<()> {
        info!("Starting new Wireguard instance for repository {}", self.repository);
        let result =
            Shell::exec("wg-quick", &format!("up {}", self.repository), None, true)
                .await;
        if result.success() {
            info!("Wireguard instance started successfully:\n{}", result.stderr());
            Ok(())
        } else {
            bail!("Error running Wireguard instance: {}", result.stderr());
        }
    }

    pub async fn down(&self) -> Result<()> {
        info!("Stopping Wireguard instance for repository {}", self.repository);
        let result = Shell::exec(
            "wg-quick",
            &format!("down {}", self.repository),
            None,
            true,
        )
        .await;
        if result.success() {
            info!("Wireguard instance stopped successfully:\n{}", result.stderr());
            Ok(())
        } else {
            bail!("Error stopping Wireguard instance: {}", result.stderr());
        }
    }

    pub async fn status(&self) -> Result<()> {
        let result = Shell::exec("wg", &format!("show {}", self.repository), None, true).await;
        if result.success() {
            info!("Wireguard statistics for repository {}:\n{}", self.repository, result.stdout().trim());
            Ok(())
        } else {
            bail!("Error checking Wireguard instance status: {}", result.stderr());
        }
    }
}
