use std::fs;
use std::io;
use std::path::Path;

use clap::Clap;
use eyre::{bail, Result};

use crate::cmd::CommandLine;
use crate::shell::Shell;

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
    Commit(Commit),
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
}

impl Clone {
    pub fn exec(&self, cmd: &CommandLine) -> Result<()> {
        let path = Path::new(&cmd.config).join(&self.repository);
        info!("Cloning trust repository {} in Fireguard config directory {}", self.repository, cmd.config);
        let result = Shell::exec("git", &format!("clone {} {}", self.repository, path.display()), None, None, false);
        if result.success() {
            info!("Trust repository cloned in {}", path.display());
            Ok(())
        } else {
            bail!("Error cloning trust repository: {}", result.stderr());
        }
    }
}

/// List the available trust repositories
#[derive(Clap, Debug)]
pub struct List {}

impl List {
    pub fn exec(&self, cmd: &CommandLine) -> Result<()> {
        match fs::read_dir(&cmd.config) {
            Ok(dir) => {
                let repos = dir.map(|res| res.map(|e| e.path())).collect::<Result<Vec<_>, io::Error>>()?;
                info!("Avalilable trust repositoriers in Fireguard config directory: {:?}", repos);
                Ok(())
            },
            Err(e) => {
                bail!("Error listing trust repositorires: {}", e);
            }
        }
    }
}

/// Delete a Fireguard trust repository
#[derive(Clap, Debug)]
pub struct Remove {
    /// Repository name
    #[clap(short = 'r', long = "repository")]
    pub repository: String,
}

impl Remove {
    pub fn exec(&self, cmd: &CommandLine) -> Result<()> {
        let path = Path::new(&cmd.config).join(&self.repository);
        info!("Deleting trust repository {}", path.display());
        match fs::remove_dir_all(&path) {
            Ok(_) => {
                info!("Deleted Fireguard trust repository {}", path.display());
                Ok(())
            }
            Err(e) => {
                bail!("Error removing trust repository {}: {}", path.display(), e);
            }
        }
    }
}

/// Update a Fireguard trust repository
#[derive(Clap, Debug)]
pub struct Pull {
    /// Repository name
    #[clap(short = 'r', long = "repository")]
    pub repository: String,
}

impl Pull {
    pub fn exec(&self, cmd: &CommandLine) -> Result<()> {
        let path = Path::new(&cmd.config).join(&self.repository);
        info!("Updating trust repository {}", path.display());
        let result = Shell::exec("gi", "pull", None, Some(&format!("{}", path.display())), false);
        if result.success() {
            info!("Trust repository {} successfully updated:", path.display());
            Ok(())
        } else {
            bail!("Error updating trust repository {}: {}", path.display(), result.stderr());
        }
    }
}

/// Commit a Fireguard trust repository
#[derive(Clap, Debug)]
pub struct Commit {
    /// Repository name
    #[clap(short = 'r', long = "repository")]
    pub repository: String,
}

impl Commit {
    pub fn exec(&self, cmd: &CommandLine) -> Result<()> {
        let path = Path::new(&cmd.config).join(&self.repository);
        info!("Committing trust repository {}", path.display());
        Ok(())
    }
}
