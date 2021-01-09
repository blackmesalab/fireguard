use std::fs;
use std::io;
use std::path::Path;

use clap::Clap;
use eyre::{bail, Result};

use crate::cmd::CommandLine;
use crate::config::Config;
use crate::wg::key::WgKeys;

/// Peer - peers management for a trust repository
#[derive(Clap, Debug)]
pub struct Peer {
    /// Peer subcommands
    #[clap(subcommand)]
    pub action: Action,
}

#[derive(Clap, Debug)]
pub enum Action {
    /// List the available peers in this trust repository
    List(List),
    /// Add a new peer
    Add(Add),
}

impl Peer {
    pub fn exec(&self, cmd: &CommandLine) -> Result<()> {
        match self.action {
            Action::List(ref action) => action.exec(cmd)?,
            Action::Add(ref action) => action.exec(cmd)?,
        }
        Ok(())
    }
}

/// List the available peers in this trust repository
#[derive(Clap, Debug)]
pub struct List {
    /// Repository name
    #[clap(short = 'r', long = "repository")]
    pub repository: String,
}

impl List {
    pub fn exec(&self, cmd: &CommandLine) -> Result<()> {
        let path = format!("{}/{}/network.toml", cmd.config, self.repository);
        info!("Loading network topology from {}", path);
        match Config::load(&path) {
            Ok(hosts) => {
                info!("Available peers in {}: {:?}", self.repository, hosts.peers().keys());
                Ok(())
            }
            Err(e) => {
                bail!("Error listing available peers in {}: {}", self.repository, e);
            }
        }
    }
}

/// List the available peers in this trust repository
#[derive(Clap, Debug)]
pub struct Add {
    /// Peer name
    #[clap(short = 'p', long = "peer")]
    pub peer: String,
    /// Repository name
    #[clap(short = 'r', long = "repository")]
    pub repository: String,
}

impl Add {
    pub fn exec(&self, cmd: &CommandLine) -> Result<()> {
        let keys = WgKeys::generate()?;
        info!("Generated public key for {}/{}: {}", self.repository, self.peer, keys.public);
        Ok(())
    }
}
