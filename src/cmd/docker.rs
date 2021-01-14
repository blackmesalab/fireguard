use clap::Clap;
use color_eyre::eyre::{bail, Result};

use crate::cmd::Fireguard;
use crate::cmd::{Dns, Peer, Repo, Wg};
use crate::shell::Shell;

/// Docker - Docker command management
#[derive(Clap, Debug)]
pub struct Docker {
    /// Docker subcommands
    #[clap(subcommand)]
    pub action: Action,
    /// Docker image name
    #[clap(short = 'd', long = "docker-image-name", default_value = "blackmesalab/fireguard")]
    pub docker_image_name: String,
    /// Docker image version
    #[clap(short = 'v', long = "docker-image-version", default_value = "latest")]
    pub docker_image_version: String,
    /// Volumes to mount inside the container
    #[clap(short = 'V', long = "docker-volumes")]
    pub docker_volumes: Option<Vec<String>>,
}

#[derive(Clap, Debug)]
pub enum Action {
    /// Trust repositories management under Docker
    Repo(Repo),
    /// Peers management under Docker
    Peer(Peer),
    /// Wireguard management under Docker
    Wg(Wg),
    /// DNS management under Docker
    Dns(Dns),
}

impl Docker {
    fn docker_image(&self) -> String {
        format!("{}:{}", self.docker_image_name, self.docker_image_version)
    }

    pub async fn exec(&self, fg: &Fireguard) -> Result<()> {
        let args = fg.args.join(" ");
        let mut docker_cmd = format!("run -ti --rm --privileged --net=host");
        if let Some(volumes) = self.docker_volumes.as_ref() {
            docker_cmd += &format!(" -v {}", volumes.join("-v "));
        } else {
            docker_cmd += " -v /etc/fireguard:/etc/fireguard";
        } 
        docker_cmd += &format!(" {} fireguard {}", self.docker_image(), args); 
        info!("Running command `{}` inside Docker container {}", args, self.docker_image());
        let result = Shell::exec(
            "docker",
            &docker_cmd,
            None,
            false,
        )
        .await;
        if result.success() {
            debug!("Command {} succeeded inside Docker container {}", args, self.docker_image());
            Ok(())
        } else {
            bail!("Error running command {} inside docker container {}", args, self.docker_image())
        }
    }
}
