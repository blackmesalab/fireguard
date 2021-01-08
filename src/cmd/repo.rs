use std::fs;
use std::path::Path;

use eyre::{bail, Result};
use clap::Clap;

use crate::shell::Shell;
use crate::cmd::CommandLine;

/// Repo - trust repositories management
#[derive(Clap, Debug)]
pub struct Repo {
    /// Repo subcommands
    #[clap(subcommand)]
    pub action: Action,
}

#[derive(Clap, Debug)]
pub enum Action {
    /// Clone a Fireguard trust repository
    Clone(Clone),
    /// List the available Fireguard trust repositories
    List(List),
    /// Delete a Fireguard trust repository
    Remove(Remove),
    /// Update a Fireguard trust repository
    Pull(Pull),
    /// Update a Fireguard trust repository
    Commit(Commit)
}

impl Repo {
    pub fn exec(&self, cmd: &CommandLine) -> Result<()> {
        match self.action {
            Action::Clone(ref action) => action.exec(cmd)?,
            Action::List(ref action) => action.exec(cmd)?,
            Action::Remove(ref action) => action.exec(cmd)?,
            Action::Pull(ref action) => action.exec(cmd)?,
            Action::Commit(ref action) => action.exec(cmd)?,
        }
        Ok(())
    }
}

/// Clone a new repository with the chain of trust
#[derive(Clap, Debug)]
pub struct Clone {
    /// Repository URL
    #[clap(short = 'r', long = "repository")]
    pub repository: String,
    /// Repository Name
    #[clap(short = 'n', long = "name")]
    pub name: String
}

impl Clone {
    pub fn exec(&self, cmd: &CommandLine) -> Result<()> {
        let path = Path::new(&cmd.config).join(&self.name);
        info!("Cloning trust repository {} in Fireguard config directory {}", self.repository, cmd.config);
        let result = Shell::exec("git", &format!("clone {} {}", self.repository, path.display()), None);
        if result.success() {
            info!("Trust repository cloned in {}:\n{}", path.display(), result.stdout());
            Ok(())
        } else {
            error!("Error cloning trust repository:\n{}", result.stderr());
            bail!("Error cloning trust repository");
        }
    }
}

/// List the available trust repositories
#[derive(Clap, Debug)]
pub struct List {}

impl List {
    pub fn exec(&self, cmd: &CommandLine) -> Result<()> {
        let result = Shell::exec("ls", &format!("{}/", &cmd.config), None);
        if result.success() {
            info!("Avalilable trust repositoriers in Fireguard config directory:\n{}", result.stdout().trim_end());
            Ok(())
        } else {
            error!("Error listing trust repositorires:\n{}", result.stderr());
            bail!("Error listing trust repositories");
        }
    }
}

/// Delete a Fireguard trust repository
#[derive(Clap, Debug)]
pub struct Remove {
    /// Repository Name
    #[clap(short = 'n', long = "name")]
    pub name: String,
}

impl Remove {
    pub fn exec(&self, cmd: &CommandLine) -> Result<()> {
        let path = Path::new(&cmd.config).join(&self.name);
        info!("Deleting trust repository {}", path.display());
        match fs::remove_dir_all(&path) {
            Ok(_) => {
                info!("Deleted Fireguard trust repository {}", path.display());
                Ok(())
            }
            Err(e) => {
                error!("Error removing trust repository {}: {}", path.display(), e);
                bail!("Error removing trust repository");
            }
        }
    }
}

/// Update a Fireguard trust repository
#[derive(Clap, Debug)]
pub struct Pull {
    /// Repository Name
    #[clap(short = 'n', long = "name")]
    pub name: String,
}

impl Pull {
    pub fn exec(&self, cmd: &CommandLine) -> Result<()> {
        let path = Path::new(&cmd.config).join(&self.name);
        info!("Updating trust repository {}", path.display());
        let result = Shell::exec("git", "pull", Some(&format!("{}", path.display())));
        if result.success() {
            info!("Trust repository {} successfully updated:\n{}", path.display(), result.stdout().trim_end());
            Ok(())
        } else {
            error!("Error updating trust repository {}:\n{}", path.display(), result.stderr());
            bail!("Error updating trust repository");
        }
    }
}

/// Commit a Fireguard trust repository
#[derive(Clap, Debug)]
pub struct Commit {
    /// Repository Name
    #[clap(short = 'n', long = "name")]
    pub name: String,
}

impl Commit {
    pub fn exec(&self, cmd: &CommandLine) -> Result<()> {
        let path = Path::new(&cmd.config).join(&self.name);
        info!("Committing trust repository {}", path.display());
        Ok(())
    }
}
