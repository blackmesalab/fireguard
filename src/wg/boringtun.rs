use std::collections::HashMap;

use chrono::DateTime;
use color_eyre::eyre::{bail, Result};

use crate::shell::Shell;

const WG_QUICK_USERSPACE_IMPLEMENTATION: &str = "borintun";
const WG_QUICK_SUDO: &str = "1";

#[derive(Default, Debug, Clone)]
pub struct BoringStatus {
    pub repository: String,
    pub public_key: String,
    pub private_key: String,
    pub listen_port: u32,
    pub fwmark: u32,
    pub table: Option<u32>,
    pub peers: Vec<BoringPeer>,
}

impl BoringStatus {
    pub fn new(status: &str, peers: Vec<BoringPeer>) -> Result<Self> {
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
            Ok(BoringStatus::default())
        }
    }
}

#[derive(Default, Debug, Clone)]
pub struct BoringPeer {
    endpoint: Option<String>,
    public_key: String,
    latest_handshake: Option<u64>,
    tranfer_rx: Option<u128>,
    tranfer_tx: Option<u128>,
    persistent_keepalive: Option<u32>,
    allowed_ips: Vec<String>,
}

impl BoringPeer {}

pub struct BoringTun {
    repository: String,
}

impl BoringTun {
    pub fn new(repository: &str) -> Result<Self> {
        if !Shell::runnable("boringtun") || !Shell::runnable("wg-quick") {
            bail!("Missing command dependency")
        }
        Ok(BoringTun { repository: repository.to_string() })
    }

    fn build_wg_quick_env(&self) -> HashMap<&str, &str> {
        let mut env = HashMap::new();
        env.insert("WG_QUICK_USERSPACE_IMPLEMENTATION", WG_QUICK_USERSPACE_IMPLEMENTATION);
        env.insert("WG_SUDO", WG_QUICK_SUDO);
        debug!("Injected WG_QUICK_USERSPACE_IMPLEMENTATION and WG_SUDO variables into command environment");
        env
    }

    pub async fn up(&self) -> Result<()> {
        info!("Starting new Boringtun instance for repository {}", self.repository);
        let result =
            Shell::exec_with_env("wg-quick", &format!("up {}", self.repository), None, self.build_wg_quick_env(), true)
                .await;
        if result.success() {
            info!("Boringtun instance started successfully:\n{}", result.stdout());
            Ok(())
        } else {
            bail!("Error running Boringtun instance: {}", result.stderr());
        }
    }

    pub async fn down(&self) -> Result<()> {
        info!("Stopping Boringtun instance for repository {}", self.repository);
        let result = Shell::exec_with_env(
            "wg-quick",
            &format!("down {}", self.repository),
            None,
            self.build_wg_quick_env(),
            true,
        )
        .await;
        if result.success() {
            info!("Boringtun instance stopped successfully:\n{}", result.stdout());
            Ok(())
        } else {
            bail!("Error stopping Boringtun instance: {}", result.stderr());
        }
    }

    pub async fn status(&self) -> Result<()> {
        let result = Shell::exec("wg", &format!("show {}", self.repository), None, true).await;
        if result.success() {
            info!("Boringtun statistics for repository {}:\n{}", self.repository, result.stdout().trim());
            Ok(())
        } else {
            bail!("Error checking Boringtun instance status: {}", result.stderr());
        }
    }
}
