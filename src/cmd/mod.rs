mod peer;
mod repo;

use std::path::{Path, PathBuf};

use clap::Clap;
use eyre::{bail, Result};

use crate::config::Config;

use peer::Peer;
use repo::Repo;

/// Fireguard - wireguard autoconfiguration application
#[derive(Clap, Debug)]
#[clap(author, about, version)]
pub struct CommandLine {
    /// Fireguard subcommands
    #[clap(subcommand)]
    pub action: Action,
    /// Config directory
    #[clap(short = 'c', long = "config", default_value = "/etc/fireguard")]
    pub config: String,
}

impl CommandLine {
    pub fn exec(&self) -> Result<()> {
        match self.action {
            Action::Repo(ref action) => action.exec(self)?,
            Action::Peer(ref action) => action.exec(self)?,
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
    fn config_file(&self, config_dir: &str, repository: &str) -> PathBuf {
        Path::new(config_dir).join(repository).join("network.toml")
    }

    fn load_config(&self, config_dir: &str, repository: &str) -> Result<Config> {
        let path = self.config_file(config_dir, repository);
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
