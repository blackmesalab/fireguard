use std::env;
use std::path::PathBuf;
use std::process::{self, Command};
use std::time::Duration;

use color_eyre::eyre::Result;
use nix::sys::signal;
use nix::unistd::Pid;
use rand::Rng;
use tokio::fs;
use tokio::task;
use tokio::time;

use crate::github::Releases;

lazy_static! {
    pub static ref NEW_VERSION_PATH: PathBuf = env::temp_dir();
    pub static ref NEW_VERSION_FILE: PathBuf = NEW_VERSION_PATH.join("fireguard");
}

pub struct UpgradeBin {
    wait_between_checks: Duration,
    url: String,
    current_tag: String,
}

impl UpgradeBin {
    pub fn new(wait_between_checks: Duration, url: &str, current_tag: &str) -> Self {
        UpgradeBin { wait_between_checks, url: url.to_string(), current_tag: current_tag.to_string() }
    }

    fn calculate_jitter(&self) -> Duration {
        let mut rng = rand::thread_rng();
        let value = rng.gen_range(0..self.wait_between_checks.as_secs());
        Duration::from_secs(value)
    }

    pub async fn run_in_background(self, args: &[String]) -> Result<()> {
        let task_args = args.to_vec();
        task::spawn(async move {
            loop {
                let wait_duration = self.wait_between_checks + self.calculate_jitter();
                match Releases::new(&self.url).await {
                    Ok(releases) => {
                        let tag_name = releases.tag_name.clone();
                        if self.current_tag == tag_name {
                            info!(
                                "Fireguard {} is already running, sleeping for {} seconds",
                                self.current_tag,
                                wait_duration.as_secs()
                            );
                        } else {
                            info!("Fireguard needs to be updated from {} to {}", self.current_tag, tag_name);
                            match releases.download(&tag_name).await {
                                Ok(()) => match fork::daemon(true, true) {
                                    Ok(fork::Fork::Child) => {
                                        let mut cmd_args = vec!["--old-pid".to_string(), process::id().to_string()];
                                        cmd_args.extend(task_args.clone());
                                        match Command::new(NEW_VERSION_PATH.as_path()).args(cmd_args).spawn() {
                                            Ok(command) => match command.wait_with_output() {
                                                Ok(output) => {
                                                    info!(
                                                        "Forked new executable from {}: {}",
                                                        NEW_VERSION_PATH.display(),
                                                        String::from_utf8_lossy(&output.stdout)
                                                    );
                                                    break;
                                                }
                                                Err(e) => {
                                                    error!("Error extracting stdout from upgrade command: {}", e);
                                                }
                                            },
                                            Err(e) => {
                                                error!(
                                                    "Error forking new executable from {}: {}",
                                                    NEW_VERSION_PATH.display(),
                                                    e
                                                );
                                            }
                                        }
                                    }
                                    Ok(fork::Fork::Parent(pid)) => {
                                        debug!("Double fork parent process PID {} noop operation", pid)
                                    }
                                    Err(e) => {
                                        error!("Unable to daemonize new Fireguard instance: {}", e);
                                    }
                                },
                                Err(e) => {
                                    error!(
                                        "Unable to dowload Fireguard {}: {}, sleeping for {} seconds",
                                        tag_name,
                                        e,
                                        wait_duration.as_secs(),
                                    );
                                }
                            }
                        }
                    }
                    Err(e) => {
                        error!(
                            "Unable to fetch latest Github release: {}, sleeping for {} seconds",
                            e,
                            wait_duration.as_secs(),
                        );
                    }
                }
                info!("Sleeping for {} seconds", wait_duration.as_secs());
                time::sleep(wait_duration).await;
            }
        });
        Ok(())
    }

    pub async fn flip_binary_on_disk(&self, destination: PathBuf) -> Result<()> {
        info!("Copying {} to {}", NEW_VERSION_PATH.display(), destination.display());
        let bytes = fs::copy(NEW_VERSION_PATH.as_path(), &destination).await?;
        info!("Copied {} bytes executable to {}", bytes, destination.display());
        Ok(())
    }

    pub fn terminate_old_process(&self, pid: i32) -> Result<()> {
        info!("Terminating old Fireguard instance with PID {}", pid);
        Ok(signal::kill(Pid::from_raw(pid), signal::SIGINT)?)
    }
}
