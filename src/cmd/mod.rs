use clap::Clap;
use eyre::Result;

mod repo;
mod peer;

use repo::Repo;
use peer::Peer;

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
    Peer(Peer)
}
