use std::io;
use std::path::Path;

use clap::Clap;
use color_eyre::eyre::{bail, Result};
use tokio::fs;
use tokio::stream::StreamExt;

use crate::cmd::{Command, Fireguard};
use crate::shell::Shell;

/// Repo - trust repositories management
#[derive(Clap, Debug)]
pub struct Repo {
    /// Repo subcommands
    #[clap(subcommand)]
    pub action: Action,
    /// Repository name
    #[clap(short = 'r', long = "repository")]
    pub repository: Option<String>,
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
    /// Commit a Fireguard trust repository
    Commit(Commit),
}

impl Command for Repo {}
impl Repo {
    async fn pre_checks(&self, fg: &Fireguard) -> Result<()> {
        let config = Path::new(&fg.config_dir);
        if config.is_dir() {
            Ok(())
        } else {
            bail!(
                "Please create Fireguar config directory {} as root: mkdir -p {} && chown {} {}",
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
            Action::List(_) => {}
            _ => {
                if self.repository.is_none() {
                    bail!("--repository / -r argument is mandatory");
                }
            }
        }
        match self.action {
            Action::Clone(ref action) => action.exec(fg, &self.repository.as_ref().unwrap()).await?,
            Action::List(ref action) => action.exec(fg).await?,
            Action::Remove(ref action) => action.exec(fg, &self.repository.as_ref().unwrap()).await?,
            Action::Pull(ref action) => action.exec(fg, &self.repository.as_ref().unwrap()).await?,
            Action::Commit(ref action) => action.exec(fg, &self.repository.as_ref().unwrap()).await?,
        }
        Ok(())
    }
}

/// Clone a new repository with the chain of trust
#[derive(Clap, Debug)]
pub struct Clone {}

impl Command for Clone {}
impl Clone {
    pub async fn exec(&self, fg: &Fireguard, repository: &str) -> Result<()> {
        let repo_name = match Path::new(repository).file_stem() {
            Some(name) => name,
            None => bail!("Unable to get repository name from url {}", repository),
        };
        let path = Path::new(&fg.config_dir);
        let config_path = path.to_path_buf().join(repo_name);

        info!("Creating Fireguard repository directory {}", config_path.display());
        fs::create_dir_all(&config_path).await?;

        info!("Cloning trust repository {} in Fireguard config directory {}", repository, config_path.display());
        let result = Shell::exec("git", &format!("clone {} {}", repository, config_path.display()), None, false).await;
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

impl Command for List {}
impl List {
    pub async fn exec(&self, fg: &Fireguard) -> Result<()> {
        match fs::read_dir(&fg.config_dir).await {
            Ok(dir) => {
                let repos = dir.map(|res| res.map(|e| e.path())).collect::<Result<Vec<_>, io::Error>>().await?;
                info!("Avalilable trust repositoriers in Fireguard config directory: {:?}", repos);
                Ok(())
            }
            Err(e) => {
                bail!("Error listing trust repositorires: {}", e);
            }
        }
    }
}

/// Delete a Fireguard trust repository
#[derive(Clap, Debug)]
pub struct Remove {}

impl Command for Remove {}
impl Remove {
    pub async fn exec(&self, fg: &Fireguard, repository: &str) -> Result<()> {
        let path = Path::new(&fg.config_dir).join(repository);
        info!("Deleting trust repository {}", path.display());
        match fs::remove_dir_all(&path).await {
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
pub struct Pull {}

impl Command for Pull {}
impl Pull {
    pub async fn exec(&self, fg: &Fireguard, repository: &str) -> Result<()> {
        let path = Path::new(&fg.config_dir).join(repository);
        info!("Updating trust repository {}", path.display());
        let result = Shell::exec("git", "pull", Some(&format!("{}", path.display())), false).await;
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
pub struct Commit {}

impl Command for Commit {}
impl Commit {
    pub async fn exec(&self, fg: &Fireguard, repository: &str) -> Result<()> {
        let path = Path::new(&fg.config_dir).join(repository);
        info!("Committing trust repository {}", path.display());
        Ok(())
    }
}
