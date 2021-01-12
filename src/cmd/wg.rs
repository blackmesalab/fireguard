use std::path::Path;
use std::thread;
use std::process;

use clap::Clap;
use color_eyre::eyre::{bail, Result};
use tokio::fs::read_to_string;
use tokio::task;
use signal_hook::consts::signal::*;
use signal_hook::iterator::Signals;

use crate::cmd::{Command, Fireguard};
use crate::wg::{BoringTun, WgConfig};

/// Wg - Wireguard management
#[derive(Clap, Debug)]
pub struct Wg {
    /// Wg subcommands
    #[clap(subcommand)]
    pub action: Action,
    /// Repository name
    #[clap(short = 'r', long = "repository")]
    pub repository: String,
}

#[derive(Clap, Debug)]
pub enum Action {
    /// Render the Wireguard configuration for the current host
    Render(Render),
    /// Start the Wireguard userspace tunnel
    Up(Up),
    /// Stop the Wireguard userspace tunnel
    Down(Down),
    /// Show the Wireguard userspace tunnel status and stats
    Status(Status),
}

impl Wg {
    async fn pre_checks(&self, fg: &Fireguard) -> Result<()> {
        let config = Path::new(&fg.config_dir);
        if config.is_dir() {
            Ok(())
        } else {
            bail!(
                "Please create Fireguard config directory {} as root: mkdir -p {} && chown {} {}",
                fg.config_dir,
                fg.config_dir,
                whoami::username(),
                fg.config_dir
            );
        }
    }
    pub async fn exec(&self, fg: &Fireguard) -> Result<()> {
        self.pre_checks(fg).await?;
        match self.action {
            Action::Render(ref action) => action.exec(fg, &self.repository).await?,
            Action::Up(ref action) => action.exec(fg, &self.repository).await?,
            Action::Down(ref action) => action.exec(fg, &self.repository).await?,
            Action::Status(ref action) => action.exec(fg, &self.repository).await?,
        }
        Ok(())
    }
}

/// Render the Wireguard configuration for the current host
#[derive(Clap, Debug)]
pub struct Render {
    /// User name
    #[clap(short = 'u', long = "username")]
    pub username: String,
    /// Peer name
    #[clap(short = 'p', long = "peername")]
    pub peername: String,
    /// Private key
    #[clap(short = 'P', long = "private-key")]
    pub private_key: String,
    /// Config file path
    #[clap(short = 'c', long = "config-dir", default_value = "/etc/wireguard")]
    pub config_dir: String,
}

impl Command for Render {}
impl Render {
    async fn pre_checks(&self, _fg: &Fireguard) -> Result<()> {
        let config = Path::new(&self.config_dir);
        if config.is_dir() {
            Ok(())
        } else {
            bail!(
                "Please create Wireguard config directory {} as root: mkdir -p {} && chown {} {}",
                self.config_dir,
                self.config_dir,
                whoami::username(),
                self.config_dir
            );
        }
    }

    pub async fn exec(&self, fg: &Fireguard, repository: &str) -> Result<()> {
        self.pre_checks(fg).await?;
        let config = self.load_config(repository, &fg.config_dir, &fg.config_file).await?;
        let wg_config_path = Path::new(&self.config_dir).join(&format!("{}.conf", repository));
        let wg_config = WgConfig::new(config.peers, repository, &self.username, &self.peername, &self.private_key)?;
        wg_config.render(&wg_config_path).await?;
        let data = read_to_string(&wg_config_path).await?;
        info!("Wireguard configuration written to {}:\n{}", wg_config_path.display(), data.trim());
        Ok(())
    }
}

/// Start the Wireguard tunnel for the current host after rendering the config
#[derive(Clap, Debug)]
pub struct Up {}

impl Command for Up {}
impl Up {
    pub async fn exec(&self, _fg: &Fireguard, repository: &str) -> Result<()> {
        let bt = BoringTun::new(repository)?;
        let mut signals = Signals::new(&[SIGTERM, SIGINT]).unwrap(); 
        bt.up().await?;
        let t = task::spawn(async move {
            for sig in signals.forever() {
                warn!("Received signal {:#?}, shutting down Boringtun", sig);
                bt.down().await.unwrap_or(());
                process::exit(0);
            }
        });
        t.await?;
        Ok(())
    }
}

/// Stop the Wireguard tunnel for the current host
#[derive(Clap, Debug)]
pub struct Down {}

impl Command for Down {}
impl Down {
    pub async fn exec(&self, _fg: &Fireguard, repository: &str) -> Result<()> {
        let bt = BoringTun::new(repository)?;
        Ok(bt.down().await?)
    }
}

/// Start the Wireguard tunnel for the current host after rendering the config
#[derive(Clap, Debug)]
pub struct Status {}

impl Command for Status {}
impl Status {
    pub async fn exec(&self, _fg: &Fireguard, repository: &str) -> Result<()> {
        let bt = BoringTun::new(repository)?;
        bt.status().await?;
        Ok(())
    }
}

