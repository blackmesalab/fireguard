use std::time::Duration;
use std::{env, process};

use clap::Clap;
use color_eyre::eyre::{bail, Result};
use nix::sys::signal;
use nix::unistd::Pid;
use signal_hook::consts::signal::*;
use signal_hook::iterator::Signals;
use tokio::{fs, task};

use crate::cmd::peer::List;
use crate::cmd::repo::Clone;
use crate::cmd::wg::{Down, Render, Status as WgStatus, Up};
use crate::cmd::{Command, Fireguard};
use crate::config::Config;
use crate::upgrade::UpgradeBin;

/// Daemon - Manage Fireguard daemon
#[derive(Clap, Debug)]
pub struct Daemon {
    /// Peer subcommands
    #[clap(subcommand)]
    pub action: Action,
    /// Repository name
    #[clap(short = 'r', long = "repository")]
    pub repository: String,
}

#[derive(Clap, Debug)]
pub enum Action {
    /// Run Fireguard daemon. If `repository-url` is set a new clone in the `repository` folder will be
    /// peformed. If `private-key` is set, a new render of the Wireguard configuration will be
    /// performed. A signal handler is installer for TERM and INT with graceful shutdown. HUP for
    /// now does nothing. The main process PID is stored in a PID file.
    Serve(Serve),
    /// Stop the Fireguard daemon by sending a SIGTERM to its PID from the PID file.
    Stop(Stop),
    /// Fireguard daemon status, exposing information about the different running components and
    /// the current configuration.
    Status(Status),
}

impl Command for Daemon {}
impl Daemon {
    pub async fn exec(&self, fg: &Fireguard) -> Result<()> {
        match self.action {
            Action::Serve(ref action) => action.exec(fg, &self.repository).await?,
            Action::Stop(ref action) => action.exec(fg, &self.repository).await?,
            Action::Status(ref action) => action.exec(fg, &self.repository).await?,
        }
        Ok(())
    }
}

/// Run Fireguard daemon. If `repository-url` is set a new clone in the `repository` folder will be
/// peformed. If `private-key` is set, a new render of the Wireguard configuration will be
/// performed.
#[derive(Clap, Debug)]
pub struct Serve {
    /// Repository URL
    #[clap(short = 'U', long = "repository-url")]
    pub repository_url: Option<String>,
    /// Private key
    #[clap(short = 'P', long = "private-key", requires_all = &["username", "peername"])]
    pub private_key: Option<String>,
    /// User name
    #[clap(short = 'u', long = "username")]
    pub username: Option<String>,
    /// Peer name
    #[clap(short = 'p', long = "peername")]
    pub peername: Option<String>,
    /// Wireguard config file path
    #[clap(short = 'c', long = "config-dir", default_value = "/etc/wireguard")]
    pub config_dir: String,
    /// How much to wait between upgrade checks
    #[clap(short = 'w', long = "wait-between-checks", default_value = "43200")]
    pub wait_between_checks: u64,
    /// Github releases URL
    #[clap(
        short = 'r',
        long = "release-url",
        default_value = "https://api.github.com/repos/blackmesalab/fireguard/releases/latest"
    )]
    pub release_url: String,
}

impl Command for Serve {}
impl Serve {
    async fn handle_signals(&self, config: Config, repository: String) -> Result<()> {
        let mut signals = Signals::new(&[SIGTERM, SIGINT])?;
        let t = task::spawn(async move {
            for sig in signals.forever() {
                warn!("Received signal {:#?}, shutting down Fireguard", sig);
                let down = Down {};
                down.exec(None, &repository).await.unwrap_or_else(|_| error!("Unable to shut down Wireguard"));
                config
                    .remove_pid_file("fireguard")
                    .await
                    .unwrap_or_else(|_| error!("Unable to remove PID file for wireguard"));
                return;
            }
        });
        t.await?;
        Ok(())
    }

    pub async fn exec(&self, fg: &Fireguard, repository: &str) -> Result<()> {
        let upgrade = UpgradeBin::new(
            Duration::from_secs(self.wait_between_checks),
            &self.release_url,
            env!("CARGO_PKG_VERSION"),
        );
        if let Some(pid) = fg.old_pid.as_ref() {
            let pid = pid.parse::<i32>()?;
            upgrade.terminate_old_process(pid)?;
            upgrade.flip_binary_on_disk(env::current_exe()?).await?;
        }
        info!("Starting Fireguard daemon in foreground");
        if let Some(repo) = self.repository_url.as_ref() {
            let clone = Clone {};
            clone.exec(fg, repo).await?;
        }
        if let Some(pkey) = self.private_key.as_ref() {
            let render = Render {
                username: self.username.clone().unwrap(),
                peername: self.peername.clone().unwrap(),
                private_key: pkey.clone(),
                config_dir: self.config_dir.clone(),
            };
            render.exec(fg, repository).await?;
        }
        let config = self.load_config(repository, &fg.config_dir, &fg.config_file).await?;
        let up = Up {};
        up.exec(None, repository).await?;
        let repository_name = repository.to_owned();
        config.write_pid_file("fireguard", process::id()).await?;
        upgrade.run_in_background(&fg.args).await?;
        info!("Fireguard daemon started successfully");
        self.handle_signals(config, repository_name).await?;
        Ok(())
    }
}

/// Stop Fireguard daemon
#[derive(Clap, Debug)]
pub struct Stop {}

impl Command for Stop {}
impl Stop {
    pub async fn exec(&self, fg: &Fireguard, repository: &str) -> Result<()> {
        info!("Stopping foreground Fireguard daemon");
        let config = self.load_config(repository, &fg.config_dir, &fg.config_file).await?;
        let pid = fs::read_to_string(&config.pid_file("fireguard")).await?.parse::<i32>()?;
        debug!("Sending SIGTERM to PID {}", pid);
        signal::kill(Pid::from_raw(pid), signal::SIGTERM)?;
        Ok(())
    }
}

/// Stop Fireguard daemon
#[derive(Clap, Debug)]
pub struct Status {}

impl Command for Status {}
impl Status {
    pub async fn exec(&self, fg: &Fireguard, repository: &str) -> Result<()> {
        let config = self.load_config(repository, &fg.config_dir, &fg.config_file).await?;
        match fs::read_to_string(config.pid_file("fireguard")).await {
            Ok(pid) => {
                info!("Fireguard daemon is running with PID {}", pid);
                let peer_list = List {};
                peer_list
                    .exec(fg, config, repository)
                    .await
                    .unwrap_or_else(|_| error!("Unable to read {} peer list", repository));
                let status = WgStatus {};
                status
                    .exec(None, repository)
                    .await
                    .unwrap_or_else(|_| error!("Unable to get {} Wireguard status", repository));
                Ok(())
            }
            Err(_) => {
                bail!("Fireguard PID not found, did you start Fireguard with `daemon serve` command?");
            }
        }
    }
}
