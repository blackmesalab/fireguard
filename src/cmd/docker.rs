use clap::Clap;
use color_eyre::eyre::{bail, Result};

use crate::cmd::{Daemon, Dns, Fireguard, Peer, Repo, Wg};
use crate::shell::Shell;
use crate::utils::install_wireguard_kernel_module;

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
    #[clap(short = 'v', long = "docker-image-version")]
    pub docker_image_version: Option<String>,
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
    /// Daemon management
    Daemon(Daemon),
}

impl Docker {
    fn docker_image(&self) -> String {
        if let Some(version) = self.docker_image_version.as_ref() {
            format!("{}:{}", self.docker_image_name, version)
        } else {
            format!("{}:{}", self.docker_image_name, crate_version!())
        }
    }

    pub async fn exec(&self, fg: &Fireguard) -> Result<()> {
        install_wireguard_kernel_module().await?;
        let args = fg.args.join(" ");
        let mut docker_cmd = format!("run -t --rm --privileged --net=host");
        // TODO: document how to use volumes, especially if there are plans for custom paths.
        if let Some(volumes) = self.docker_volumes.as_ref() {
            docker_cmd += &format!(" -v {}", volumes.join("-v "));
        } else {
            docker_cmd += " -v /etc/fireguard:/etc/fireguard";
            docker_cmd += " -v /etc/wireguard:/etc/wireguard";
        }
        docker_cmd += &format!(" {} {}", self.docker_image(), args);
        info!("Running command `{}` inside Docker container {}", args, self.docker_image());
        let result = Shell::exec("docker", &docker_cmd, None, false).await;
        if result.success() {
            debug!("Command {} succeeded inside Docker container {}", args, self.docker_image());
            Ok(())
        } else {
            bail!("Error running command {} inside docker container {}", args, self.docker_image())
        }
    }
}
