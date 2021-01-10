mod peer;
mod repo;

use std::path::{Path, PathBuf};

use clap::Clap;
use color_eyre::eyre::{bail, Result};

use crate::config::Config;

use peer::Peer;
use repo::Repo;

/// Fireguard - wireguard autoconfiguration application
#[derive(Clap, Debug)]
#[clap(author, about, version)]
pub struct Fireguard {
    /// Fireguard subcommands
    #[clap(subcommand)]
    pub action: Action,
    /// Config directory
    #[clap(short = 'c', long = "config-dir", default_value = "/etc/fireguard")]
    pub config_dir: String,
    /// Config file
    #[clap(short = 'C', long = "config-file", default_value = "network.toml")]
    pub config_file: String,
}

impl Fireguard {
    pub async fn exec(&self) -> Result<()> {
        let config = Path::new(&self.config_dir);
        if !config.is_dir() {
            bail!(
                "Please create directory {}: mkdir -p {} && chown {} {}",
                self.config_dir,
                self.config_dir,
                whoami::username(),
                self.config_dir
            );
        }
        match self.action {
            Action::Repo(ref action) => action.exec(self).await?,
            Action::Peer(ref action) => action.exec(self).await?,
        }
        Ok(())
    }
}

#[derive(Clap, Debug)]
pub enum Action {
    /// Trust repositories management
    Repo(Repo),
    /// Peers management
    Peer(Peer),
}

pub trait Command {
    fn config_file(&self, repository: &str, config_dir: &str, config_file: &str) -> PathBuf {
        Path::new(config_dir).join(repository).join(config_file)
    }

    fn load_config(&self, repository: &str, config_dir: &str, config_file: &str) -> Result<Config> {
        let path = self.config_file(repository, config_dir, config_file);
        info!("Loading network topology from {}", path.display());
        match Config::load(&path) {
            Ok(hosts) => {
                info!("Available peers in {}: {:?}", repository, hosts.peers.keys());
                Ok(hosts)
            }
            Err(e) => {
                bail!("Error listing available peers in {}: {}", repository, e)
            }
        }
    }
}
