use std::process;

use clap::Clap;
use color_eyre::eyre::Result;
use signal_hook::consts::signal::*;
use signal_hook::iterator::Signals;
use tokio::task;

use crate::cmd::repo::Clone;
use crate::cmd::wg::{Down, Render, Up};
use crate::cmd::{Command, Fireguard};

/// Daemon - Manage Fireguard daemon
#[derive(Clap, Debug)]
pub struct Daemon {
    /// Peer subcommands
    #[clap(subcommand)]
    pub action: Action,
}

#[derive(Clap, Debug)]
pub enum Action {
    /// Run the Fireguard daemon
    Serve(Serve),
}

impl Command for Daemon {}
impl Daemon {
    pub async fn exec(&self, fg: &Fireguard) -> Result<()> {
        match self.action {
            Action::Serve(ref action) => action.exec(fg).await?,
        }
        Ok(())
    }
}

/// Run Fireguard daemon
#[derive(Clap, Debug)]
pub struct Serve {
    /// Repository URL
    #[clap(short = 'u', long = "repository-url")]
    pub repository_url: String,
    /// Repository name
    #[clap(short = 'r', long = "repository-name")]
    pub repository_name: String,
    /// Private key
    #[clap(short = 'P', long = "private-key")]
    pub private_key: String,
    /// User name
    #[clap(short = 'u', long = "username")]
    pub username: String,
    /// Peer name
    #[clap(short = 'p', long = "peername")]
    pub peername: String,
    /// Config file path
    #[clap(short = 'c', long = "config-dir", default_value = "/etc/wireguard")]
    pub config_dir: String,
}

impl Command for Serve {}
impl Serve {
    pub async fn exec(&self, fg: &Fireguard) -> Result<()> {
        info!("Starting Fireguard daemon in foreground");
        let clone = Clone {};
        clone.exec(fg, &self.repository_url).await?;
        let render = Render {
            username: self.username.clone(),
            peername: self.peername.clone(),
            private_key: self.private_key.clone(),
            config_dir: self.config_dir.clone(),
        };
        render.exec(fg, &self.repository_name).await?;
        let config = self.load_config(&self.repository_name, &fg.config_dir, &fg.config_file).await?;
        config.write_pid_file("fireguard", process::id()).await?;
        let up = Up {};
        up.exec(None, &self.repository_name).await?;
        let mut signals = Signals::new(&[SIGTERM, SIGINT]).unwrap();
        let repository_name = self.repository_name.clone();
        let t = task::spawn(async move {
            for sig in signals.forever() {
                warn!("Received signal {:#?}, shutting down Fireguard", sig);
                let down = Down {};
                down.exec(None, &repository_name).await.unwrap_or(());
                return;
            }
        });
        t.await?;
        Ok(())
    }
}
