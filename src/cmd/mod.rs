mod daemon;
mod dns;
mod docker;
mod peer;
mod repo;
mod upgrade;
mod wg;

use std::env;
use std::path::{Path, PathBuf};

use async_trait::async_trait;
use clap::Clap;
use color_eyre::eyre::{bail, Result};

use crate::config::Config;

use daemon::Daemon;
use dns::Dns;
use docker::Docker;
use peer::Peer;
use repo::Repo;
use upgrade::Upgrade;
use wg::Wg;

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
    #[clap(short = 'C', long = "config-file", default_value = "nodes.toml")]
    pub config_file: String,
    /// Enable debug logging
    #[clap(short = 'D', long = "debug")]
    pub debug: bool,
    /// Cmdline args vec, do not use, it is autofilled
    #[clap(long = "args", default_values = &[])]
    pub args: Vec<String>,
}

impl Fireguard {
    async fn pre_checks(&mut self) -> Result<()> {
        let config = Path::new(&self.config_dir);
        if config.is_dir() {
            let mut args = env::args().collect::<Vec<String>>();
            debug!("Command line args: [{}]", args.join(", "));
            if args[0].starts_with("target/") {
                args.remove(0);
            }
            for (idx, arg) in args.iter().enumerate() {
                if arg == "docker" {
                    args.remove(idx);
                    break;
                }
            }
            debug!("Command line args after sanification: [{}]", args.join(", "));
            self.args = args;
            Ok(())
        } else {
            bail!(
                "Please create directory {} as root: mkdir -p {} && chown {} {}",
                self.config_dir,
                self.config_dir,
                whoami::username(),
                self.config_dir
            );
        }
    }

    pub async fn exec(&mut self) -> Result<()> {
        self.pre_checks().await?;
        match self.action {
            Action::Repo(ref action) => action.exec(self).await?,
            Action::Peer(ref action) => action.exec(self).await?,
            Action::Wg(ref action) => action.exec(self).await?,
            Action::Dns(ref action) => action.exec(self).await?,
            Action::Docker(ref action) => action.exec(self).await?,
            Action::Daemon(ref action) => action.exec(self).await?,
            Action::Upgrade(ref action) => action.exec(self).await?,
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
    /// Wireguard management
    Wg(Wg),
    /// DNS management
    Dns(Dns),
    /// Docker management
    Docker(Docker),
    /// Daemon management
    Daemon(Daemon),
    /// Upgrade management
    Upgrade(Upgrade),
}

#[async_trait]
pub trait Command {
    fn config_file(&self, repository: &str, config_dir: &str, config_file: &str) -> PathBuf {
        Path::new(config_dir).join(repository).join(config_file)
    }

    async fn load_config(&self, repository: &str, config_dir: &str, config_file: &str) -> Result<Config> {
        let path = self.config_file(repository, config_dir, config_file);
        debug!("Loading network topology from {}", path.display());
        match Config::load(&path).await {
            Ok(hosts) => {
                debug!("Available peers in {}: {:?}", repository, hosts.peers.keys());
                Ok(hosts)
            }
            Err(e) => {
                bail!("Error listing available peers in {}: {}", repository, e)
            }
        }
    }
}
