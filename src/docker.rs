use bollard::Docker as Ballard;
use color_eyre::eyre::{eyre, Result};

pub struct Docker {
    inner: Ballard,
}

impl Docker {
    pub async fn new() -> Result<Self> {
        match Ballard::connect_with_local_defaults() {
            Ok(d) => match d.version().await {
                Ok(info) => {
                    info!(
                        "Docker {} {} {} is available",
                        info.version.unwrap_or("unknown".to_string()),
                        info.arch.unwrap_or("unknown".to_string()),
                        info.os.unwrap_or("unknown".to_string())
                    );
                    let docker = Self { inner: d };
                    Ok(docker)
                }
                Err(e) => {
                    Err(eyre!("Docker unavailable on this host: {}", e))
                }
            },
            Err(e) => {
                Err(eyre!("Docker unavailable on this host: {}", e))
            }
        }
    }
}
