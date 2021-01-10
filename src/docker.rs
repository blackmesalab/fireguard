use bollard::Docker as Ballard;

pub struct Docker {
    inner: Ballard,
}

impl Docker {
    pub async fn new() -> Option<Self> {
        match Ballard::connect_with_unix_defaults() {
            Ok(d) => match d.version().await {
                Ok(info) => {
                    info!(
                        "Docker {} {} {} is available",
                        info.version.unwrap_or("unknown".to_string()),
                        info.arch.unwrap_or("unknown".to_string()),
                        info.os.unwrap_or("unknown".to_string())
                    );
                    let docker = Self { inner: d };
                    Some(docker)
                }
                Err(_) => {
                    warn!("Docker unavailable on this host");
                    None
                }
            },
            Err(_) => {
                warn!("Docker unavailable on this host");
                None
            }
        }
    }
}
