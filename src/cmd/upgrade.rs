use clap::Clap;
use color_eyre::eyre::{bail, Result};

use crate::cmd::{Command, Fireguard};
use crate::shell::Shell;
use crate::utils::install_wireguard_kernel_module;

/// Upgrade - Upgrade Fireguard binary and its Docker container
#[derive(Clap, Debug)]
pub struct Upgrade {
    /// Upgrade subcommands
    #[clap(subcommand)]
    pub action: Action,
}

#[derive(Clap, Debug)]
pub enum Action {
    // /// Check if new version is available
    // Check(Check),
    // /// Apply the available upgrade
    // Apply(Apply),
}

impl Command for Upgrade {}
impl Upgrade {
    pub async fn exec(&self, fg: &Fireguard) -> Result<()> {
        Ok(())
    }
}
