use color_eyre::eyre::{bail, Result};

use crate::shell::Shell;

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
        let result = Shell::exec("wg-quick", &format!("up {}", self.repository), None, true).await;
        if result.success() {
            info!("Wireguard instance started successfully:\n{}", result.stderr());
            Ok(())
        } else {
            bail!("Error running Wireguard instance: {}", result.stderr());
        }
    }

    pub async fn down(&self) -> Result<()> {
        info!("Stopping Wireguard instance for repository {}", self.repository);
        let result = Shell::exec("wg-quick", &format!("down {}", self.repository), None, true).await;
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
